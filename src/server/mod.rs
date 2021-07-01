use std::{
    error::Error,
    io::{Read, Write},
    net::TcpListener,
};

pub trait Server {
    fn start(&mut self);
    fn request(&mut self, request: String) -> String;
}

pub fn listen(port: u16, server: &mut dyn Server) -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind(("127.0.0.1", port))?;

    server.start();

    for stream in listener.incoming() {
        println!("Client connected");

        let mut stream = match stream {
            Err(error) => {
                println!("Error while accepting client: {}", error);
                continue;
            }
            Ok(stream) => stream,
        };

        let mut buffer = Vec::with_capacity(1024);
        unsafe { buffer.set_len(1024) };
        match stream.read(&mut buffer) {
            Ok(_) => {}
            Err(error) => {
                println!("Error while reading request: {}", error);
                continue;
            }
        }

        let request = match String::from_utf8(buffer) {
            Ok(request) => request,
            Err(error) => {
                println!("Error while translating request: {}", error);
                continue;
            }
        };

        let response = server.request(request);

        match stream.write_all(response.as_bytes()) {
            Ok(_) => {}
            Err(error) => {
                println!("Error while sending response: {}", error);
                continue;
            }
        }
    }

    Ok(())
}
