#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(non_snake_case)]
#![feature(custom_attribute)]


#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_bytes;
extern crate serde_fressian;
extern crate ordered_float;

use std::collections::{HashMap, HashSet};
use serde::de::{Deserialize};
use serde::Serialize;

use serde_fressian::ser::{self, Serializer, FressianWriter};
use serde_fressian::de::{self, Deserializer, from_vec};



use serde_fressian::value::{Value};
use ordered_float::OrderedFloat;

#[test]
fn value_de_test(){

    // (write true)
    let control_bytes: Vec<u8> = vec![245];
    let b: bool = de::from_vec(&control_bytes).unwrap();
    assert_eq!(b,true);
    let val: Value = de::from_vec(&control_bytes).unwrap();
    assert_eq!(val, Value::BOOL(true));

    // (write 32.2)
    let control_bytes: Vec<u8> = vec![249,66,0,204,205];
    let f: f32 = de::from_vec(&control_bytes).unwrap();
    assert_eq!(f, 32.2);
    let val: Value = de::from_vec(&control_bytes).unwrap();
    assert_eq!(val, Value::FLOAT(OrderedFloat::from(32.2)));

    // (fress.writer/writeDouble w 64.4)
    let control_bytes: Vec<u8> = vec![250,64,80,25,153,153,153,153,154];
    let d: f64 = de::from_vec(&control_bytes).unwrap();
    assert_eq!(d, 64.4 as f64);
    let val: Value = de::from_vec(&control_bytes).unwrap();
    assert_eq!(val, Value::DOUBLE(OrderedFloat::from(d)));
}