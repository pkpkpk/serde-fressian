#![cfg_attr(feature= "use_regex_crate", use_regex_crate)]
#![cfg_attr(feature= "use_uuid_crate", use_uuid_crate)]
#![cfg_attr(feature= "use_url_crate", use_url_crate)]

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

#[cfg(use_uuid_crate)]
extern crate uuid as _uuid;

#[cfg(use_url_crate)]
extern crate url;

// extern crate chrono;

mod imp;
pub mod error;
pub mod ser;
pub mod de;
pub mod value;
pub mod types;
pub mod wasm;

pub use crate::types::{
    inst,
    uuid,
    uri,
    regex,
    sym,
    key,
    typed_arrays,
    set
};

pub use crate::imp::cache;
