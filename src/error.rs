extern crate serde;

use std;
use std::fmt::{self, Display};

use serde::{ser, de};

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Clone, Debug, PartialEq)]
pub struct Error {
    /// Serde_json::
    ///   This `Box` allows us to keep the size of `Error` as small as possible. A
    ///   larger `Error` type was substantially slower due to all the functions
    ///   that pass around `Result<T, Error>`.
    err: Box<ErrorImpl>,
}

// #[derive(Copy, Clone, PartialEq, Eq, Debug)]
// pub enum Category {
//     Io, /// The error was caused by a failure to read or write bytes on an IO stream.
//     Syntax, /// The error was caused by input that was not syntactically valid JSON.
//     Data,/// The error was caused by input data that was semantically incorrect.
//     Eof,
// }

#[derive(Clone, Debug, PartialEq)]
pub enum ErrorCode{
    Msg(Box<str>),
    Message(String),
    UnmatchedCode(u8),
    UnsupportedType,
    Eof,
    Syntax,
    Expectedi64,
    ExpectedDoubleCode,
    ExpectedFloatCode,
    ExpectedBooleanCode,
    ExpectedChunkBytesConclusion,
    ExpectedBytesCode,
    InvalidUTF8,
    ExpectedStringCode,
    ExpectedNonZeroReadLength,
    IntTooLargeFori64,
}

#[derive(Clone, Debug, PartialEq)]
struct ErrorImpl {
    code: ErrorCode,
    position: usize,
    // would be nice to distinguish writing from reading errors rather
    // than generic position property,
    // write position is not very useful
}


impl Error {
    pub fn msg(msg: String) -> Self {
        Error {
            err: Box::new(ErrorImpl {
                code: ErrorCode::Message(msg),
                position: 0,
            }),
        }
    }

    pub fn syntax(code: ErrorCode, position: usize) -> Self {
        Error {
            err: Box::new(ErrorImpl {
                code: code,
                position: position,
            }),
        }
    }

    pub fn Eof(position: usize) -> Self {
        Error {
            err: Box::new(ErrorImpl {
                code: ErrorCode::Eof,
                position: position,
            }),
        }
    }

    // pub fn fix_position<F>(self, f: F) -> Self
    // where
    //     F: FnOnce(ErrorCode) -> Error,
    // {
    //     if self.err.line == 0 {
    //         f(self.err.code)
    //     } else {
    //         self
    //     }
    // }
}


impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
         Error::msg(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
         Error::msg(msg.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(std::error::Error::description(self))
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        let err = *self.err;
        let code = err.code;
        match code {
            ErrorCode::Message(ref msg) => msg,
            ErrorCode::Eof => "unexpected end of input",
            /////////////////////////////////////////////////////////////////////////
            _ => unimplemented!(),
        }
    }
}

