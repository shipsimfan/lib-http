use std::collections::HashMap;

pub struct Response {
    status: Status,
    headers: HashMap<String, String>,
    body: Vec<u8>,
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
    pub fn new(status: Status, body: Vec<u8>) -> Self {
        Response {
            status: status,
            headers: HashMap::new(),
            body: body,
        }
    }

    pub fn new_ok(body: Vec<u8>) -> Self {
        Response::new(Status::OK, body)
    }

    pub fn new_status(status: Status) -> Self {
        Response::new(status, Vec::new())
    }

    pub fn parse(buffer: Vec<u8>) -> Result<Self, crate::Error> {
        let (header, body) = match super::split_http_message(buffer) {
            Ok(value) => value,
            Err(()) => return Err(crate::Error::BadResponse),
        };

        // Parse header
        let (status, headers) = Response::parse_header(header)?;

        // Create request
        Ok(Response {
            status: status,
            headers: headers,
            body: body,
        })
    }

    pub fn compile(self) -> Vec<u8> {
        let header = format!("HTTP/1.1 {} {}\r\n\r\n", self.status.code(), self.status,);

        let mut ret = Vec::from(header);
        ret.extend_from_slice(self.body.as_slice());
        ret
    }

    fn parse_header(header: String) -> Result<(Status, HashMap<String, String>), crate::Error> {
        let mut iter = header.split("\r\n");

        // Parse request line
        let status = match iter.next() {
            Some(response_line) => Response::parse_response_line(response_line)?,
            None => return Err(crate::Error::BadResponse),
        };

        // Parse headers
        let mut headers = HashMap::new();
        while let Some(header) = iter.next() {
            match header.split_once(':') {
                Some((key, value)) => {
                    headers.insert(key.trim().to_string(), value.trim().to_string());
                }
                None => return Err(crate::Error::BadResponse),
            }
        }

        Ok((status, headers))
    }

    fn parse_response_line(response_line: &str) -> Result<Status, crate::Error> {
        let mut response_line = response_line.split(" ");

        // Verify HTTP version
        match response_line.next() {
            Some(line) => {
                if line != "HTTP/1.1" {
                    return Err(crate::Error::BadResponse);
                }
            }
            None => return Err(crate::Error::BadResponse),
        }

        // Get status code
        let status_code = match response_line.next() {
            Some(line) => match usize::from_str_radix(line, 10) {
                Ok(value) => value,
                Err(_) => return Err(crate::Error::BadResponse),
            },
            None => return Err(crate::Error::BadResponse),
        };

        // Get status name
        let status_name: Vec<&str> = response_line.collect();
        let mut status_str = String::new();
        for str in status_name {
            status_str.push_str(str);
            status_str.push(' ');
        }

        // Parse and verify status
        Status::parse(status_code, status_str.trim())
    }

    pub fn status(&self) -> Status {
        self.status
    }

    pub fn headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    pub fn body(&self) -> &Vec<u8> {
        &self.body
    }
}

impl Status {
    pub fn code(&self) -> usize {
        *self as usize
    }

    pub fn parse(code: usize, name: &str) -> Result<Self, crate::Error> {
        match code {
            100 => {
                if name != "Continue" {
                    Err(crate::Error::BadResponse)
                } else {
                    Ok(Status::Continue)
                }
            }
            101 => {
                if name != "Switching Protocols" {
                    Err(crate::Error::BadResponse)
                } else {
                    Ok(Status::SwitchingProtocols)
                }
            }
            200 => {
                if name != "OK" {
                    Err(crate::Error::BadResponse)
                } else {
                    Ok(Status::OK)
                }
            }
            201 => {
                if name != "Created" {
                    Err(crate::Error::BadResponse)
                } else {
                    Ok(Status::Created)
                }
            }
            202 => {
                if name != "Accepted" {
                    Err(crate::Error::BadResponse)
                } else {
                    Ok(Status::Accepted)
                }
            }
            203 => {
                if name != "Non-Authoritative Information" {
                    Err(crate::Error::BadResponse)
                } else {
                    Ok(Status::NonAuthoritativeInformation)
                }
            }
            204 => {
                if name != "No Content" {
                    Err(crate::Error::BadResponse)
                } else {
                    Ok(Status::NoContent)
                }
            }
            205 => {
                if name != "Reset Content" {
                    Err(crate::Error::BadResponse)
                } else {
                    Ok(Status::ResetContent)
                }
            }
            206 => {
                if name != "Partial Content" {
                    Err(crate::Error::BadResponse)
                } else {
                    Ok(Status::PartialContent)
                }
            }
            300 => {
                if name != "Multiple Choices" {
                    Err(crate::Error::BadResponse)
                } else {
                    Ok(Status::MultipleChoices)
                }
            }
            301 => {
                if name != "Moved Permanently" {
                    Err(crate::Error::BadResponse)
                } else {
                    Ok(Status::MovedPermanently)
                }
            }
            302 => {
                if name != "Found" {
                    Err(crate::Error::BadResponse)
                } else {
                    Ok(Status::Found)
                }
            }
            303 => {
                if name != "See Other" {
                    Err(crate::Error::BadResponse)
                } else {
                    Ok(Status::SeeOther)
                }
            }
            304 => {
                if name != "Not Modified" {
                    Err(crate::Error::BadResponse)
                } else {
                    Ok(Status::NotModified)
                }
            }
            305 => {
                if name != "Use Proxy" {
                    Err(crate::Error::BadResponse)
                } else {
                    Ok(Status::UseProxy)
                }
            }
            307 => {
                if name != "Temporary Redirect" {
                    Err(crate::Error::BadResponse)
                } else {
                    Ok(Status::TemporaryRedirect)
                }
            }
            400 => {
                if name != "Bad Request" {
                    Err(crate::Error::BadResponse)
                } else {
                    Ok(Status::BadRequest)
                }
            }
            401 => {
                if name != "Unauthorized" {
                    Err(crate::Error::BadResponse)
                } else {
                    Ok(Status::Unauthorized)
                }
            }
            402 => {
                if name != "Payment Required" {
                    Err(crate::Error::BadResponse)
                } else {
                    Ok(Status::PaymentRequired)
                }
            }
            403 => {
                if name != "Forbidden" {
                    Err(crate::Error::BadResponse)
                } else {
                    Ok(Status::Forbidden)
                }
            }
            404 => {
                if name != "Not Found" {
                    Err(crate::Error::BadResponse)
                } else {
                    Ok(Status::NotFound)
                }
            }
            405 => {
                if name != "Method Not Allowed" {
                    Err(crate::Error::BadResponse)
                } else {
                    Ok(Status::MethodNotAllowed)
                }
            }
            406 => {
                if name != "Not Acceptable" {
                    Err(crate::Error::BadResponse)
                } else {
                    Ok(Status::NotAcceptable)
                }
            }
            407 => {
                if name != "Proxy Authentication Required" {
                    Err(crate::Error::BadResponse)
                } else {
                    Ok(Status::ProxyAuthenticationRequired)
                }
            }
            408 => {
                if name != "Request Time-out" {
                    Err(crate::Error::BadResponse)
                } else {
                    Ok(Status::RequestTimeout)
                }
            }
            409 => {
                if name != "Conflict" {
                    Err(crate::Error::BadResponse)
                } else {
                    Ok(Status::Conflict)
                }
            }
            410 => {
                if name != "Gone" {
                    Err(crate::Error::BadResponse)
                } else {
                    Ok(Status::Gone)
                }
            }
            411 => {
                if name != "Length Required" {
                    Err(crate::Error::BadResponse)
                } else {
                    Ok(Status::LengthRequired)
                }
            }
            412 => {
                if name != "Precondition Failed" {
                    Err(crate::Error::BadResponse)
                } else {
                    Ok(Status::PreconditionFailed)
                }
            }
            413 => {
                if name != "Request Entity Too Large" {
                    Err(crate::Error::BadResponse)
                } else {
                    Ok(Status::RequestEntityTooLarge)
                }
            }
            414 => {
                if name != "Request-URI Too Large" {
                    Err(crate::Error::BadResponse)
                } else {
                    Ok(Status::RequestURITooLarge)
                }
            }
            415 => {
                if name != "Unsupported Media Type" {
                    Err(crate::Error::BadResponse)
                } else {
                    Ok(Status::UnsupportedMediaType)
                }
            }
            416 => {
                if name != "Requested Range Not Satisfiable" {
                    Err(crate::Error::BadResponse)
                } else {
                    Ok(Status::RequestedRangeNotSatisfiable)
                }
            }
            417 => {
                if name != "Expectation Failed" {
                    Err(crate::Error::BadResponse)
                } else {
                    Ok(Status::ExpectationFailed)
                }
            }
            500 => {
                if name != "Internal Server Error" {
                    Err(crate::Error::BadResponse)
                } else {
                    Ok(Status::InternalServerError)
                }
            }
            501 => {
                if name != "Not Implemented" {
                    Err(crate::Error::BadResponse)
                } else {
                    Ok(Status::NotImplemented)
                }
            }
            502 => {
                if name != "Bad Gateway" {
                    Err(crate::Error::BadResponse)
                } else {
                    Ok(Status::BadGateway)
                }
            }
            503 => {
                if name != "Service Unavailable" {
                    Err(crate::Error::BadResponse)
                } else {
                    Ok(Status::ServiceUnavailable)
                }
            }
            504 => {
                if name != "Gateway Time-out" {
                    Err(crate::Error::BadResponse)
                } else {
                    Ok(Status::GatewayTimeout)
                }
            }
            505 => {
                if name != "HTTP Version Not Supported" {
                    Err(crate::Error::BadResponse)
                } else {
                    Ok(Status::HTTPVersionNotSupported)
                }
            }
            _ => Err(crate::Error::BadResponse),
        }
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
