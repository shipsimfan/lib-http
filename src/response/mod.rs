use self::header::Header;

mod header;
mod status;

pub use status::Status;

pub struct Response {
    header: Header,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: usize, reason_phrase: String, body: Option<String>) -> Self {
        Response {
            header: Header::new(status_code, reason_phrase),
            body,
        }
    }

    pub fn new_status(status: Status, body: Option<String>) -> Self {
        Response {
            header: Header::new_status(status),
            body,
        }
    }

    pub fn header(&self) -> &Header {
        &self.header
    }

    pub fn header_mut(&mut self) -> &mut Header {
        &mut self.header
    }

    pub fn generate(mut self) -> String {
        // Set Content-Length if nescessary
        match &self.body {
            Some(body) => self
                .header
                .insert_header(format!("Content-Length"), format!("{}", body.len())),
            None => {}
        }

        // Generate header
        let mut response = self.header.generate();

        // Append body
        match self.body {
            Some(body) => response.push_str(&body),
            None => {}
        }

        response
    }
}
