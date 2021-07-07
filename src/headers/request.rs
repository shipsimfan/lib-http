use crate::Error;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum RequestHeader {
    Accept(String),
    AcceptCharset(String),
    AcceptEncoding(String),
    AcceptLanguage(String),
    Authorization(String),
    Except(String),
    From(String),
    Host(String),
    IfMatch(String),
    IfModifiedSince(String),
    IfNoneMatch(String),
    IfRange(String),
    IfUnmodifiedSince(String),
    MaxForwards(String),
    ProxyAuthorization(String),
    Range(String),
    Referer(String),
    TE(String),
    UserAgent(String),
}

impl RequestHeader {
    pub fn parse(line: &str) -> Result<Self, Error> {
        let (key, value) = match line.split_once(':') {
            Some((key, value)) => (key.trim(), value.trim()),
            None => return Err(Error::BadRequest),
        };

        match key {
            "Accept" => Ok(RequestHeader::Accept(value.to_string())),
            "Accept-Charset" => Ok(RequestHeader::AcceptCharset(value.to_string())),
            "Accept-Encoding" => Ok(RequestHeader::AcceptEncoding(value.to_string())),
            "Accept-Language" => Ok(RequestHeader::AcceptLanguage(value.to_string())),
            "Authorization" => Ok(RequestHeader::Authorization(value.to_string())),
            "Except" => Ok(RequestHeader::Except(value.to_string())),
            "From" => Ok(RequestHeader::From(value.to_string())),
            "Host" => Ok(RequestHeader::Host(value.to_string())),
            "If-Match" => Ok(RequestHeader::IfMatch(value.to_string())),
            "If-Modified-Since" => Ok(RequestHeader::IfModifiedSince(value.to_string())),
            "If-None-Match" => Ok(RequestHeader::IfNoneMatch(value.to_string())),
            "If-Range" => Ok(RequestHeader::IfRange(value.to_string())),
            "If-Unmodified-Since" => Ok(RequestHeader::IfUnmodifiedSince(value.to_string())),
            "Max-Forwards" => Ok(RequestHeader::MaxForwards(value.to_string())),
            "Proxy-Authorization" => Ok(RequestHeader::ProxyAuthorization(value.to_string())),
            "Range" => Ok(RequestHeader::Range(value.to_string())),
            "Referer" => Ok(RequestHeader::Referer(value.to_string())),
            "TE" => Ok(RequestHeader::TE(value.to_string())),
            "User-Agent" => Ok(RequestHeader::UserAgent(value.to_string())),
            _ => Err(Error::InvalidHeader),
        }
    }
}
