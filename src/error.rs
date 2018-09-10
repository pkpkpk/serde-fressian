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
    Message(String),
    Io(io::Error),
    Eof,
    //// serialization errors
    UnsupportedType,
    Unsupported_TA_Type,
    Unsupported_Cache_Type, //temporary
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
    Msg,
    De,
    Ser
}

impl ErrorImpl{
    pub fn classify(&self) -> Category {
        match self.code {
            ErrorCode::Eof => Category::Eof,
            /////////////////////////
            _ => Category::De
        }
    }
}

impl Serialize for ErrorImpl {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {

        // would be nice to distinguish writing from reading errors rather
        // write position is not very useful
        let mut map_state = serializer.serialize_map(None)?;

        map_state.serialize_key("type")?;
        map_state.serialize_value("serde-fressian")?;

        map_state.serialize_key("category")?;
        map_state.serialize_value(&self.classify())?;

        map_state.serialize_key("position")?;
        map_state.serialize_value(&self.position)?;

        map_state.serialize_key("ErrorCode")?;
        // map_state.serialize_value(&self.code)?;

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
                // serializer.serialize_unit_variant("",0, &self)
                // serializer.serialize_str(&self.code.to_string())
            }
        }

        map_state.end()
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
            // Eof,
            ErrorCode::Message(ref msg) => f.write_str(msg),
            ErrorCode::Io(ref err) => Display::fmt(err, f),
            ErrorCode::UnmatchedCode(code) => f.write_str(format!("unmatched code: {}", code).as_ref()),
            ErrorCode::Unsupported_TA_Type => f.write_str("Unsupported TypedArray Ser type"),
            ErrorCode::Unsupported_Cache_Type => f.write_str("Unsupported Cache type"),
            ErrorCode::AttemptToReadPastEnd => f.write_str("attempted to read past end!"),
            ErrorCode::UnexpectedEof => f.write_str("unexpected EOF"),
            ErrorCode::ExpectedListCode => f.write_str("deserializing seq, expected list code"),
            ErrorCode::MapExpectedListCode => f.write_str("deserializing map, expected list code"),
            ErrorCode::IntTooLargeFori64 => f.write_str("int cannot fit inside signed i64"),
            ErrorCode::Expectedi64 => f.write_str("expected i64"),
            ErrorCode::ExpectedDoubleCode => f.write_str("expected double code"),
            ErrorCode::ExpectedFloatCode => f.write_str("expected float code"),
            ErrorCode::ExpectedBooleanCode => f.write_str("expected boolean code"),
            ErrorCode::ExpectedChunkBytesConclusion => f.write_str("expected bytes conclusion (following chunks)"),
            ErrorCode::ExpectedBytesCode => f.write_str("expected bytes code"),
            ErrorCode::InvalidUTF8 => f.write_str("deserialized invalid utf8"),
            ErrorCode::ExpectedStringCode => f.write_str("expected string code"),
            ErrorCode::ExpectedNonZeroReadLength => f.write_str("expected non-zero read length"),
            _ => f.write_str(self.to_string().as_ref())
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

