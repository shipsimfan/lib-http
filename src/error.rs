#[derive(Debug)]
pub enum Error {
    // System Errors
    AcceptConnectionError(std::io::Error),
    RequestConversionError(std::string::FromUtf8Error),
    RequestReadError(std::io::Error),
    ResponseWriteError(std::io::Error),
    BadRequest,
    InvalidHeader,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Error::AcceptConnectionError(error) =>
                    format!("Unable to accept client - {}", error),
                Error::RequestConversionError(error) =>
                    format!("Unable to convert request - {}", error),
                Error::RequestReadError(error) => format!("Unable to read request - {}", error),
                Error::ResponseWriteError(error) => format!("Unable to write response - {}", error),
                Error::BadRequest => format!("Bad request"),
                Error::InvalidHeader => format!("Invalid header"),
            }
        )
    }
}
