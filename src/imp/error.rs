extern crate serde;

use std;
use std::fmt::{self, Display};

use serde::{ser, de};

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    Message(String),
    UnmatchedCode(u8),
    Eof,
    Syntax,
}

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
         Error::Message(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
         Error::Message(msg.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(std::error::Error::description(self))
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Message(ref msg) => msg,
            Error::Eof => "unexpected end of input",
            /* and so forth */
            _ => unimplemented!(),
        }
    }
}

