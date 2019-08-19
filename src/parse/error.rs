use std::str::Utf8Error;

pub(crate) type BitsError<'a> = ((&'a [u8], usize), nom::error::ErrorKind);
pub(crate) type ParseError<'a> = nom::Err<(&'a [u8], nom::error::ErrorKind)>;

#[derive(Debug)]
pub enum DnsParseError<'a> {
    ParseError(ParseError<'a>),
    InvalidUtf8(Utf8Error),
    InvalidLabel,
}

impl<'a> From<ParseError<'a>> for DnsParseError<'a> {
    fn from(err: ParseError<'a>) -> Self {
        DnsParseError::ParseError(err)
    }
}

impl From<Utf8Error> for DnsParseError<'static> {
    fn from(err: Utf8Error) -> Self {
        DnsParseError::InvalidUtf8(err)
    }
}
