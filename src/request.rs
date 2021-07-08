use std::{
    collections::HashMap,
    io::{Read, Write},
    net::TcpStream,
};

use crate::Response;

pub struct Request {
    // Request line
    method: Method,
    path: String,
    headers: HashMap<String, String>,
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

const DEFAULT_HTTP_PORT: u16 = 80;

impl Request {
    pub fn new(host: &str, method: Method, path: &str) -> Self {
        let mut headers = HashMap::new();
        headers.insert("Host".to_string(), host.to_string());
        headers.insert("Connection".to_string(), "close".to_string());

        Request {
            method: method,
            path: path.to_string(),
            headers: headers,
            body: Vec::new(),
        }
    }

    pub fn parse(buffer: Vec<u8>) -> Result<Self, crate::Error> {
        let (header, body) = match super::split_http_message(buffer) {
            Ok(value) => value,
            Err(()) => return Err(crate::Error::BadRequest),
        };

        // Parse header
        let (method, path, headers) = Request::parse_header(header)?;

        // Create request
        Ok(Request {
            method: method,
            path: path,
            headers: headers,
            body: body,
        })
    }

    pub fn compile(self) -> Result<(String, u16, Vec<u8>), crate::Error> {
        // Get the host
        let host = match &self.headers.get("Host") {
            Some(host) => host.as_str(),
            None => return Err(crate::Error::BadRequest),
        };

        // Get port from host, or use default if not specified
        let (addr, port) = match host.split_once(':') {
            Some((addr, port)) => (
                addr,
                match u16::from_str_radix(port, 10) {
                    Ok(value) => value,
                    Err(_) => return Err(crate::Error::BadRequest),
                },
            ),
            None => (host, DEFAULT_HTTP_PORT),
        };

        // Build the request header
        let mut request_header = String::new();

        // Insert the request line
        let request_line = format!("{} {} HTTP/1.1\r\n", self.method, self.path);
        request_header.push_str(&request_line);

        // Insert the headers
        for (key, value) in &self.headers {
            let header = format!("{}: {}\r\n", key, value);
            request_header.push_str(&header);
        }

        // Insert the end of header
        request_header.push_str("\r\n");

        // Combine the header and body into the request
        let mut request = request_header.into_bytes();
        request.extend_from_slice(self.body.as_slice());

        Ok((addr.to_string(), port, request))
    }

    pub fn send(self) -> Result<Response, crate::Error> {
        // Compile the request
        let (addr, port, request) = self.compile()?;

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
        let mut response_buffer = Vec::new();
        match stream.read_to_end(&mut response_buffer) {
            Ok(_) => Response::parse(response_buffer),
            Err(error) => Err(crate::Error::ResponseReadError(error)),
        }
    }

    fn parse_header(
        header: String,
    ) -> Result<(Method, String, HashMap<String, String>), crate::Error> {
        let mut iter = header.split("\r\n");

        // Parse request line
        let (method, path) = match iter.next() {
            Some(request_line) => Request::parse_request_line(request_line)?,
            None => return Err(crate::Error::BadRequest),
        };

        // Parse headers
        let mut headers = HashMap::new();
        while let Some(header) = iter.next() {
            match header.split_once(':') {
                Some((key, value)) => {
                    headers.insert(key.trim().to_string(), value.trim().to_string());
                }
                None => return Err(crate::Error::BadRequest),
            }
        }

        Ok((method, path, headers))
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

    pub fn headers(&self) -> &HashMap<String, String> {
        &self.headers
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
