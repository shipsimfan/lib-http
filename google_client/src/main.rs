fn main() {
    let request = http::Request::new("www.google.com", http::Method::GET, "/");

    match request.send() {
        Ok(response) => {
            println!("{}", response.status());
            for (key, value) in response.headers() {
                println!("{}: {}", key, value);
            }
            println!("{}", String::from_utf8_lossy(response.body()));
        }
        Err(error) => println!("Error: {}", error),
    }
}
