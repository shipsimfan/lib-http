mod header;
mod method;

pub use header::Header;
pub use method::Method;

pub struct Request {
    header: Header,
    body: String,
}

impl Request {
    pub fn new(header: Header, body: String) -> Self {
        Request { header, body }
    }

    pub fn header(&self) -> &Header {
        &self.header
    }

    pub fn body(&self) -> &str {
        &self.body
    }
}
