use crate::Error;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum EntityHeader {
    Allow(String),
    ContentEncoding(String),
    ContentLanguage(String),
    ContentLength(String),
    ContentLocation(String),
    ContentMD5(String),
    ContentRange(String),
    ContentType(String),
    Expires(String),
    LastModified(String),
}

impl EntityHeader {
    pub fn parse(line: &str) -> Result<Self, Error> {
        let (key, value) = match line.split_once(':') {
            Some((key, value)) => (key.trim(), value.trim()),
            None => return Err(Error::BadRequest),
        };

        match key {
            "Allow" => Ok(EntityHeader::Allow(value.to_string())),
            "Content-Encoding" => Ok(EntityHeader::ContentEncoding(value.to_string())),
            "Content-Language" => Ok(EntityHeader::ContentLanguage(value.to_string())),
            "Content-Length" => Ok(EntityHeader::ContentLength(value.to_string())),
            "Content-Location" => Ok(EntityHeader::ContentLocation(value.to_string())),
            "Content-MD5" => Ok(EntityHeader::ContentMD5(value.to_string())),
            "Content-Range" => Ok(EntityHeader::ContentRange(value.to_string())),
            "Content-Type" => Ok(EntityHeader::ContentType(value.to_string())),
            "Expires" => Ok(EntityHeader::Expires(value.to_string())),
            "Last-Modified" => Ok(EntityHeader::LastModified(value.to_string())),
            _ => Err(Error::InvalidHeader),
        }
    }
}
