use std::{
    error::Error,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

pub trait Server {
    fn on_start(&mut self) {}
    fn on_connection(&mut self, stream: &mut TcpStream) -> bool {
        true
    }
    fn on_request(&mut self, request: String) -> String;
}

fn display_error(message: &str, error: Box<dyn Error>) {
    println!("\x1B[31;1mERROR\x1B[0m: {} - {}", message, error);
}

fn read_request(stream: &mut TcpStream) -> Result<String, ()> {
    let mut buffer = Vec::with_capacity(1024);
    unsafe { buffer.set_len(1024) };
    match stream.read(&mut buffer) {
        Ok(_) => {}
        Err(error) => return Err(display_error("Unable to read request", Box::new(error))),
    }

    match String::from_utf8(buffer) {
        Ok(request) => Ok(request),
        Err(error) => Err(display_error("Unable to convert request", Box::new(error))),
    }
}

fn write_response(stream: &mut TcpStream, response: String) {
    match stream.write_all(response.as_bytes()) {
        Ok(_) => {}
        Err(error) => return println!("Error while sending response: {}", error),
    }
}

fn handle_client(mut stream: TcpStream, server: &mut dyn Server) {
    if !server.on_connection(&mut stream) {
        return;
    }

    let request = match read_request(&mut stream) {
        Ok(request) => request,
        Err(()) => return,
    };

    let response = server.on_request(request);

    write_response(&mut stream, response);
}

pub fn listen(port: u16, server: &mut dyn Server) -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind(("127.0.0.1", port))?;

    server.on_start();

    for stream in listener.incoming() {
        // Accept the incoming connection
        let stream = match stream {
            Err(error) => {
                display_error("Unable to accept client", Box::new(error));
                continue;
            }
            Ok(stream) => stream,
        };

        // Handle connection
        handle_client(stream, server);
    }

    Ok(())
}
