use crate::headers::{entity::EntityHeader, general::GeneralHeader, request::RequestHeader};
use std::{
    borrow::Cow,
    collections::HashMap,
    io::{Read, Write},
    net::TcpStream,
};

pub struct Request {
    // Request line
    method: Method,
    path: String,
    general_headers: HashMap<GeneralHeader, String>,
    request_headers: HashMap<RequestHeader, String>,
    entity_headers: HashMap<EntityHeader, String>,
    other_headers: Vec<String>,
    body: Vec<u8>,
}

#[derive(Debug, Clone, Copy)]
pub enum Method {
    OPTIONS,
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    TRACE,
    CONNECT,
}

const BUFFER_SIZE: usize = 8192;

impl Request {
    pub fn new(host: &str, method: Method, path: &str) -> Self {
        let mut request_headers = HashMap::new();
        request_headers.insert(RequestHeader::Host, host.to_string());

        let mut general_headers = HashMap::new();
        general_headers.insert(GeneralHeader::Connection, "close".to_string());

        Request {
            method: method,
            path: path.to_string(),
            general_headers: general_headers,
            request_headers: request_headers,
            entity_headers: HashMap::new(),
            other_headers: Vec::new(),
            body: Vec::new(),
        }
    }

    pub fn send(self) -> Result<(String, Vec<u8>), crate::Error> {
        // Get the host
        let host = match &self.request_headers.get(&RequestHeader::Host) {
            Some(host) => host.as_str(),
            None => return Err(crate::Error::BadRequest),
        };

        let (addr, port) = match host.split_once(':') {
            Some((addr, port)) => (
                addr,
                match u16::from_str_radix(port, 10) {
                    Ok(value) => value,
                    Err(_) => return Err(crate::Error::BadRequest),
                },
            ),
            None => (host, 80),
        };

        // Build the request
        let mut request = format!("{} {} HTTP/1.1\r\n", self.method, self.path);

        for (header, value) in &self.general_headers {
            let header_str = format!("{}: {}\r\n", header, value);
            request.push_str(&header_str);
        }

        for (header, value) in &self.request_headers {
            let header_str = format!("{}: {}\r\n", header, value);
            request.push_str(&header_str);
        }

        for (header, value) in &self.entity_headers {
            let header_str = format!("{}: {}\r\n", header, value);
            request.push_str(&header_str);
        }

        for header in &self.other_headers {
            request.push_str(header);
            request.push_str("\r\n");
        }

        request.push_str("\r\n");

        let mut request = request.into_bytes();
        for val in self.body {
            request.push(val);
        }

        // Connect to the server
        let mut stream = match TcpStream::connect((addr, port)) {
            Ok(stream) => stream,
            Err(error) => return Err(crate::Error::ConnectionError(error)),
        };

        // Send the request
        match stream.write_all(&request) {
            Ok(()) => {}
            Err(error) => return Err(crate::Error::RequestWriteError(error)),
        }

        // Read the response
        let mut response_buffer = Vec::with_capacity(BUFFER_SIZE);
        unsafe { response_buffer.set_len(BUFFER_SIZE) };
        match stream.read(&mut response_buffer) {
            Ok(_) => {
                let mut i = 0;
                let mut iter = response_buffer.iter();
                while let Some(c) = iter.next() {
                    if *c == '\r' as u8 {
                        i += 1;
                        match iter.next() {
                            None => {}
                            Some(c) => {
                                if *c == '\n' as u8 {
                                    i += 1;
                                    match iter.next() {
                                        None => {}
                                        Some(c) => {
                                            if *c == '\r' as u8 {
                                                i += 1;
                                                match iter.next() {
                                                    None => {}
                                                    Some(c) => {
                                                        if *c == '\n' as u8 {
                                                            i += 1;
                                                            let header =
                                                                match String::from_utf8_lossy(
                                                                    &response_buffer[..i],
                                                                ) {
                                                                    Cow::Borrowed(str) => {
                                                                        str.to_string()
                                                                    }
                                                                    Cow::Owned(_) => return Err(
                                                                        crate::Error::BadResponse,
                                                                    ),
                                                                };
                                                            let body = if i == response_buffer.len()
                                                            {
                                                                Vec::new()
                                                            } else {
                                                                Vec::from(&response_buffer[i..])
                                                            };

                                                            return Ok((header, body));
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    i += 1;
                }

                Err(crate::Error::BadResponse)
            }
            Err(error) => Err(crate::Error::ResponseReadError(error)),
        }
    }

    pub fn parse(string: &str) -> Result<Self, crate::Error> {
        // Split into lines
        let mut iter = string.split("\r\n");

        // Parse request line
        let (method, path) = Request::parse_request_line(match iter.next() {
            Some(line) => line,
            None => return Err(crate::Error::BadRequest),
        })?;

        // Parse headers
        let mut general_headers = HashMap::new();
        let mut request_headers = HashMap::new();
        let mut entity_headers = HashMap::new();
        let mut other_headers = Vec::new();
        loop {
            let line = match iter.next() {
                Some(line) => line,
                None => return Err(crate::Error::BadRequest),
            };

            if line == "" {
                break;
            }

            match GeneralHeader::parse(line) {
                Ok((key, value)) => {
                    general_headers.insert(key, value);
                }
                Err(error) => match error {
                    crate::Error::InvalidHeader => match RequestHeader::parse(line) {
                        Ok((key, value)) => {
                            request_headers.insert(key, value);
                        }
                        Err(error) => match error {
                            crate::Error::InvalidHeader => match EntityHeader::parse(line) {
                                Ok((key, value)) => {
                                    entity_headers.insert(key, value);
                                }
                                Err(error) => match error {
                                    crate::Error::InvalidHeader => {
                                        other_headers.push(line.to_string());
                                    }
                                    _ => return Err(error),
                                },
                            },
                            _ => return Err(error),
                        },
                    },
                    _ => return Err(error),
                },
            }
        }

        // Parse body
        let mut body = String::new();
        while let Some(line) = iter.next() {
            body.push_str(line);
            body.push_str("\r\n");
        }

        let body = body.into_bytes();

        Ok(Request {
            method: method,
            path: path,
            general_headers: general_headers,
            request_headers: request_headers,
            entity_headers: entity_headers,
            other_headers: other_headers,
            body: body,
        })
    }

    fn parse_request_line(request_line: &str) -> Result<(Method, String), crate::Error> {
        let mut request_line = request_line.split(" ");

        // Parse method
        let method = match request_line.next() {
            Some(line) => Method::parse(line)?,
            None => return Err(crate::Error::BadRequest),
        };

        // Parse path
        let path = match request_line.next() {
            Some(line) => line.to_string(),
            None => return Err(crate::Error::BadRequest),
        };

        // Verify HTTP version
        match request_line.next() {
            Some(line) => {
                if line != "HTTP/1.1" {
                    return Err(crate::Error::BadRequest);
                }
            }
            None => return Err(crate::Error::BadRequest),
        }

        // Verify end of line
        match request_line.next() {
            Some(_) => Err(crate::Error::BadRequest),
            None => Ok((method, path)),
        }
    }

    pub fn method(&self) -> Method {
        self.method
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn general_headers(&self) -> &HashMap<GeneralHeader, String> {
        &self.general_headers
    }

    pub fn request_headers(&self) -> &HashMap<RequestHeader, String> {
        &self.request_headers
    }

    pub fn entity_headers(&self) -> &HashMap<EntityHeader, String> {
        &self.entity_headers
    }

    pub fn other_headers(&self) -> &Vec<String> {
        &self.other_headers
    }

    pub fn body(&self) -> &Vec<u8> {
        &self.body
    }
}

impl Method {
    pub fn parse(string: &str) -> Result<Self, crate::Error> {
        match string {
            "OPTIONS" => Ok(Method::OPTIONS),
            "GET" => Ok(Method::GET),
            "HEAD" => Ok(Method::HEAD),
            "POST" => Ok(Method::POST),
            "PUT" => Ok(Method::PUT),
            "DELETE" => Ok(Method::DELETE),
            "TRACE" => Ok(Method::TRACE),
            "CONNECT" => Ok(Method::CONNECT),
            _ => Err(crate::Error::BadRequest),
        }
    }
}

impl std::fmt::Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Method::OPTIONS => "OPTIONS",
                Method::GET => "GET",
                Method::HEAD => "HEAD",
                Method::POST => "POST",
                Method::PUT => "PUT",
                Method::DELETE => "DELETE",
                Method::TRACE => "TRACE",
                Method::CONNECT => "CONNECT",
            }
        )
    }
}
