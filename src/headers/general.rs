use crate::Error;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum GeneralHeader {
    CacheControl(String),
    Connection(String),
    Date(String),
    Pragma(String),
    Trailer(String),
    TransferEncoding(String),
    Upgrade(String),
    Via(String),
    Warning(String),
}

impl GeneralHeader {
    pub fn parse(line: &str) -> Result<Self, Error> {
        let (key, value) = match line.split_once(':') {
            Some((key, value)) => (key.trim(), value.trim()),
            None => return Err(Error::BadRequest),
        };

        match key {
            "Cache-Control" => Ok(GeneralHeader::CacheControl(value.to_string())),
            "Connection" => Ok(GeneralHeader::Connection(value.to_string())),
            "Date" => Ok(GeneralHeader::Date(value.to_string())),
            "Pragma" => Ok(GeneralHeader::Pragma(value.to_string())),
            "Trailer" => match value {
                "Transfer-Encoding" | "Content-Length" | "Trailer" => Err(Error::BadRequest),
                _ => Ok(GeneralHeader::Trailer(value.to_string())),
            },
            "Transfer-Encoding" => Ok(GeneralHeader::TransferEncoding(value.to_string())),
            "Upgrade" => Ok(GeneralHeader::Upgrade(value.to_string())),
            "Via" => Ok(GeneralHeader::Via(value.to_string())),
            "Warning" => Ok(GeneralHeader::Warning(value.to_string())),
            _ => Err(Error::InvalidHeader),
        }
    }
}
