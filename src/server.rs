use std::{
    error::Error,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

use crate::{Request, Response, Status};

const BUFFER_SIZE: usize = 1024;

pub trait Server {
    fn on_start(&mut self) {}
    #[allow(unused_variables)]
    fn on_connection(&mut self, stream: &mut TcpStream) -> bool {
        true
    }
    fn on_request(&mut self, request: Request) -> Response;
    fn on_error(&mut self, error: crate::Error);
}

fn read_request(stream: &mut TcpStream) -> Result<String, crate::Error> {
    let mut buffer = Vec::with_capacity(BUFFER_SIZE);
    unsafe { buffer.set_len(BUFFER_SIZE) };
    match stream.read(&mut buffer) {
        Ok(_) => {}
        Err(error) => return Err(crate::Error::RequestReadError(error)),
    }

    match String::from_utf8(buffer) {
        Ok(string) => Ok(string),
        Err(error) => Err(crate::Error::RequestConversionError(error)),
    }
}

fn handle_client(mut stream: TcpStream, server: &mut dyn Server) {
    // Call on connection
    if !server.on_connection(&mut stream) {
        return;
    }

    // Read request
    let request_string = match read_request(&mut stream) {
        Ok(request) => request,
        Err(error) => return server.on_error(error),
    };

    // Parse request and generate response
    let response = match Request::parse(&request_string) {
        Ok(request) => server.on_request(request),
        Err(_) => Response::new_status(Status::BadRequest),
    };

    // Write response
    match stream.write_all(response.to_string().as_bytes()) {
        Ok(()) => {}
        Err(error) => server.on_error(crate::Error::ResponseWriteError(error)),
    }
}

pub fn listen(port: u16, server: &mut dyn Server) -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind(("127.0.0.1", port))?;

    server.on_start();

    for stream in listener.incoming() {
        // Accept the incoming connection
        let stream = match stream {
            Err(error) => {
                server.on_error(crate::Error::AcceptConnectionError(error));
                continue;
            }
            Ok(stream) => stream,
        };

        // Handle connection
        handle_client(stream, server);
    }

    Ok(())
}
