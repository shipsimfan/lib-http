struct Server;

fn main() {
    let mut server = Server;

    match http::server::listen(3000, &mut server) {
        Err(error) => panic!("Error: {}", error),
        Ok(()) => {}
    }
}

impl http::server::Server for Server {
    fn request(&mut self, request: String) -> String {
        println!("Recieved request:\n{}", request);

        "HTTP/1.1 200 OK\r\n\r\nHello World!".to_string()
    }

    fn start(&mut self) {
        println!("Listening . . . ");
    }
}
