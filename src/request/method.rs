#[derive(Clone, Copy)]
pub enum Method {
    Options,
    Get,
    Head,
    Post,
    Put,
    Delete,
    Trace,
    Connect,
}

#[derive(Debug)]
pub struct InvalidMethodError(String);

impl Method {
    pub fn parse<S: AsRef<str>>(str: S) -> Result<Self, InvalidMethodError> {
        Ok(match str.as_ref() {
            "OPTIONS" => Method::Options,
            "GET" => Method::Get,
            "HEAD" => Method::Head,
            "POST" => Method::Post,
            "PUT" => Method::Put,
            "DELETE" => Method::Delete,
            "TRACE" => Method::Trace,
            "CONNECT" => Method::Connect,
            _ => return Err(InvalidMethodError(str.as_ref().to_owned())),
        })
    }
}

impl std::fmt::Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Method::Options => "OPTIONS",
                Method::Get => "GET",
                Method::Head => "HEAD",
                Method::Post => "POST",
                Method::Put => "PUT",
                Method::Delete => "DELETE",
                Method::Trace => "TRACE",
                Method::Connect => "CONNECT",
            }
        )
    }
}

impl std::error::Error for InvalidMethodError {}

impl std::fmt::Display for InvalidMethodError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid method ({})", self.0)
    }
}
