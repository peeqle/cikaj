use std::io;
use std::net::AddrParseError;

#[derive(Debug)]
pub enum SocketInitErrors {
    Addr(AddrParseError),
    Plain(io::Error),
}
impl From<AddrParseError> for SocketInitErrors {
    fn from(value: AddrParseError) -> Self {
        SocketInitErrors::Addr(value)
    }
}
impl From<io::Error> for SocketInitErrors {
    fn from(value: io::Error) -> Self {
        SocketInitErrors::Plain(value)
    }
}
