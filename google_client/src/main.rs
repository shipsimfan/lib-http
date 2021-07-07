fn main() {
    let request = http::Request::new("www.google.ca", http::Method::GET, "/");

    match request.send() {
        Ok((header, body)) => {
            println!("Response:");
            println!("{}", header);
            println!("{:#X?}", body);
        }
        Err(error) => println!("Error: {}", error),
    }
}
