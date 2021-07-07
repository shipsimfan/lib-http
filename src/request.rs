use crate::headers::{entity::EntityHeader, general::GeneralHeader, request::RequestHeader};
use std::collections::HashSet;

pub struct Request {
    // Request line
    method: Method,
    path: String,
    general_headers: HashSet<GeneralHeader>,
    request_headers: HashSet<RequestHeader>,
    entity_headers: HashSet<EntityHeader>,
    other_headers: Vec<String>,
    body: String,
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

impl Request {
    pub fn parse(string: &str) -> Result<Self, crate::Error> {
        // Split into lines
        let mut iter = string.split("\r\n");

        // Parse request line
        let (method, path) = Request::parse_request_line(match iter.next() {
            Some(line) => line,
            None => return Err(crate::Error::BadRequest),
        })?;

        // Parse headers
        let mut general_headers = HashSet::new();
        let mut request_headers = HashSet::new();
        let mut entity_headers = HashSet::new();
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
                Ok(header) => {
                    general_headers.insert(header);
                }
                Err(error) => match error {
                    crate::Error::InvalidHeader => match RequestHeader::parse(line) {
                        Ok(header) => {
                            request_headers.insert(header);
                        }
                        Err(error) => match error {
                            crate::Error::InvalidHeader => match EntityHeader::parse(line) {
                                Ok(header) => {
                                    entity_headers.insert(header);
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

    pub fn general_headers(&self) -> &HashSet<GeneralHeader> {
        &self.general_headers
    }

    pub fn request_headers(&self) -> &HashSet<RequestHeader> {
        &self.request_headers
    }

    pub fn entity_headers(&self) -> &HashSet<EntityHeader> {
        &self.entity_headers
    }

    pub fn other_headers(&self) -> &Vec<String> {
        &self.other_headers
    }

    pub fn body(&self) -> &str {
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
