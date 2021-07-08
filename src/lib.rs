mod error;
mod request;
mod response;
pub mod server;

use std::borrow::Cow;

pub use error::Error;
pub use request::Method;
pub use request::Request;
pub use response::Response;
pub use response::Status;

fn get_next_u8<'a, I>(iter: &mut I) -> Result<u8, ()>
where
    I: Iterator<Item = &'a u8>,
{
    match iter.next() {
        None => Err(()),
        Some(val) => Ok(*val),
    }
}

// Splits a message into its header and body
fn split_http_message(buffer: Vec<u8>) -> Result<(String, Vec<u8>), ()> {
    let mut iter = buffer.iter();
    let mut i = 0;
    loop {
        i += 1;
        if get_next_u8(&mut iter)? != '\r' as u8 {
            continue;
        }

        i += 1;
        if get_next_u8(&mut iter)? != '\n' as u8 {
            continue;
        }

        i += 1;
        if get_next_u8(&mut iter)? != '\r' as u8 {
            continue;
        }

        i += 1;
        if get_next_u8(&mut iter)? != '\n' as u8 {
            continue;
        }

        let header = match String::from_utf8_lossy(&buffer[..i - 4]) {
            Cow::Borrowed(str) => str.to_string(),
            Cow::Owned(_) => return Err(()),
        };

        let body = if i == buffer.len() {
            Vec::new()
        } else {
            Vec::from(&buffer[i..])
        };

        return Ok((header, body));
    }
}
