pub struct Response {
    status: Status,
    body: String,
}

#[repr(usize)]
#[derive(Debug, Clone, Copy)]
pub enum Status {
    Continue = 100,
    SwitchingProtocols = 101,
    OK = 200,
    Created = 201,
    Accepted = 202,
    NonAuthoritativeInformation = 203,
    NoContent = 204,
    ResetContent = 205,
    PartialContent = 206,
    MultipleChoices = 300,
    MovedPermanently = 301,
    Found = 302,
    SeeOther = 303,
    NotModified = 304,
    UseProxy = 305,
    TemporaryRedirect = 307,
    BadRequest = 400,
    Unauthorized = 401,
    PaymentRequired = 402,
    Forbidden = 403,
    NotFound = 404,
    MethodNotAllowed = 405,
    NotAcceptable = 406,
    ProxyAuthenticationRequired = 407,
    RequestTimeout = 408,
    Conflict = 409,
    Gone = 410,
    LengthRequired = 411,
    PreconditionFailed = 412,
    RequestEntityTooLarge = 413,
    RequestURITooLarge = 414,
    UnsupportedMediaType = 415,
    RequestedRangeNotSatisfiable = 416,
    ExpectationFailed = 417,
    InternalServerError = 500,
    NotImplemented = 501,
    BadGateway = 502,
    ServiceUnavailable = 503,
    GatewayTimeout = 504,
    HTTPVersionNotSupported = 505,
}

impl Response {
    pub fn new(status: Status, body: String) -> Self {
        Response {
            status: status,
            body: body,
        }
    }

    pub fn new_ok(body: String) -> Self {
        Response::new(Status::OK, body)
    }

    pub fn new_status(status: Status) -> Self {
        Response::new(status, String::new())
    }

    pub fn to_string(self) -> String {
        format!(
            "HTTP/1.1 {} {}\r\n\r\n{}",
            self.status.code(),
            self.status,
            self.body
        )
    }
}

impl Status {
    pub fn code(&self) -> usize {
        *self as usize
    }
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Status::Continue => "Continue",
                Status::SwitchingProtocols => "Switching Protocols",
                Status::OK => "OK",
                Status::Created => "Created",
                Status::Accepted => "Accepted",
                Status::NonAuthoritativeInformation => "Non-Authoritative Information",
                Status::NoContent => "No Content",
                Status::ResetContent => "Reset Content",
                Status::PartialContent => "Partial Content",
                Status::MultipleChoices => "Multiple Choices",
                Status::MovedPermanently => "Moved Permanently",
                Status::Found => "Found",
                Status::SeeOther => "See Other",
                Status::NotModified => "Not Modified",
                Status::UseProxy => "Use Proxy",
                Status::TemporaryRedirect => "Temporary Redirect",
                Status::BadRequest => "Bad Request",
                Status::Unauthorized => "Unauthorized",
                Status::PaymentRequired => "Payment Required",
                Status::Forbidden => "Forbidden",
                Status::NotFound => "Not Found",
                Status::MethodNotAllowed => "Method Not Allowed",
                Status::NotAcceptable => "Not Acceptable",
                Status::ProxyAuthenticationRequired => "Proxy Authentication Required",
                Status::RequestTimeout => "Request Time-out",
                Status::Conflict => "Conflict",
                Status::Gone => "Gone",
                Status::LengthRequired => "Length Required",
                Status::PreconditionFailed => "Precondition Failed",
                Status::RequestEntityTooLarge => "Request Entity Too Large",
                Status::RequestURITooLarge => "Request-URI Too Large",
                Status::UnsupportedMediaType => "Unsupported Media Type",
                Status::RequestedRangeNotSatisfiable => "Requested Range Not Satisfiable",
                Status::ExpectationFailed => "Expectation Failed",
                Status::InternalServerError => "Internal Server Error",
                Status::NotImplemented => "Not Implemented",
                Status::BadGateway => "Bad Gateway",
                Status::ServiceUnavailable => "Service Unavailable",
                Status::GatewayTimeout => "Gateway Time-out",
                Status::HTTPVersionNotSupported => "HTTP Version Not Supported",
            }
        )
    }
}
