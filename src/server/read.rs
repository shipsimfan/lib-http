use crate::{request, Request, RequestParseError};
use std::{io::Read, net::TcpStream, num::ParseIntError};

#[derive(Debug)]
pub enum ReadError {
    ReadError(std::io::Error),
    InvalidUTF8(std::string::FromUtf8Error),
    RequestParseError(RequestParseError),
    InvalidContentLength(ParseIntError),
}

pub fn read_request(stream: &mut TcpStream) -> Result<Option<Request>, ReadError> {
    // Read until "\r\n\r\n"
    let mut buffer = Vec::with_capacity(128);

    let mut char_buffer = [0];
    loop {
        let bytes_read = stream.read(&mut char_buffer)?;
        if bytes_read == 0 {
            return Ok(None);
        }

        buffer.push(char_buffer[0]);

        if buffer.len() >= 4 {
            let buffer_len = buffer.len();
            if *buffer.get(buffer_len - 1).unwrap() == b'\n'
                && *buffer.get(buffer_len - 2).unwrap() == b'\r'
                && *buffer.get(buffer_len - 3).unwrap() == b'\n'
                && *buffer.get(buffer_len - 4).unwrap() == b'\r'
            {
                break;
            }
        }
    }

    let header_str = String::from_utf8(buffer)?;

    // Parse header
    let header = request::Header::parse(header_str)?;

    // Check if there is a body
    let body_length = match header.get_header("Content-Length") {
        None => return Ok(Some(Request::new(header, String::new()))),
        Some(value) => value.parse()?,
    };

    if body_length == 0 {
        return Ok(Some(Request::new(header, String::new())));
    }

    // Read body
    let mut buffer = Vec::with_capacity(body_length);
    loop {
        let bytes_read = stream.read(&mut char_buffer)?;
        if bytes_read == 0 {
            break;
        }

        buffer.push(char_buffer[0]);

        if buffer.len() == body_length {
            break;
        }
    }

    Ok(Some(Request::new(header, String::from_utf8(buffer)?)))
}

impl std::error::Error for ReadError {}

impl std::fmt::Display for ReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ReadError::ReadError(error) => format!("Unable to read request ({})", error),
                ReadError::InvalidUTF8(error) => format!("Invalid UTF-8 ({})", error),
                ReadError::RequestParseError(error) =>
                    format!("Unable to parse request ({})", error),
                ReadError::InvalidContentLength(error) =>
                    format!("Invalid Content-Length value ({})", error),
            }
        )
    }
}

impl From<std::io::Error> for ReadError {
    fn from(error: std::io::Error) -> Self {
        ReadError::ReadError(error)
    }
}

impl From<std::string::FromUtf8Error> for ReadError {
    fn from(error: std::string::FromUtf8Error) -> Self {
        ReadError::InvalidUTF8(error)
    }
}

impl From<RequestParseError> for ReadError {
    fn from(error: RequestParseError) -> Self {
        ReadError::RequestParseError(error)
    }
}

impl From<ParseIntError> for ReadError {
    fn from(error: ParseIntError) -> Self {
        ReadError::InvalidContentLength(error)
    }
}
