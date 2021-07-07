use crate::Error;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum EntityHeader {
    Allow,
    ContentEncoding,
    ContentLanguage,
    ContentLength,
    ContentLocation,
    ContentMD5,
    ContentRange,
    ContentType,
    Expires,
    LastModified,
}

impl EntityHeader {
    pub fn parse(line: &str) -> Result<(Self, String), Error> {
        let (key, value) = match line.split_once(':') {
            Some((key, value)) => (key.trim(), value.trim()),
            None => return Err(Error::BadRequest),
        };

        match key {
            "Allow" => Ok((EntityHeader::Allow, value.to_string())),
            "Content-Encoding" => Ok((EntityHeader::ContentEncoding, value.to_string())),
            "Content-Language" => Ok((EntityHeader::ContentLanguage, value.to_string())),
            "Content-Length" => Ok((EntityHeader::ContentLength, value.to_string())),
            "Content-Location" => Ok((EntityHeader::ContentLocation, value.to_string())),
            "Content-MD5" => Ok((EntityHeader::ContentMD5, value.to_string())),
            "Content-Range" => Ok((EntityHeader::ContentRange, value.to_string())),
            "Content-Type" => Ok((EntityHeader::ContentType, value.to_string())),
            "Expires" => Ok((EntityHeader::Expires, value.to_string())),
            "Last-Modified" => Ok((EntityHeader::LastModified, value.to_string())),
            _ => Err(Error::InvalidHeader),
        }
    }
}

impl std::fmt::Display for EntityHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                EntityHeader::Allow => "Allow",
                EntityHeader::ContentEncoding => "Content-Encoding",
                EntityHeader::ContentLanguage => "Content-Language",
                EntityHeader::ContentLength => "Content-Length",
                EntityHeader::ContentLocation => "Content-Location",
                EntityHeader::ContentMD5 => "Content-MD5",
                EntityHeader::ContentRange => "Content-Range",
                EntityHeader::ContentType => "Content-Type",
                EntityHeader::Expires => "Expires",
                EntityHeader::LastModified => "Last-Modified",
            }
        )
    }
}
