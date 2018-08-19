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


#[test]
fn inst_test(){
    // use chrono::{ DateTime, Utc,};
    use serde_fressian::INST::{INST};
    // #inst "2018-08-13T02:20:05.875-00:00"
    let value: Vec<u8> = vec![200,123,101,49,21,83,115];
    let control_str = "2018-08-13T02:20:05";
    let dt: INST = de::from_vec(&value).unwrap();
    // assert_eq!(dt.to_string(), "2018-08-13T02:20:05.875-00:00");
    // couldnt figure out fff in "yyyy-mm-ddThh:mm:ss.fff+hh:mm"
    assert_eq!(dt.format("%Y-%m-%dT%H:%M:%S").to_string(), control_str);
    // rt
    let dt: INST = INST::from_millis(1534126805875);
    let written_bytes = ser::to_vec(&dt).unwrap();
    let i: INST = de::from_vec(&written_bytes).unwrap();
    assert_eq!(i.format("%Y-%m-%dT%H:%M:%S").to_string(), "2018-08-13T02:20:05");
}

#[test]
fn uuid_test(){
    use serde_fressian::UUID::{UUID};
    use uuid::Uuid;

    // #uuid "c8bf117b-8ee4-4e74-8c1f-51df0a757fe8"
    let control_value =  Uuid::parse_str("c8bf117b-8ee4-4e74-8c1f-51df0a757fe8").unwrap();
    let control_bytes: Vec<u8> = vec![195,217,16,200,191,17,123,142,228,78,116,140,31,81,223,10,117,127,232];

    let test_value: UUID = serde_fressian::de::from_vec(&control_bytes).unwrap();
    assert_eq!(*test_value, control_value);

    let mut fw = Serializer::from_vec(Vec::new());
    UUID::from_Uuid(control_value).serialize(&mut fw).unwrap();
    let buf = fw.to_vec();
    assert_eq!(buf, control_bytes);
    let test_value: UUID = serde_fressian::de::from_vec(&buf).unwrap();
    assert_eq!(*test_value, control_value);
}

#[test]
fn uri_test(){
    use url::{Url, Host};
    use serde_fressian::URI::{URI};

    // "https://www.youtube.com/watch?v=xvhQitzj0zQ"
    let control_bytes: Vec<u8> = vec![197,227,43,104,116,116,112,115,58,47,47,119,119,119,46,121,111,117,116,117,98,101,46,99,111,109,47,119,97,116,99,104,63,118,61,120,118,104,81,105,116,122,106,48,122,81];
    let control_value: Url = Url::parse("https://www.youtube.com/watch?v=xvhQitzj0zQ").unwrap();

    let test_value: URI = serde_fressian::de::from_vec(&control_bytes).unwrap();
    assert_eq!(*test_value, control_value);

    let mut fw = Serializer::from_vec(Vec::new());
    URI::from_Url(control_value.clone()).serialize(&mut fw).unwrap();
    let buf = fw.to_vec();
    assert_eq!(buf, control_bytes);
    let test_value: URI = serde_fressian::de::from_vec(&buf).unwrap();
    assert_eq!(test_value.as_str(), control_value.as_str());
    assert_eq!(test_value.as_str(), "https://www.youtube.com/watch?v=xvhQitzj0zQ");
}

#[test]
fn regex_test(){
    use regex::Regex;
    use serde_fressian::REGEX::{REGEX};
    // "\n[abc]"
    let control_bytes: Vec<u8> = vec![196,225,92,110,91,97,98,99,93];

    let test_value: REGEX = serde_fressian::de::from_vec(&control_bytes).unwrap();
    assert_eq!(test_value.as_str() , r"\n[abc]");

    let control_value: Regex = Regex::new(r"\n[abc]").unwrap();
    let test_bytes: Vec<u8> = serde_fressian::ser::to_vec(&REGEX::from_Regex(control_value)).unwrap();
    assert_eq!(test_bytes, control_bytes);
    let rt_value: REGEX = serde_fressian::de::from_vec(&test_bytes).unwrap();
    assert_eq!(test_value.as_str() , r"\n[abc]");
}

#[test]
fn sym_test(){
    use serde_fressian::SYM::{SYM};

    // (api/write 'foo/bar)
    let control_bytes: Vec<u8> = vec![201,205,221,102,111,111,205,221,98,97,114];
    let control_value: SYM = SYM::new("foo".to_string(), "bar".to_string());

    let test_value: SYM = serde_fressian::de::from_vec(&control_bytes).unwrap();
    assert_eq!(test_value, control_value);

    let test_bytes: Vec<u8> = serde_fressian::ser::to_vec(&control_value).unwrap();
    assert_eq!(test_bytes, control_bytes);

    assert_eq!(control_value,serde_fressian::de::from_vec(&test_bytes).unwrap())
}