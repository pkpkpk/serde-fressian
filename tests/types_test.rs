#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(non_snake_case)]
#![feature(custom_attribute)]


#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_bytes;
extern crate serde_fressian;
extern crate uuid as _uuid;
extern crate url;
extern crate regex as _regex;
// extern crate chrono;

use std::collections::{HashMap, HashSet};
use serde::de::{Deserialize};
use serde::Serialize;


use serde_fressian::ser::{self, Serializer};
use serde_fressian::de::{self, Deserializer, from_vec};


#[test]
fn inst_test(){
    // use chrono::{ DateTime, Utc,};
    // use chrono::offset::{TimeZone, Offset};
    // use chrono::naive::{NaiveDateTime};
    use serde_fressian::inst::{INST};

    // // #inst "2018-08-13T02:20:05.875-00:00"
    // let value: Vec<u8> = vec![200,123,101,49,21,83,115];

    //eq #inst "2018-08-27T00:13:56.181-00:00"
    // // let f_rfc: DateTime<chrono::FixedOffset> = DateTime::parse_from_rfc3339("2018-08-27T00:13:56.181-00:00").unwrap();

    let i: INST = INST::from_millis(1535328836181);
    assert_eq!(i.to_millis(), 1535328836181)
}

#[test]
fn uuid_test(){
    use serde_fressian::uuid::{UUID};
    use _uuid::Uuid;

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
    use serde_fressian::uri::{URI};

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
    use _regex::Regex;
    use serde_fressian::regex::{REGEX};
    // "\n[abc]"
    let control_bytes: Vec<u8> = vec![196,225,92,110,91,97,98,99,93];

    let test_value: REGEX = serde_fressian::de::from_vec(&control_bytes).unwrap();
    assert_eq!(test_value.as_str() , r"\n[abc]");

    let control_value: Regex = Regex::new(r"\n[abc]").unwrap();
    let test_bytes: Vec<u8> = serde_fressian::ser::to_vec(&REGEX::from_Regex(control_value)).unwrap();
    assert_eq!(test_bytes, control_bytes);
    let rt_value: REGEX = serde_fressian::de::from_vec(&test_bytes).unwrap();
    assert_eq!(rt_value.as_str() , r"\n[abc]");
}

#[test]
fn sym_test(){
    use serde_fressian::sym::{SYM};

    // (api/write 'foo)
    let control_bytes: Vec<u8> = vec![201,247,205,221,102,111,111];
    let control_value: SYM = SYM::new(None, "foo".to_string());
    let test_value: SYM = serde_fressian::de::from_vec(&control_bytes).unwrap();
    assert_eq!(test_value, control_value);

    let test_bytes: Vec<u8> = serde_fressian::ser::to_vec(&control_value).unwrap();
    assert_eq!(test_bytes, control_bytes);

    assert_eq!(control_value,serde_fressian::de::from_vec(&test_bytes).unwrap());


    // (api/write 'foo/bar)
    let control_bytes: Vec<u8> = vec![201,205,221,102,111,111,205,221,98,97,114];
    let control_value: SYM = SYM::new(Some("foo".to_string()), "bar".to_string());

    let test_value: SYM = serde_fressian::de::from_vec(&control_bytes).unwrap();
    assert_eq!(test_value, control_value);

    let test_bytes: Vec<u8> = serde_fressian::ser::to_vec(&control_value).unwrap();
    assert_eq!(test_bytes, control_bytes);

    assert_eq!(control_value,serde_fressian::de::from_vec(&test_bytes).unwrap())
}

#[test]
fn key_test(){
    use serde_fressian::key::{KEY};

    // (api/write :foo)
    let control_bytes: Vec<u8> = vec![202,247,205,221,102,111,111];
    let control_value: KEY = KEY::new(None, "foo".to_string());
    let test_value: KEY = serde_fressian::de::from_vec(&control_bytes).unwrap();
    assert_eq!(test_value, control_value);

    let test_bytes: Vec<u8> = serde_fressian::ser::to_vec(&control_value).unwrap();
    assert_eq!(test_bytes, control_bytes);

    assert_eq!(control_value,serde_fressian::de::from_vec(&test_bytes).unwrap());


    // (api/write :foo/bar)
    let control_bytes: Vec<u8> = vec![202,205,221,102,111,111,205,221,98,97,114];
    let control_value: KEY = KEY::new(Some("foo".to_string()), "bar".to_string());

    let test_value: KEY = serde_fressian::de::from_vec(&control_bytes).unwrap();
    assert_eq!(test_value, control_value);

    let test_bytes: Vec<u8> = serde_fressian::ser::to_vec(&control_value).unwrap();
    assert_eq!(test_bytes, control_bytes);

    assert_eq!(control_value,serde_fressian::de::from_vec(&test_bytes).unwrap())
}

#[test]
fn typed_arrays_test(){
    use serde_fressian::typed_arrays::*;

    // (api/write (js/Int32Array. #js[1 2 3]))
    let control_bytes: Vec<u8> = vec![179,3,1,2,3];
    let v: Vec<i32> = vec![1,2,3];
    let control_value: Int_Array = Int_Array::from_vec(v);

    let test_value: Int_Array = serde_fressian::de::from_vec(&control_bytes).unwrap();
    assert_eq!(test_value, control_value);
    let test_bytes: Vec<u8> = serde_fressian::ser::to_vec(&control_value).unwrap();
    assert_eq!(test_bytes, control_bytes);
    assert_eq!(control_value,serde_fressian::de::from_vec(&test_bytes).unwrap());


    // (js/Float64Array. #js[-2 -1 0 1 2])
    let control_bytes: Vec<u8> = vec![177,5,250,192,0,0,0,0,0,0,0,250,191,240,0,0,0,0,0,0,251,252,250,64,0,0,0,0,0,0,0];
    let v: Vec<f64> = vec![-2.0, -1.0, 0.0, 1.0, 2.0];
    let control_value: Double_Array = Double_Array::from_vec(v);

    let test_value: Double_Array = serde_fressian::de::from_vec(&control_bytes).unwrap();
    assert_eq!(test_value, control_value);
    let test_bytes: Vec<u8> = serde_fressian::ser::to_vec(&control_value).unwrap();
    assert_eq!(test_bytes, control_bytes);
    assert_eq!(control_value,serde_fressian::de::from_vec(&test_bytes).unwrap());

    // (fress.writer/writeAs w "boolean[]" [true false true false false])
    let control_bytes: Vec<u8> = vec![178,5,245,246,245,246,246];
    let v: Vec<bool>= vec![true,false,true,false,false];
    let control_value: Boolean_Array = Boolean_Array::from_vec(v);

    let test_value: Boolean_Array = serde_fressian::de::from_vec(&control_bytes).unwrap();
    assert_eq!(test_value, control_value);
    let test_bytes: Vec<u8> = serde_fressian::ser::to_vec(&control_value).unwrap();
    assert_eq!(test_bytes, control_bytes);
    assert_eq!(control_value,serde_fressian::de::from_vec(&test_bytes).unwrap());
}
//f32
//i64
//need serde 'with' attribute tests