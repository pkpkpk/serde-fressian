#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(non_snake_case)]
#![feature(custom_attribute)]


#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_bytes;
extern crate serde_fressian;
extern crate uuid;
extern crate url;
extern crate regex;
// extern crate chrono;

use std::collections::{HashMap, HashSet};
use serde::de::{Deserialize};
use serde::Serialize;

use serde_fressian::ser::{self, Serializer, FressianWriter};
use serde_fressian::de::{self, Deserializer, from_vec};


// use serde_fressian::INST::{INST};
// use serde_fressian::UUID::{UUID};
// use serde_fressian::URI::{URI};
// use serde_fressian::REGEX::{REGEX};
// use serde_fressian::SYM::{SYM};
// use serde_fressian::KEY::{KEY};
// use serde_fressian::typed_arrays::*;

use serde_fressian::value;