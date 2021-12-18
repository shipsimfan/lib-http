use super::method::{InvalidMethodError, Method};
use std::collections::HashMap;

pub struct Header {
    method: Method,
    uri: String,
    headers: HashMap<String, String>,
}

#[derive(Debug)]
pub enum RequestParseError {
    NoRequestLine,
    InvalidEnding,
    InvalidHeaderLine(String),
    InvalidMethod(InvalidMethodError),
    NoURI,
    InvalidHTTPVersion,
    NoVersion,
    RequestLineTooLong,
}

impl Header {
    pub fn parse<S: AsRef<str>>(str: S) -> Result<Self, RequestParseError> {
        let mut lines = str.as_ref().split("\r\n");

        // Parse request line
        let (method, uri) = Header::parse_request_line(match lines.next() {
            Some(str) => str.trim(),
            None => return Err(RequestParseError::NoRequestLine),
        })?;

        // Parse headers
        let mut headers = HashMap::new();
        loop {
            let line = match lines.next() {
                Some(str) => str.trim(),
                None => return Err(RequestParseError::InvalidEnding),
            };

            if line == "" {
                break;
            }

            let mut parts = line.split(':');
            let key = match parts.next() {
                Some(str) => str.trim(),
                None => return Err(RequestParseError::InvalidHeaderLine(line.to_owned())),
            };

            let mut value = match parts.next() {
                Some(str) => str.to_owned(),
                None => String::new(),
            };
            for part in parts {
                value.push(':');
                value.push_str(part);
            }

            let value = value.trim().to_owned();

            headers.insert(key.to_owned(), value);
        }

        Ok(Header {
            method,
            uri,
            headers,
        })
    }

    fn parse_request_line<S: AsRef<str>>(str: S) -> Result<(Method, String), RequestParseError> {
        let mut parts = str.as_ref().split(' ');

        // Parse method
        let method = match Method::parse(match parts.next() {
            Some(str) => str.trim(),
            None => return Err(RequestParseError::NoRequestLine),
        }) {
            Ok(method) => method,
            Err(error) => return Err(RequestParseError::InvalidMethod(error)),
        };

        // Parse URI
        let uri = match parts.next() {
            Some(str) => str.trim(),
            None => return Err(RequestParseError::NoURI),
        };

        // Parse version
        match parts.next() {
            Some(str) => {
                if str.trim() != "HTTP/1.1" {
                    return Err(RequestParseError::InvalidHTTPVersion);
                }
            }
            None => return Err(RequestParseError::NoVersion),
        }

        // Verify end of line
        match parts.next() {
            Some(_) => Err(RequestParseError::RequestLineTooLong),
            None => Ok((method, uri.to_owned())),
        }
    }

    pub fn get_header<S: AsRef<str>>(&self, key: S) -> Option<&str> {
        self.headers.get(key.as_ref()).map(|s| s.as_str())
    }

    pub fn method(&self) -> Method {
        self.method
    }

    pub fn uri(&self) -> &str {
        &self.uri
    }
}

impl std::error::Error for RequestParseError {}

impl std::fmt::Display for RequestParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                RequestParseError::NoRequestLine => format!("No request line"),
                RequestParseError::InvalidEnding => format!("Invalid request header ending"),
                RequestParseError::InvalidHeaderLine(line) =>
                    format!("Invalid header line ({})", line),
                RequestParseError::InvalidMethod(error) => format!("{}", error),
                RequestParseError::NoURI => format!("No URI"),
                RequestParseError::InvalidHTTPVersion => format!("Invalid HTTP version"),
                RequestParseError::NoVersion => format!("No version"),
                RequestParseError::RequestLineTooLong => format!("Request line too long"),
            }
        )
    }
}
