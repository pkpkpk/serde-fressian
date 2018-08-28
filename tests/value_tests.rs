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

use ordered_float::OrderedFloat;
use serde_bytes::ByteBuf;

use serde_fressian::INST::{INST};
use serde_fressian::UUID::{UUID};
use serde_fressian::URI::{URI};
use serde_fressian::REGEX::{REGEX};
use serde_fressian::SYM::{SYM};
use serde_fressian::KEY::{KEY};
use serde_fressian::typed_arrays::*;

use serde_fressian::value::{self, Value};
use serde_fressian::de::{self};
use serde_fressian::ser::{self};

#[test]
fn bool_value_rt(){
    // (write true)
    let control_bytes: Vec<u8> = vec![245];
    let b: bool = de::from_vec(&control_bytes).unwrap();
    assert_eq!(b,true);

    let test_value: Value = de::from_vec(&control_bytes).unwrap();
    assert_eq!(test_value, Value::BOOL(true));
    let test_bytes: Vec<u8> = ser::to_vec(&test_value).unwrap();
    assert_eq!(control_bytes, test_bytes);
}

#[test]
fn float_value_rt(){
    // (write 32.2)
    let control_bytes: Vec<u8> = vec![249,66,0,204,205];
    let f: f32 = de::from_vec(&control_bytes).unwrap();
    assert_eq!(f, 32.2);
    let test_value: Value = de::from_vec(&control_bytes).unwrap();
    assert_eq!(test_value, Value::FLOAT(OrderedFloat::from(32.2)));
    let test_bytes: Vec<u8> = ser::to_vec(&test_value).unwrap();
    assert_eq!(control_bytes, test_bytes);
}

#[test]
fn double_value_rt(){
    // (fress.writer/writeDouble w 64.4)
    let control_bytes: Vec<u8> = vec![250,64,80,25,153,153,153,153,154];
    let d: f64 = de::from_vec(&control_bytes).unwrap();
    assert_eq!(d, 64.4 as f64);
    let test_value: Value = de::from_vec(&control_bytes).unwrap();
    assert_eq!(test_value, Value::DOUBLE(OrderedFloat::from(d)));
    let test_bytes: Vec<u8> = ser::to_vec(&test_value).unwrap();
    assert_eq!(control_bytes, test_bytes);
}

#[test]
fn string_value_rt(){
    // (write "foo")
    let control_bytes: Vec<u8> = vec![221,102,111,111];
    let control_value: String = "foo".to_string();
    let s: String = de::from_vec(&control_bytes).unwrap();
    assert_eq!(s, control_value);
    let test_value: Value = de::from_vec(&control_bytes).unwrap();
    assert_eq!(test_value, Value::STRING(control_value));
    let test_bytes: Vec<u8> = ser::to_vec(&test_value).unwrap();
    assert_eq!(control_bytes, test_bytes);
}

#[test]
fn named_value_rt(){
    // (write :foo)
    let control_bytes: Vec<u8> = vec![202,247,205,221,102,111,111];
    let control_value = KEY::new(None,"foo".to_string());
    let k: KEY = de::from_vec(&control_bytes).unwrap();
    assert_eq!(k, control_value);
    let test_value: Value = de::from_vec(&control_bytes).unwrap();
    assert_eq!(test_value, Value::KEY(control_value));
    let test_bytes: Vec<u8> = ser::to_vec(&test_value).unwrap();
    assert_eq!(control_bytes, test_bytes);

    // (write :foo/bar)
    let control_bytes: Vec<u8> = vec![202,205,221,102,111,111,205,221,98,97,114];
    let control_value = KEY::new(Some("foo".to_string()), "bar".to_string());
    let k: KEY = de::from_vec(&control_bytes).unwrap();
    assert_eq!(k, control_value);
    let test_value: Value = de::from_vec(&control_bytes).unwrap();
    assert_eq!(test_value, Value::KEY(control_value));
    let test_bytes: Vec<u8> = ser::to_vec(&test_value).unwrap();
    assert_eq!(control_bytes, test_bytes);

    // (write ['foo 'bar/baz])
    let control_bytes: Vec<u8> = vec![230,201,247,205,221,102,111,111,201,205,221,98,97,114,205,221,98,97,122];
    let control_value: Vec<SYM> = vec![SYM::new(None,"foo".to_string()), SYM::new(Some("bar".to_string()),"baz".to_string()) ];
    let v: Vec<SYM> = de::from_vec(&control_bytes).unwrap();
    assert_eq!(v, control_value);
    let test_value: Value = de::from_vec(&control_bytes).unwrap();
    assert_eq!(test_value, Value::LIST(vec![Value::SYM(SYM::new(None,"foo".to_string())), Value::SYM(SYM::new(Some("bar".to_string()),"baz".to_string()))]));
    let test_bytes: Vec<u8> = ser::to_vec(&test_value).unwrap();
    assert_eq!(control_bytes, test_bytes);
}


#[test]
fn misc_value_test(){
    //Vec<string>
    // (write ["what" "will" "grow" "crooked"])
    let control_bytes: Vec<u8> = vec![232,222,119,104,97,116,222,119,105,108,108,222,103,114,111,119,225,99,114,111,111,107,101,100];
    let control_value: Vec<String> = vec!["what".to_string(), "will".to_string(), "grow".to_string(), "crooked".to_string()];
    let v: Vec<String> = de::from_vec(&control_bytes).unwrap();
    assert_eq!(v, control_value);
    // completely untyped
    let test_value: Value = de::from_vec(&control_bytes).unwrap();
    assert_eq!(test_value, Value::LIST(vec![Value::STRING("what".to_string()), Value::STRING("will".to_string()), Value::STRING("grow".to_string()), Value::STRING("crooked".to_string())]));
    let test_bytes: Vec<u8> = ser::to_vec(&test_value).unwrap();
    assert_eq!(control_bytes, test_bytes);
    // know its a vec
    let test_value: Vec<Value> = de::from_vec(&control_bytes).unwrap();
    assert_eq!(test_value, vec![Value::STRING("what".to_string()), Value::STRING("will".to_string()), Value::STRING("grow".to_string()), Value::STRING("crooked".to_string())]);
    let test_bytes: Vec<u8> = ser::to_vec(&test_value).unwrap();
    assert_eq!(control_bytes, test_bytes);

    // Vec<ints>
    // (write  [-1 64 65 1024])
    let control_bytes: Vec<u8> = vec![232,255,80,64,80,65,84,0];
    let control_value: Vec<i64> = vec![-1,64,65,1024];
    let v: Vec<i64> = de::from_vec(&control_bytes).unwrap();
    assert_eq!(v, control_value);
    let test_value: Value = de::from_vec(&control_bytes).unwrap();
    assert_eq!(test_value, Value::LIST(vec![Value::INT(-1), Value::INT(64), Value::INT(65), Value::INT(1024)]));
    let test_bytes: Vec<u8> = ser::to_vec(&test_value).unwrap();
    assert_eq!(control_bytes, test_bytes);

    // float-array
    // (write (fress.util/f32-array [-2 -1 0 1 2]))
    let control_bytes: Vec<u8> = vec![180,5,249,192,0,0,0,249,191,128,0,0,249,0,0,0,0,249,63,128,0,0,249,64,0,0,0];
    let v: Float_Array = Float_Array::from(vec![-2.0, -1.0, 0.0, 1.0, 2.0]);
    let control_value = Value::from(v);
    let test_value: Value = de::from_vec(&control_bytes).unwrap();
    assert_eq!(test_value, control_value);
    let test_bytes: Vec<u8> = ser::to_vec(&test_value).unwrap();
    assert_eq!(control_bytes, test_bytes);
}


#[test]
fn hetero_coll_rt(){
    // (write [ #inst "2018-08-27T00:13:56.181-00:00",  #"\n", #uuid "9d046b06-f24e-4301-a266-8b80783e0f00", (goog.Uri. "https://www.youtube.com/watch?v=Z1nFB-R-_gI") ])
    let control_bytes: Vec<u8> = vec![232,200,123,101,120,186,218,85,196,220,92,110,195,217,16,157,4,107,6,242,78,67,1,162,102,139,128,120,62,15,0,197,227,43,104,116,116,112,115,58,47,47,119,119,119,46,121,111,117,116,117,98,101,46,99,111,109,47,119,97,116,99,104,63,118,61,90,49,110,70,66,45,82,45,95,103,73];
    let date: INST = INST::from_millis(1535328836181);
    let re: REGEX = REGEX::from_str(r"\n").unwrap();
    let u: UUID = UUID::from_bytes(&[157,4,107,6,  242,78,67,1,  162,102,139,128,  120,62,15,0]).unwrap();
    let uri: URI = URI::from_str("https://www.youtube.com/watch?v=Z1nFB-R-_gI").unwrap();

    let control_value: Vec<Value> = vec![ Value::from(date), Value::from(re), Value::from(u), Value::from(uri)];
    let test_value: Vec<Value> = de::from_vec(&control_bytes).unwrap();
    assert_eq!(test_value, control_value);
    let test_bytes: Vec<u8> = ser::to_vec(&test_value).unwrap();
    assert_eq!(control_bytes, test_bytes);
}

#[test]
fn map_value_rt(){
    // (write {:foo 42, "baz" [1 2 3]})
    let control_bytes: Vec<u8> = vec![192,232,202,247,205,221,102,111,111,42,221,98,97,122,231,1,2,3];
    let k0 = Value::KEY(KEY::new(None,"foo".to_string()));
    let v0 = Value::INT(42);
    let k1 = Value::STRING("baz".to_string());
    let v1 = Value::LIST(vec![Value::INT(1), Value::INT(2), Value::INT(3) ]);
    let mut control_map: BTreeMap<Value,Value> = BTreeMap::new();
    control_map.insert(k0,v0);
    control_map.insert(k1,v1);
    let control_value = Value::MAP(control_map);
    let test_value: Value = de::from_vec(&control_bytes).unwrap();
    assert_eq!(test_value, control_value);
    let test_bytes: Vec<u8> = ser::to_vec(&test_value).unwrap();
    let derived_value: Value = de::from_vec(&test_bytes).unwrap();
    assert_eq!(control_value, derived_value);
}
