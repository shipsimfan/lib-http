use crate::{Request, Response, Status};
use std::{
    io::Write,
    net::{TcpListener, TcpStream},
    thread,
};

mod read;

pub use read::ReadError;

pub type ClientErrorFn = fn(error: HandleClientError);

pub trait Server: Send + Sync {
    fn handle_request(&self, request: Request) -> Response;
}

#[derive(Debug)]
pub enum HandleClientError {
    AcceptClientError(std::io::Error),
    ReadRequestError(ReadError),
    WriteResponseError(std::io::Error),
}

fn handle_client<S: Server>(
    stream: Result<TcpStream, std::io::Error>,
    server: &S,
) -> Result<(), HandleClientError> {
    // Accept client
    let mut stream = match stream {
        Ok(stream) => stream,
        Err(error) => return Err(HandleClientError::AcceptClientError(error)),
    };

    // Handle requests until the connection closes
    while handle_request(&mut stream, server)? {}

    Ok(())
}

fn handle_request<S: Server>(
    stream: &mut TcpStream,
    server: &S,
) -> Result<bool, HandleClientError> {
    // Read request
    let request = match read::read_request(stream) {
        Ok(request) => match request {
            Some(request) => request,
            None => return Ok(false),
        },
        Err(error) => {
            // Create 400 response
            let response = Response::new_status(Status::BadRequest, Some(format!("{}", error)));

            // Send response
            stream.write_all(response.generate().as_bytes()).ok();

            return Err(HandleClientError::ReadRequestError(error));
        }
    };

    let ret = match request.header().get_header("Connection") {
        Some(str) => str == "keep-alive",
        None => false,
    };

    // Handle request
    let response = server.handle_request(request);

    // Write response
    match stream.write_all(response.generate().as_bytes()) {
        Ok(()) => match stream.flush() {
            Ok(()) => Ok(ret),
            Err(error) => Err(HandleClientError::WriteResponseError(error)),
        },
        Err(error) => Err(HandleClientError::WriteResponseError(error)),
    }
}

pub fn start_server<S: Server>(
    port: u16,
    server: &'static S,
    client_error_callback: Option<ClientErrorFn>,
) -> Result<(), std::io::Error> {
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

impl std::error::Error for HandleClientError {}

impl std::fmt::Display for HandleClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                HandleClientError::AcceptClientError(error) =>
                    format!("Unable to accept client ({})", error),
                HandleClientError::ReadRequestError(error) =>
                    format!("Failed to read request - {}", error),
                HandleClientError::WriteResponseError(error) =>
                    format!("Unable to write response ({})", error),
            }
        )
    }
}
