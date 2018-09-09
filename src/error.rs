extern crate serde;

use std;
use std::fmt::{self, Debug, Display};

use serde::{ser, de};

pub type Result<T> = ::std::result::Result<T, Error>;

// #[derive(Debug, PartialEq)]
// #[derive(Debug)]
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
use std::io;

// #[derive(Debug, PartialEq)]
// #[derive(Debug)]
pub enum ErrorCode{
    // Msg(Box<str>),
    Message(String),
    UnmatchedCode(u8),
    /// Some IO error occurred while serializing or deserializing.
    Io(io::Error),
    UnsupportedType,
    UnsupportedNamedType(String),
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
    MapExpectedListCode,
    ExpectedListCode,
    UnexpectedEof,
    AttemptToReadPastEnd,
}

// #[derive(Debug, PartialEq)]
struct ErrorImpl {
    code: ErrorCode,
    position: usize,
    // would be nice to distinguish writing from reading errors rather
    // than generic position property,
    // write position is not very useful
}


#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Category {
    Io, /// The error was caused by a failure to read or write bytes on an IO stream.
    Syntax, /// The error was caused by input that was not syntactically valid JSON.
    Eof,
}

impl Error {
    pub fn is_eof(&self) -> bool { self.classify() == Category::Eof }

    pub fn classify(&self) -> Category {
        match self.err.code{
            ErrorCode::Eof => Category::Eof,
            /////////////////////////
            _ => Category::Syntax
        }
    }

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
    pub fn io(error: io::Error) -> Self {
        Error {
            err: Box::new(ErrorImpl {
                code: ErrorCode::Io(error),
                position: 0
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

impl Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            // UnsupportedType,
            // Eof,
            // Syntax,
            // Expectedi64,
            // ExpectedDoubleCode,
            // ExpectedFloatCode,
            // ExpectedBooleanCode,
            // ExpectedChunkBytesConclusion,
            // ExpectedBytesCode,
            // InvalidUTF8,
            // ExpectedStringCode,
            // ExpectedNonZeroReadLength,
            // IntTooLargeFori64,
            ErrorCode::Message(ref msg) => f.write_str(msg),
            ErrorCode::Io(ref err) => Display::fmt(err, f),
            ErrorCode::UnmatchedCode(code) => f.write_str(format!("unmatched code: {}", code).as_ref()),
            // ErrorCode::InvalidNumber => f.write_str("invalid number"),
            _ => f.write_str("need to finished Display for ErrorCode: ")
        }
    }
}


impl Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(std::error::Error::description(self))
    }
}

impl Display for ErrorImpl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.position == 0 {
            Display::fmt(&self.code, f)
        } else {
            write!(
                f,
                "{} at byte-position {}",
                self.code, self.position
            )
        }
    }
}

// Remove two layers of verbosity from the debug representation. Humans often
// end up seeing this representation because it is what unwrap() shows.
impl Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Error({:?}, byte-position: {})",
            self.err.code.to_string(),
            self.err.position
        )
    }
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



impl std::error::Error for Error {
    fn description(&self) -> &str {
        match self.err.code {
            ErrorCode::Io(ref err) => std::error::Error::description(err),
            // should use Display::fmt or to_string().
            _ => "fressian error!",
        }
    }
}

