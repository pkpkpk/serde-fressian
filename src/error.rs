extern crate serde;

use std;
use std::fmt::{self, Debug, Display};
use serde::{de};
use serde::ser::{self,Serialize, Serializer, SerializeMap};
use std::io;

pub type Result<T> = ::std::result::Result<T, Error>;

pub struct Error {
    pub err: Box<ErrorImpl>,
}

pub enum ErrorCode {
    Eof,
    Io(io::Error),
    Message(String),
    UnsupportedType,
    //// serialization errors
    UnsupportedTAType,
    UnsupportedCacheType, //temporary
    IntTooLargeFori64,
    //// deserialization errors
    UnmatchedCode(u8),
    Expectedi64, //rawinput read_int got bad int
    ExpectedNonZeroReadLength, //gave length 0 to ByteReader::read_bytes
    ExpectedDoubleCode,
    ExpectedFloatCode,
    ExpectedBooleanCode,
    ExpectedChunkBytesConclusion,
    ExpectedBytesCode,
    ExpectedStringCode,
    MapExpectedListCode,
    ExpectedListCode,
    InvalidUTF8,
    UnexpectedEof,
    AttemptToReadPastEnd,
}

pub struct ErrorImpl {
    pub code: ErrorCode,
    pub position: usize,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Serialize)]
pub enum Category {
    Io,
    Eof,
    De,
    Ser,
    Misc
}

impl ErrorImpl{
    pub fn classify(&self) -> Category {
        match self.code {
            ErrorCode::Eof => Category::Eof,

            ErrorCode::Io(_) => Category::Io,

            ErrorCode::UnsupportedType
            | ErrorCode::Message(_) => Category::Misc,

            ErrorCode::UnsupportedTAType
            | ErrorCode::UnsupportedCacheType
            | ErrorCode::IntTooLargeFori64 => Category::Ser,

            _ => Category::De
        }
    }
}

impl Serialize for ErrorImpl {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map_state = serializer.serialize_map(None)?;

        map_state.serialize_key("type")?;
        map_state.serialize_value("serde-fressian")?;

        map_state.serialize_key("category")?;
        map_state.serialize_value(&self.classify())?;

        map_state.serialize_key("position")?;
        map_state.serialize_value(&self.position)?;

        map_state.serialize_key("ErrorCode")?;

        match &self.code {
            ErrorCode::Io(_) => {
                map_state.serialize_value("IO::Error")?;
            }
            ErrorCode::Message(msg) => {
                map_state.serialize_value("Message")?;
                map_state.serialize_key("value")?;
                map_state.serialize_value(&msg)?;
            }
            ErrorCode::UnmatchedCode(code) => {
                map_state.serialize_value("UnmatchedCode")?;
                map_state.serialize_key("value")?;
                map_state.serialize_value(&code)?;
            }
            _ => {
                map_state.serialize_value(&self.code.to_string())?;
            }
        }

        map_state.end()
    }
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.err.serialize(serializer)
    }
}

impl Error {
    pub fn is_eof(&self) -> bool { self.classify() == Category::Eof }

    pub fn classify(&self) -> Category { self.err.classify() }

    pub fn msg(msg: String) -> Self {
        Error {
            err: Box::new(ErrorImpl {
                code: ErrorCode::Message(msg),
                position: 0,
            }),
        }
    }

    //rename
    pub fn syntax(code: ErrorCode, position: usize) -> Self {
        Error {
            err: Box::new(ErrorImpl {
                code: code,
                position: position,
            }),
        }
    }

    pub fn unmatched_code(code: u8, position: usize) -> Self {
        Error {
            err: Box::new(ErrorImpl {
                code: ErrorCode::UnmatchedCode(code),
                position: position,
            }),
        }
    }

    pub fn eof(position: usize) -> Self {
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
            ErrorCode::Eof => f.write_str("Eof"),
            ErrorCode::Message(ref msg) => f.write_str(msg),
            ErrorCode::Io(ref err) => Display::fmt(err, f),
            ErrorCode::UnmatchedCode(_code) => f.write_str("UnmatchedCode"),
            ErrorCode::UnsupportedType => f.write_str("UnsupportedType"),
            ErrorCode::UnsupportedTAType => f.write_str("UnsupportedTAType"),
            ErrorCode::UnsupportedCacheType => f.write_str("UnsupportedCacheType"),
            ErrorCode::AttemptToReadPastEnd => f.write_str("AttemptToReadPastEnd"),
            ErrorCode::UnexpectedEof => f.write_str("UnexpectedEof"),
            ErrorCode::ExpectedListCode => f.write_str("ExpectedListCode"),
            ErrorCode::MapExpectedListCode => f.write_str("MapExpectedListCode"),
            ErrorCode::IntTooLargeFori64 => f.write_str("IntTooLargeFori64"),
            ErrorCode::Expectedi64 => f.write_str("Expectedi64"),
            ErrorCode::ExpectedDoubleCode => f.write_str("ExpectedDoubleCode"),
            ErrorCode::ExpectedFloatCode => f.write_str("ExpectedFloatCode"),
            ErrorCode::ExpectedBooleanCode => f.write_str("ExpectedBooleanCode"),
            ErrorCode::ExpectedChunkBytesConclusion => f.write_str("ExpectedChunkBytesConclusion"),
            ErrorCode::ExpectedBytesCode => f.write_str("ExpectedBytesCode"),
            ErrorCode::InvalidUTF8 => f.write_str("InvalidUTF8"),
            ErrorCode::ExpectedStringCode => f.write_str("ExpectedStringCode"),
            ErrorCode::ExpectedNonZeroReadLength => f.write_str("ExpectedNonZeroReadLength")
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

impl Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Error({:?}, byte-position: {})",
            self.err.code.to_string(), // is what unwrap() shows.
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

