#![feature(extern_prelude)]
#![cfg_attr(feature= "use_regex_crate", use_regex_crate)]

extern crate itertools;

#[macro_use]
extern crate shrinkwraprs;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_bytes;
extern crate byteorder;
extern crate ordered_float;

#[cfg(use_regex_crate)]
extern crate regex as _regex;

extern crate uuid as _uuid;
extern crate url;
// extern crate chrono;

mod imp;
pub mod error;
pub mod ser;
pub mod de;
pub mod value;
pub mod types;
pub mod wasm;

pub use types::{
    inst,
    uuid,
    uri,
    regex,
    sym,
    key,
    typed_arrays,
    set
};


