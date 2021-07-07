use crate::Error;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum GeneralHeader {
    CacheControl,
    Connection,
    Date,
    Pragma,
    Trailer,
    TransferEncoding,
    Upgrade,
    Via,
    Warning,
}

impl GeneralHeader {
    pub fn parse(line: &str) -> Result<(Self, String), Error> {
        let (key, value) = match line.split_once(':') {
            Some((key, value)) => (key.trim(), value.trim()),
            None => return Err(Error::BadRequest),
        };

        match key {
            "Cache-Control" => Ok((GeneralHeader::CacheControl, value.to_string())),
            "Connection" => Ok((GeneralHeader::Connection, value.to_string())),
            "Date" => Ok((GeneralHeader::Date, value.to_string())),
            "Pragma" => Ok((GeneralHeader::Pragma, value.to_string())),
            "Trailer" => Ok((GeneralHeader::Trailer, value.to_string())),
            "Transfer-Encoding" => Ok((GeneralHeader::TransferEncoding, value.to_string())),
            "Upgrade" => Ok((GeneralHeader::Upgrade, value.to_string())),
            "Via" => Ok((GeneralHeader::Via, value.to_string())),
            "Warning" => Ok((GeneralHeader::Warning, value.to_string())),
            _ => Err(Error::InvalidHeader),
        }
    }
}

impl std::fmt::Display for GeneralHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                GeneralHeader::CacheControl => "Cache-Control",
                GeneralHeader::Connection => "Connection",
                GeneralHeader::Date => "Date",
                GeneralHeader::Pragma => "Pragma",
                GeneralHeader::Trailer => "Trailer",
                GeneralHeader::TransferEncoding => "Transfer-Encoding",
                GeneralHeader::Upgrade => "Upgrade",
                GeneralHeader::Via => "Via",
                GeneralHeader::Warning => "Warning",
            }
        )
    }
}
