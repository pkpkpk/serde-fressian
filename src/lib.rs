#![feature(extern_prelude)]
#![feature(try_from)]
#![feature(use_extern_macros)]

#[macro_use]
extern crate itertools;

#[macro_use]
extern crate serde_derive;

extern crate serde;

extern crate byteorder;

mod imp;
pub mod ser;
pub mod de;


