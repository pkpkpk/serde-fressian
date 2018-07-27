#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(non_snake_case)]
#![feature(custom_attribute)]


#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_bytes;

extern crate serde_fressian;

use std::collections::HashMap;
use serde::Serialize;

use serde_fressian::ser::{Serializer};

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

#[test] //Float/MIN_VALUE
fn test_ser_min_f32 (){
    let mut fw = Serializer::new(Vec::new());

    let v: f32 = 1.4E-45;
    let control: Vec<u8> = vec![249, 0, 0, 0, 1];
    fw.write_float(v).unwrap();
    let buf = fw.to_vec();
    assert_eq!(&buf, &control);
    fw.reset();
    v.serialize(&mut fw).unwrap();
    let buf = fw.to_vec();
    assert_eq!(&buf, &control);
}

#[test] // Float/MAX_VALUE
fn test_ser_max_f32 (){
    let mut fw = Serializer::new(Vec::new());

    let v: f32 = 3.4028235E38;
    let control: Vec<u8> = vec![249, 127, 127, 255, 255];
    fw.write_float(v).unwrap();

    let buf = fw.to_vec();
    assert_eq!(&buf, &control);
    fw.reset();
    v.serialize(&mut fw).unwrap();
    let buf = fw.to_vec();
    assert_eq!(&buf, &control);
}

#[test] // DOUBLE/MIN_VALUE
fn test_ser_min_f64 (){
    let mut fw = Serializer::new(Vec::new());

    let v: f64 = 4.9E-324;
    let control: Vec<u8> = vec![250, 0, 0, 0, 0, 0, 0, 0, 1];
    fw.write_double(v).unwrap();

    let buf = fw.to_vec();
    assert_eq!(&buf, &control);
    fw.reset();
    v.serialize(&mut fw).unwrap();
    let buf = fw.to_vec();
    assert_eq!(&buf, &control);
}

#[test] // DOUBLE/MAX_VALUE
fn test_ser_max_f64 (){
    let mut fw = Serializer::new(Vec::new());

    let v: f64 = 1.7976931348623157E308;
    let control: Vec<u8> = vec![250, 127, 239, 255, 255, 255, 255, 255, 255 ];
    fw.write_double(v).unwrap();
    let buf = fw.to_vec();
    assert_eq!(&buf, &control);
    fw.reset();
    v.serialize(&mut fw).unwrap();
    let buf = fw.to_vec();
    assert_eq!(&buf, &control);
}

#[test]
fn ints_test (){
    let mut fw = Serializer::new(Vec::new());
    //Short/MIN_VALUE
    let v: i16 = -32768;
    let control: Vec<u8> = vec![103, 128, 0];
    fw.write_int(v as i64).unwrap();
    let buf = fw.to_vec();
    assert_eq!(&buf, &control);
    fw.reset();
    v.serialize(&mut fw).unwrap();
    let buf = fw.to_vec();
    assert_eq!(&buf, &control);

    //Short/MAX_VALUE
    let v: i16 = 32767;
    let control: Vec<u8> = vec![104, 127, 255];
    fw.reset();
    fw.write_int(v as i64).unwrap();
    let buf = fw.to_vec();
    assert_eq!(&buf, &control);
    fw.reset();
    v.serialize(&mut fw).unwrap();
    let buf = fw.to_vec();
    assert_eq!(&buf, &control);

    //Integer/MIN_VALUE
    let v: i32 = -2147483648;
    let control: Vec<u8> = vec![117, 128, 0, 0, 0];
    fw.reset();
    fw.write_int(v as i64).unwrap();
    let buf = fw.to_vec();
    assert_eq!(&buf, &control);
    fw.reset();
    v.serialize(&mut fw).unwrap();
    let buf = fw.to_vec();
    assert_eq!(&buf, &control);

    //Integer/MAX_VALUE
    let v: i32 = 2147483647;
    let control: Vec<u8> = vec![118, 127, 255, 255, 255];
    fw.reset();
    fw.write_int(v as i64).unwrap();
    let buf = fw.to_vec();
    assert_eq!(&buf, &control);
    fw.reset();
    v.serialize(&mut fw).unwrap();
    let buf = fw.to_vec();
    assert_eq!(&buf, &control);

    // min i40
    let v: i64 = -549755813887;
    let control: Vec<u8> = vec![121, 128, 0, 0, 0, 1];
    fw.reset();
    fw.write_int(v as i64).unwrap();
    let buf = fw.to_vec();
    assert_eq!(&buf, &control);
    fw.reset();
    v.serialize(&mut fw).unwrap();
    let buf = fw.to_vec();
    assert_eq!(&buf, &control);

    // max i40
    let v: i64 = 549755813888;
    let control: Vec<u8> = vec![122, 128, 0, 0, 0, 0];
    fw.reset();
    fw.write_int(v as i64).unwrap();
    let buf = fw.to_vec();
    assert_eq!(&buf, &control);
    fw.reset();
    v.serialize(&mut fw).unwrap();
    let buf = fw.to_vec();
    assert_eq!(&buf, &control);

    // max i48
    let v: i64 = 140737490000000;
    let control: Vec<u8> = vec![126, 128, 0, 0, 25, 24, 128];
    fw.reset();
    fw.write_int(v as i64).unwrap();
    let buf = fw.to_vec();
    assert_eq!(&buf, &control);
    fw.reset();
    v.serialize(&mut fw).unwrap();
    let buf = fw.to_vec();
    assert_eq!(&buf, &control);

    // JS_MAX_SAFE_INT
    let v: i64 = 9007199254740991;
    let control: Vec<u8> = vec![248, 0, 31, 255, 255, 255, 255, 255, 255];
    fw.reset();
    fw.write_int(v as i64).unwrap();
    let buf = fw.to_vec();
    assert_eq!(&buf, &control);
    fw.reset();
    v.serialize(&mut fw).unwrap();
    let buf = fw.to_vec();
    assert_eq!(&buf, &control);

    // JS_MAX_SAFE_INT++
    let v: i64 = 9007199254740992;
    let control: Vec<u8> = vec![248, 0, 32, 0, 0, 0, 0, 0, 0];
    fw.reset();
    fw.write_int(v as i64).unwrap();
    let buf = fw.to_vec();
    assert_eq!(&buf, &control);
    fw.reset();
    v.serialize(&mut fw).unwrap();
    let buf = fw.to_vec();
    assert_eq!(&buf, &control);

    // JS_MIN_SAFE_INT
    let v: i64 = -9007199254740991;
    let control: Vec<u8> = vec![248, 255, 224, 0, 0, 0, 0, 0, 1];
    fw.reset();
    fw.write_int(v as i64).unwrap();
    let buf = fw.to_vec();
    assert_eq!(&buf, &control);
    fw.reset();
    v.serialize(&mut fw).unwrap();
    let buf = fw.to_vec();
    assert_eq!(&buf, &control);

    // JS_MIN_SAFE_INT--
    let v: i64 = -9007199254740992;
    let control: Vec<u8> = vec![248, 255, 224, 0, 0, 0, 0, 0, 0];
    fw.reset();
    fw.write_int(v as i64).unwrap();
    let buf = fw.to_vec();
    assert_eq!(&buf, &control);
    fw.reset();
    v.serialize(&mut fw).unwrap();
    let buf = fw.to_vec();
    assert_eq!(&buf, &control);

    // long max (i64)
    let v: i64 = 9223372036854775807;
    let control: Vec<u8> = vec![248, 127, 255, 255, 255, 255, 255, 255, 255];
    fw.reset();
    fw.write_int(v as i64).unwrap();
    let buf = fw.to_vec();
    assert_eq!(&buf, &control);
    fw.reset();
    v.serialize(&mut fw).unwrap();
    let buf = fw.to_vec();
    assert_eq!(&buf, &control);

    // long min (i64)
    let v: i64 = -9223372036854775808;
    let control: Vec<u8> = vec![248, 128, 0, 0, 0, 0, 0, 0, 0];
    fw.reset();
    fw.write_int(v as i64).unwrap();
    let buf = fw.to_vec();
    assert_eq!(&buf, &control);
    fw.reset();
    v.serialize(&mut fw).unwrap();
    let buf = fw.to_vec();
    assert_eq!(&buf, &control);

}

#[test]
fn bytes_test(){
    let mut fw = Serializer::new(Vec::new());

    // packed count
    let v: Vec<u8> = vec![255,254,253,0,1,2,3];
    let control: Vec<u8> = vec![215,255,254,253,0,1,2,3];
    fw.write_bytes(&v, 0, v.len()).unwrap();
    let buf = fw.to_vec();
    assert_eq!(&buf, &control);

    // can't override built in serialize impl for vec<u8>
    // https://github.com/rust-lang/rust/issues/31844
    // soln -> https://docs.serde.rs/serde_bytes/
    let bb = serde_bytes::ByteBuf::from(v);
    fw.reset();
    bb.serialize(&mut fw).unwrap();
    let buf = fw.to_vec();
    assert_eq!(&buf, &control);

    fw.reset();
    // unpacked length
    let v: Vec<u8> = vec![252,253,254,255,0,1,2,3,4];
    let control: Vec<u8> = vec![217, 9, 252, 253, 254, 255, 0, 1, 2, 3, 4];
    fw.write_bytes(&v, 0, v.len()).unwrap();
    let buf = fw.to_vec();
    assert_eq!(&buf, &control);

}

#[test]
fn string_test(){
    let mut fw = Serializer::new(Vec::new());

    let v = "".to_string();
    #[cfg(not(raw_UTF8))]
    let control: Vec<u8> = vec![218];
    #[cfg(raw_UTF8)]
    let control: Vec<u8> = vec![191,0];
    fw.write_string(&v).unwrap();
    let buf = fw.to_vec();
    assert_eq!(&buf, &control);
    fw.reset();
    &v.serialize(&mut fw);
    let buf = fw.to_vec();
    assert_eq!(&buf, &control);

    fw.reset();

    let v = "hola".to_string();
    #[cfg(not(raw_UTF8))]
    let control: Vec<u8> = vec![222,104,111,108,97];
    #[cfg(raw_UTF8)]
    let control: Vec<u8> = vec![191,4,104,111,108,97];
    fw.write_string(&v).unwrap();
    let buf = fw.to_vec();
    assert_eq!(&buf, &control);
    fw.reset();
    &v.serialize(&mut fw);
    let buf = fw.to_vec();
    assert_eq!(&buf, &control);

    fw.reset();
    let v = "é❤️ßℝ東京東京😉 😎 🤔 😐 🙄".to_string();
    #[cfg(not(raw_UTF8))]
    let control: Vec<u8> = vec![227,60,101,204,129,226,157,164,239,184,143,195,159,226,132,157,230,157,177,228,186,172,230,157,177,228,186,172,237,160,189,237,184,137,32,237,160,189,237,184,142,32,237,160,190,237,180,148,32,237,160,189,237,184,144,32,237,160,189,237,185,132];
    #[cfg(raw_UTF8)]
    let control: Vec<u8> = vec![191,50,101,204,129,226,157,164,239,184,143,195,159,226,132,157,230,157,177,228,186,172,230,157,177,228,186,172,240,159,152,137,32,240,159,152,142,32,240,159,164,148,32,240,159,152,144,32,240,159,153,132];
    fw.write_string(&v).unwrap();
    let buf = fw.to_vec();
    assert_eq!(&buf, &control);
    fw.reset();
    &v.serialize(&mut fw);
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

