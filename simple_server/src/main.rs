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

    fn on_request(&mut self, _: String) -> String {
        "Hello World!".to_string()
    }
}
