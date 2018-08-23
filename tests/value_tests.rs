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

use std::collections::{BTreeSet, BTreeMap};
use serde::de::{Deserialize};
use serde::Serialize;

use serde_fressian::ser::{self, Serializer, FressianWriter};
use serde_fressian::de::{self, Deserializer, from_vec};


use serde_fressian::INST::{INST};
use serde_fressian::UUID::{UUID};
use serde_fressian::URI::{URI};
use serde_fressian::REGEX::{REGEX};
use serde_fressian::SYM::{SYM};
use serde_fressian::KEY::{KEY};
use serde_fressian::typed_arrays::*;

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

    // (write "foo")
    let control_bytes: Vec<u8> = vec![221,102,111,111];
    let control_value: String = "foo".to_string();
    let s: String = de::from_vec(&control_bytes).unwrap();
    assert_eq!(s, control_value);
    let val: Value = de::from_vec(&control_bytes).unwrap();
    assert_eq!(val, Value::STRING(control_value));

    // (write ["what" "will" "grow" "crooked"])
    let control_bytes: Vec<u8> = vec![232,222,119,104,97,116,222,119,105,108,108,222,103,114,111,119,225,99,114,111,111,107,101,100];
    let control_value: Vec<String> = vec!["what".to_string(), "will".to_string(), "grow".to_string(), "crooked".to_string()];
    let v: Vec<String> = de::from_vec(&control_bytes).unwrap();
    assert_eq!(v, control_value);
    // completely untyped
    let val: Value = de::from_vec(&control_bytes).unwrap();
    assert_eq!(val, Value::LIST(vec![Value::STRING("what".to_string()), Value::STRING("will".to_string()), Value::STRING("grow".to_string()), Value::STRING("crooked".to_string())]));
    // know its a vec
    let val: Vec<Value> = de::from_vec(&control_bytes).unwrap();
    assert_eq!(val, vec![Value::STRING("what".to_string()), Value::STRING("will".to_string()), Value::STRING("grow".to_string()), Value::STRING("crooked".to_string())]);

    // (write {:foo 42, "baz" [1 2 3]})
    let control_bytes: Vec<u8> = vec![192,232,202,247,205,221,102,111,111,42,221,98,97,122,231,1,2,3];
    let k0 = Value::KEY(KEY::new(None,"foo".to_string()));
    let v0 = Value::INT(42);
    let k1 = Value::STRING("baz".to_string());
    let v1 = Value::LIST(vec![Value::INT(1), Value::INT(2), Value::INT(3) ]);
    let mut control_map: BTreeMap<Value,Value> = BTreeMap::new();
    control_map.insert(k0,v0);
    control_map.insert(k1,v1);
    let test_val: Value = de::from_vec(&control_bytes).unwrap();
    // assert_eq!(test_val, Value::MAP(control_map))
    // assert_eq!(val, Value::MAP(Value::STRING("what".to_string()), Value::STRING("will".to_string()), Value::STRING("grow".to_string()), Value::STRING("crooked".to_string())));

}