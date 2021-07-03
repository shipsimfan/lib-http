pub struct Request {
    // Request line
    method: Method,
    path: String,
}

#[derive(Debug, Clone, Copy)]
pub enum Method {
    OPTIONS,
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    TRACE,
    CONNECT,
}

impl Request {
    pub fn parse(string: &str) -> Result<Self, crate::Error> {
        // Split into lines
        let mut iter = string.split("\r\n");

        // Parse request line
        let (method, path) = Request::parse_request_line(match iter.next() {
            Some(line) => line,
            None => return Err(crate::Error::BadRequest),
        })?;

        Ok(Request {
            method: method,
            path: path,
        })
    }

    fn parse_request_line(request_line: &str) -> Result<(Method, String), crate::Error> {
        let mut request_line = request_line.split(" ");

        // Parse method
        let method = match request_line.next() {
            Some(line) => Method::parse(line)?,
            None => return Err(crate::Error::BadRequest),
        };

        // Parse path
        let path = match request_line.next() {
            Some(line) => line.to_string(),
            None => return Err(crate::Error::BadRequest),
        };

        // Verify HTTP version
        match request_line.next() {
            Some(line) => {
                if line != "HTTP/1.1" {
                    return Err(crate::Error::BadRequest);
                }
            }
            None => return Err(crate::Error::BadRequest),
        }

        // Verify end of line
        match request_line.next() {
            Some(_) => Err(crate::Error::BadRequest),
            None => Ok((method, path)),
        }
    }

    pub fn method(&self) -> Method {
        self.method
    }

    pub fn path(&self) -> &str {
        &self.path
    }
}

impl Method {
    pub fn parse(string: &str) -> Result<Self, crate::Error> {
        match string {
            "OPTIONS" => Ok(Method::OPTIONS),
            "GET" => Ok(Method::GET),
            "HEAD" => Ok(Method::HEAD),
            "POST" => Ok(Method::POST),
            "PUT" => Ok(Method::PUT),
            "DELETE" => Ok(Method::DELETE),
            "TRACE" => Ok(Method::TRACE),
            "CONNECT" => Ok(Method::CONNECT),
            _ => Err(crate::Error::BadRequest),
        }
    }
}

impl std::fmt::Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Method::OPTIONS => "OPTIONS",
                Method::GET => "GET",
                Method::HEAD => "HEAD",
                Method::POST => "POST",
                Method::PUT => "PUT",
                Method::DELETE => "DELETE",
                Method::TRACE => "TRACE",
                Method::CONNECT => "CONNECT",
            }
        )
    }
}
