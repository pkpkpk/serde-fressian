#![feature(extern_prelude)]
#![feature(try_from)]

#[macro_use]
extern crate itertools;

#[macro_use]
extern crate shrinkwraprs;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_bytes;

extern crate byteorder;
extern crate ordered_float;
extern crate uuid;
// extern crate chrono;
extern crate url;
extern crate regex;

mod imp;
// pub use imp::{error};
pub mod error;
pub mod ser;
pub mod de;
pub mod value;
pub mod types;
pub mod wasm;

pub use types::{
    INST,
    UUID,
    URI,
    REGEX,
    SYM,
    KEY,
    typed_arrays,
    SET
};


