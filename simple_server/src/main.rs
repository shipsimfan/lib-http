struct Server;

const PORT: u16 = 3000;

fn main() {
    let mut server = Server;

    match http::server::listen(PORT, &mut server) {
        Err(error) => panic!("Error: {}", error),
        Ok(()) => {}
    }
}

impl http::server::Server for Server {
    fn on_start(&mut self) {
        println!("Server listening on localhost:{}", PORT);
    }

    fn on_request(&mut self, request: http::Request) -> http::Response {
        println!("{} request for {}", request.method(), request.path());
        println!("Headers: ");
        println!("{:?}", request.general_headers());
        println!("{:?}", request.request_headers());
        println!("{:?}", request.entity_headers());
        println!("{:?}", request.other_headers());
        println!("Body: {}", request.body());
        http::Response::new_ok(format!("Hello World!"))
    }

    fn on_error(&mut self, error: http::Error) {
        println!("\x1B[31;1mERROR\x1B[0m: {}", error);
    }
}
