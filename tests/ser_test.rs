#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(non_snake_case)]
#![feature(custom_attribute)]


#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_bytes;

extern crate serde_fressian;

use std::collections::{HashMap, HashSet};
use serde::Serialize;

use serde_fressian::ser::{Serializer};
// use serde_fressian::de::{Deserializer};

#[test]
fn test_reset(){
    let mut fw = Serializer::new(Vec::new());

    let v: Vec<i64> = vec![-2, -1, 0, 1, 2];
    let control: Vec<u8> = vec![233,79,254,255,0,1,2];
    fw.write_list(&v).unwrap();
    let buf = fw.to_vec();
    assert_eq!(&buf, &control);
    fw.reset();
    let control: Vec<u8> = vec![];
    let buf = fw.to_vec();
    assert_eq!(&buf, &control);
}

#[test]
fn list_test(){
    let mut fw = Serializer::new(Vec::new());

    let v: Vec<i64> = vec![-2, -1, 0, 1, 2];
    let control: Vec<u8> = vec![233,79,254,255,0,1,2];
    fw.write_list(&v).unwrap();
    let buf = fw.to_vec();
    assert_eq!(&buf, &control);
    fw.reset();
    &v.serialize(&mut fw);
    let buf = fw.to_vec();
    assert_eq!(&buf, &control);

    fw.reset();

    let v: Vec<String> = vec!["i".to_string(), "am".to_string(), "a".to_string(), "reasonable".to_string(), "man".to_string(), "get".to_string(), "off".to_string(), "my".to_string(), "case".to_string()];
    #[cfg(not(raw_UTF8))]
    let control: Vec<u8> = vec![236,9,219,105,220,97,109,219,97,227,10,114,101,97,115,111,110,97,98,108,101,221,109,97,110,221,103,101,116,221,111,102,102,220,109,121,222,99,97,115,101];
    #[cfg(raw_UTF8)]
    let control: Vec<u8> = vec![236,9,191,1,105,191,2,97,109,191,1,97,191,10,114,101,97,115,111,110,97,98,108,101,191,3,109,97,110,191,3,103,101,116,191,3,111,102,102,191,2,109,121,191,4,99,97,115,101];

    fw.write_list(&v).unwrap();
    let buf = fw.to_vec();
    assert_eq!(&buf, &control);
    fw.reset();
    &v.serialize(&mut fw);
    let buf = fw.to_vec();
    assert_eq!(&buf, &control);

    fw.reset();

    let sub_v0: Vec<String> = vec!["some".to_string()];
    let sub_v1: Vec<String> = vec!["nested".to_string()];
    let sub_v2: Vec<String> = vec!["shit".to_string()];
    let v: Vec<Vec<String>> = vec![sub_v0, sub_v1, sub_v2];
    #[cfg(not(raw_UTF8))]
    let control: Vec<u8> = vec![231,229,222,115,111,109,101,229,224,110,101,115,116,101,100,229,222,115,104,105,116];
    #[cfg(raw_UTF8)]
    let control: Vec<u8> = vec![231,229,191,4,115,111,109,101,229,191,6,110,101,115,116,101,100,229,191,4,115,104,105,116];
    fw.write_list(&v).unwrap();
    let buf = fw.to_vec();
    assert_eq!(&buf, &control);

    fw.reset();
    &v.serialize(&mut fw);
    let buf = fw.to_vec();
    assert_eq!(&buf, &control);
}


fn read_count(bytes: Vec<u8>, offset: usize){

}

//set iter has notdet ordering
fn map_bytes_eq(a: Vec<u8>, b:Vec<u8>) {

    assert_eq!(&a[0], &b[0]);

    // check map flag
    // check count
    // pair off rest as map entry tuples, put in sets and compare
}

#[test]
fn map_test() {
    let mut fw = Serializer::new(Vec::new());

    let mut m: HashMap<String, u8> = HashMap::new();
    m.insert("a".to_string(), 0);
    #[cfg(not(raw_UTF8))]
    let control: Vec<u8> = vec![192,230,219,97,0];
    #[cfg(raw_UTF8)]
    let control: Vec<u8> = vec![192, 230, 191, 1, 97, 0];
    fw.write_map(&m).unwrap();
    let buf = fw.to_vec();
    assert_eq!(&buf, &control);

    fw.reset();

    // let mut m: HashMap<String, u8> = HashMap::new();
    // m.insert("a".to_string(), 0);
    // m.insert("b".to_string(), 1);
    // #[cfg(not(raw_UTF8))]
    // let control: Vec<u8> = vec![192,232,219,97,0,219,98,1];
    // #[cfg(raw_UTF8)]
    // let control: Vec<u8> = vec![192, 232, 191, 1, 97, 0, 191, 1, 98, 1];
    // fw.write_map(&m).unwrap();
    // let buf = fw.to_vec();
    // assert_eq!(&buf, &control);

}

