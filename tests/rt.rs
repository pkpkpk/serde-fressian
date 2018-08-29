#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(non_snake_case)]
#![feature(custom_attribute)]

#[macro_use] extern crate maplit;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_bytes;
extern crate serde_fressian;
extern crate ordered_float;

use std::collections::{BTreeSet, BTreeMap};

use ordered_float::OrderedFloat;
use serde_bytes::{ByteBuf, Bytes};

use serde_fressian::INST::{INST};
use serde_fressian::UUID::{UUID};
use serde_fressian::URI::{URI};
use serde_fressian::REGEX::{REGEX};
use serde_fressian::SYM::{SYM};
use serde_fressian::KEY::{KEY};
use serde_fressian::typed_arrays::*;
use serde_fressian::SET::{SET};

use serde_fressian::value::{self, Value};
use serde_fressian::de::{self};
use serde_fressian::ser::{self};

#[test]
fn bytes_rt(){
    // (write (u8-array [0 1 2 126 127 128 253 254 255]))
    let control_bytes: Vec<u8> = vec![217,9,0,1,2,126,127,128,253,254,255];
    let control_slice: &[u8] = &[0,1,2,126,127,128,253,254,255];
    let control_bb = ByteBuf::from(control_slice);

    // strongly typed
    let test_bb: ByteBuf = de::from_vec(&control_bytes).unwrap();
    assert_eq!(control_bb, test_bb);
    let test_bytes: Vec<u8> = ser::to_vec(&test_bb).unwrap();
    assert_eq!(control_bytes, test_bytes);

    // VALUE
    let control_value = Value::BYTES(control_bb);
    let test_value: Value = de::from_vec(&control_bytes).unwrap();
    assert_eq!(test_value, control_value);
    let test_bytes: Vec<u8> = ser::to_vec(&test_value).unwrap();
    assert_eq!(control_bytes, test_bytes);
}

#[test]
fn set_rt(){
    // (write #{0 1 2 3})
    let control_bytes: Vec<u8> = vec![193,232,0,1,3,2];
    let control_set: BTreeSet<i64> = btreeset!{0,1,2,3};

    // strongly typed
    let test_set: BTreeSet<i64> = de::from_vec(&control_bytes).unwrap();
    assert_eq!(test_set, control_set);
    let s: SET<i64> = SET::from(control_set);
    let test_bytes: Vec<u8> = ser::to_vec(&s).unwrap();
    // sets write with nondet ordering so cannot compare bytes directly
    let derived_set: BTreeSet<i64> = de::from_vec(&test_bytes).unwrap();
    assert_eq!(*s, derived_set);

    // VALUE
    let control_set: BTreeSet<i64> = btreeset!{0,1,2,3};
    let control_value: Value = Value::from(control_set);
    let test_value: Value = de::from_vec(&test_bytes).unwrap();
    assert_eq!(control_value, test_value);
    let test_bytes: Vec<u8> = ser::to_vec(&test_value).unwrap();
    // sets write with nondet ordering so cannot compare bytes directly
    let derived_set_value: Value = de::from_vec(&test_bytes).unwrap();
    assert_eq!(control_value, derived_set_value);
}

#[test]
fn homogenous_map_rt(){

    // (write {"a" 0 "b" 1})
    let control_bytes: Vec<u8> = vec![192,232,219,97,0,219,98,1];
    let control_map: BTreeMap<String, i64> =
        btreemap!{
            "a".to_string() => 0,
            "b".to_string() => 1
        };

    // strongly typed
    let test_map: BTreeMap<String, i64> = de::from_vec(&control_bytes).unwrap();
    assert_eq!(control_map, test_map);
    let test_bytes: Vec<u8> = ser::to_vec(&test_map).unwrap();
    // maps write with nondet ordering so cannot compare bytes directly
    let derived_map: BTreeMap<String, i64> = de::from_vec(&test_bytes).unwrap();
    assert_eq!(control_map, derived_map);

    // VALUE
    let control_map_value: Value = Value::from(control_map);
    let test_map_value: Value = de::from_vec(&control_bytes).unwrap();
    assert_eq!(control_map_value, test_map_value);
    let test_bytes: Vec<u8> = ser::to_vec(&test_map_value).unwrap();
    let derived_map_value: Value = de::from_vec(&test_bytes).unwrap();
    assert_eq!(control_map_value, derived_map_value);

}

// need serde-with + type extraction