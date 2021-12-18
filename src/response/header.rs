use std::collections::HashMap;

use super::status::Status;

pub struct Header {
    status_code: usize,
    reason_phrase: String,
    headers: HashMap<String, String>,
}

impl Header {
    pub fn new(status_code: usize, reason_phrase: String) -> Self {
        Header {
            status_code,
            reason_phrase,
            headers: HashMap::new(),
        }
    }

    pub fn new_status(status: Status) -> Self {
        Header {
            status_code: status.code(),
            reason_phrase: status.reason_phrase().to_owned(),
            headers: HashMap::new(),
        }
    }

    pub fn insert_header(&mut self, key: String, value: String) {
        self.headers.insert(key, value);
    }

    pub fn get_header<S: AsRef<str>>(&self, key: S) -> Option<&str> {
        self.headers.get(key.as_ref()).map(|s| s.as_str())
    }

    pub fn generate(self) -> String {
        let mut header = format!("HTTP/1.1 {} {}\r\n", self.status_code, self.reason_phrase);

        for (key, value) in self.headers {
            header.push_str(&format!("{}: {}\r\n", key, value));
        }

        header.push_str("\r\n");
        header
    }
}
