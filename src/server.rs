use crate::{request, Request, Response, Status};
use std::{
    error::Error,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};

pub type ClientErrorFn = fn(error: Box<dyn std::error::Error>);

pub trait Server: Send + Sync {
    fn handle_request(&self, request: Request) -> Result<Response, Box<dyn std::error::Error>>;
}

#[derive(Debug)]
enum NetworkError {
    AcceptError(std::io::Error),
    InvalidContentLength,
}

fn read_request(stream: &mut TcpStream) -> Result<Option<Request>, Box<dyn std::error::Error>> {
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
        Some(value) => match value.parse() {
            Ok(value) => value,
            Err(_) => return Err(Box::new(NetworkError::InvalidContentLength)),
        },
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

fn handle_client<S: Server>(
    stream: Result<TcpStream, std::io::Error>,
    server: &S,
) -> Result<(), Box<dyn std::error::Error>> {
    // Accept client
    let mut stream = match stream {
        Ok(stream) => stream,
        Err(error) => return Err(Box::new(NetworkError::AcceptError(error))),
    };

    // Handle requests until the connection closes
    while handle_request(&mut stream, server)? {}

    Ok(())
}

fn handle_request<S: Server>(
    stream: &mut TcpStream,
    server: &S,
) -> Result<bool, Box<dyn std::error::Error>> {
    // Read request
    let request = match read_request(stream) {
        Ok(request) => match request {
            Some(request) => request,
            None => return Ok(false),
        },
        Err(error) => {
            // Create 400 response
            let response = Response::new_status(Status::BadRequest, Some(format!("{}", error)));

            // Send response
            stream.write_all(response.generate().as_bytes())?;

            // Return error
            return Err(error);
        }
    };

    let ret = match request.header().get_header("Connection") {
        Some(str) => str == "keep-alive",
        None => false,
    };

    // Handle request
    let (response, error) = match server.handle_request(request) {
        Ok(response) => (response, None),
        Err(error) => (
            Response::new_status(Status::InternalServerError, Some(format!("{}", error))),
            Some(error),
        ),
    };

    // Write response
    match stream.write_all(response.generate().as_bytes()) {
        Ok(()) => match stream.flush() {
            Ok(()) => match error {
                None => Ok(ret),
                Some(error) => Err(error),
            },
            Err(io_error) => match error {
                // Ignore the IO error if the response contained an error
                None => Err(Box::new(io_error) as Box<dyn Error>),
                Some(error) => Err(error),
            },
        },
        Err(io_error) => match error {
            // Ignore the IO error if the response contained an error
            None => Err(Box::new(io_error) as Box<dyn Error>),
            Some(error) => Err(error),
        },
    }
}

pub fn start_server<S: Server>(
    port: u16,
    server: &'static S,
    client_error_callback: Option<ClientErrorFn>,
) -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind(("0.0.0.0", port))?;

    for stream in listener.incoming() {
        let server = server;
        thread::spawn(move || match handle_client(stream, server) {
            Ok(()) => {}
            Err(error) => match client_error_callback {
                Some(callback) => callback(error),
                None => {}
            },
        });
    }

    Ok(())
}

impl std::error::Error for NetworkError {}

impl std::fmt::Display for NetworkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                NetworkError::AcceptError(error) => format!("Unable to accept client ({})", error),
                NetworkError::InvalidContentLength => format!("Invalid 'Content-Length'"),
            }
        )
    }
}
