use crate::{Request, Response, Status};
use std::{
    error::Error,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

const BUFFER_SIZE: usize = 8192;

pub trait Server {
    fn on_start(&mut self) {}
    #[allow(unused_variables)]
    fn on_connection(&mut self, stream: &mut TcpStream) -> bool {
        true
    }
    fn on_request(&mut self, request: Request) -> Response;
    fn on_error(&mut self, error: crate::Error);
}

fn read_request(stream: &mut TcpStream) -> Result<Vec<u8>, crate::Error> {
    let mut buffer = [0u8; BUFFER_SIZE];
    match stream.read(&mut buffer) {
        Ok(bytes_read) => {
            let mut response = Vec::with_capacity(bytes_read);
            for i in 0..bytes_read {
                response.push(buffer[i]);
            }
            Ok(response)
        }
        Err(error) => Err(crate::Error::RequestReadError(error)),
    }
}

fn handle_client(mut stream: TcpStream, server: &mut dyn Server) {
    // Call on connection
    if !server.on_connection(&mut stream) {
        return;
    }

    // Read request
    let request = match read_request(&mut stream) {
        Ok(request) => request,
        Err(error) => return server.on_error(error),
    };

    // Parse request and generate response
    let response = match Request::parse(request) {
        Ok(request) => server.on_request(request),
        Err(_) => Response::new_status(Status::BadRequest),
    };

    // Write response
    match stream.write_all(response.compile().as_slice()) {
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
