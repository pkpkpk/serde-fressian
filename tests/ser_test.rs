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



#[test] //Float/MIN_VALUE
fn test_ser_min_f32 (){
    let buf: Vec<u8>  = Vec::new();
    let mut FW = Serializer::new(buf);

    let v: f32 = 1.4E-45;
    let control: Vec<u8> = vec![249, 0, 0, 0, 1];
    FW.write_float(v).unwrap();

    assert_eq!(FW.get_ref(), &control);
    FW.reset();
    v.serialize(&mut FW).unwrap();
    assert_eq!(FW.get_ref(), &control);
}

#[test] // Float/MAX_VALUE
fn test_ser_max_f32 (){
    let buf: Vec<u8>  = Vec::new();
    let mut FW = Serializer::new(buf);

    let v: f32 = 3.4028235E38;
    let control: Vec<u8> = vec![249, 127, 127, 255, 255];
    FW.write_float(v).unwrap();

    assert_eq!(FW.get_ref(), &control);
    FW.reset();
    v.serialize(&mut FW).unwrap();
    assert_eq!(FW.get_ref(), &control);
}

#[test] // DOUBLE/MIN_VALUE
fn test_ser_min_f64 (){
    let buf: Vec<u8>  = Vec::new();
    let mut FW = Serializer::new(buf);

    let v: f64 = 4.9E-324;
    let control: Vec<u8> = vec![250, 0, 0, 0, 0, 0, 0, 0, 1];
    FW.write_double(v).unwrap();

    assert_eq!(FW.get_ref(), &control);
    FW.reset();
    v.serialize(&mut FW).unwrap();
    assert_eq!(FW.get_ref(), &control);
}

#[test] // DOUBLE/MAX_VALUE
fn test_ser_max_f64 (){
    let buf: Vec<u8>  = Vec::new();
    let mut FW = Serializer::new(buf);

    let v: f64 = 1.7976931348623157E308;
    let control: Vec<u8> = vec![250, 127, 239, 255, 255, 255, 255, 255, 255 ];
    FW.write_double(v).unwrap();

    assert_eq!(FW.get_ref(), &control );
    FW.reset();
    v.serialize(&mut FW).unwrap();
    assert_eq!(FW.get_ref(), &control);
}

#[test]
fn ints_test (){

    //Short/MIN_VALUE
    let v: i16 = -32768;
    let control: Vec<u8> = vec![103, 128, 0];

    let buf: Vec<u8>  = Vec::new();
    let mut FW = Serializer::new(buf);
    FW.write_int(v as i64).unwrap();
    assert_eq!(FW.get_ref(), &control );
    FW.reset();
    v.serialize(&mut FW).unwrap();
    assert_eq!(FW.get_ref(), &control);

    //Short/MAX_VALUE
    let v: i16 = 32767;
    let control: Vec<u8> = vec![104, 127, 255];

    let buf: Vec<u8>  = Vec::new();
    let mut FW = Serializer::new(buf);
    FW.write_int(v as i64).unwrap();
    assert_eq!(FW.get_ref(), &control );
    FW.reset();
    v.serialize(&mut FW).unwrap();
    assert_eq!(FW.get_ref(), &control);

    //Integer/MIN_VALUE
    let v: i32 = -2147483648;
    let control: Vec<u8> = vec![117, 128, 0, 0, 0];

    let buf: Vec<u8>  = Vec::new();
    let mut FW = Serializer::new(buf);
    FW.write_int(v as i64).unwrap();
    assert_eq!(FW.get_ref(), &control );
    FW.reset();
    v.serialize(&mut FW).unwrap();
    assert_eq!(FW.get_ref(), &control);

    //Integer/MAX_VALUE
    let v: i32 = 2147483647;
    let control: Vec<u8> = vec![118, 127, 255, 255, 255];

    let buf: Vec<u8>  = Vec::new();
    let mut FW = Serializer::new(buf);
    FW.write_int(v as i64).unwrap();
    assert_eq!(FW.get_ref(), &control );
    FW.reset();
    v.serialize(&mut FW).unwrap();
    assert_eq!(FW.get_ref(), &control);

    // min i40
    let v: i64 = -549755813887;
    let control: Vec<u8> = vec![121, 128, 0, 0, 0, 1];

    let buf: Vec<u8>  = Vec::new();
    let mut FW = Serializer::new(buf);
    FW.write_int(v as i64).unwrap();
    assert_eq!(FW.get_ref(), &control );
    FW.reset();
    v.serialize(&mut FW).unwrap();
    assert_eq!(FW.get_ref(), &control);

    // max i40
    let v: i64 = 549755813888;
    let control: Vec<u8> = vec![122, 128, 0, 0, 0, 0];

    let buf: Vec<u8>  = Vec::new();
    let mut FW = Serializer::new(buf);
    FW.write_int(v as i64).unwrap();
    assert_eq!(FW.get_ref(), &control );
    FW.reset();
    v.serialize(&mut FW).unwrap();
    assert_eq!(FW.get_ref(), &control);

    // max i48
    let v: i64 = 140737490000000;
    let control: Vec<u8> = vec![126, 128, 0, 0, 25, 24, 128];

    let buf: Vec<u8>  = Vec::new();
    let mut FW = Serializer::new(buf);
    FW.write_int(v as i64).unwrap();
    assert_eq!(FW.get_ref(), &control );
    FW.reset();
    v.serialize(&mut FW).unwrap();
    assert_eq!(FW.get_ref(), &control);

    // JS_MAX_SAFE_INT
    let v: i64 = 9007199254740991;
    let control: Vec<u8> = vec![248, 0, 31, 255, 255, 255, 255, 255, 255];

    let buf: Vec<u8>  = Vec::new();
    let mut FW = Serializer::new(buf);
    FW.write_int(v as i64).unwrap();
    assert_eq!(FW.get_ref(), &control );
    FW.reset();
    v.serialize(&mut FW).unwrap();
    assert_eq!(FW.get_ref(), &control);

    // JS_MAX_SAFE_INT++
    let v: i64 = 9007199254740992;
    let control: Vec<u8> = vec![248, 0, 32, 0, 0, 0, 0, 0, 0];

    let buf: Vec<u8>  = Vec::new();
    let mut FW = Serializer::new(buf);
    FW.write_int(v as i64).unwrap();
    assert_eq!(FW.get_ref(), &control );
    FW.reset();
    v.serialize(&mut FW).unwrap();
    assert_eq!(FW.get_ref(), &control);

    // JS_MIN_SAFE_INT
    let v: i64 = -9007199254740991;
    let control: Vec<u8> = vec![248, 255, 224, 0, 0, 0, 0, 0, 1];

    let buf: Vec<u8>  = Vec::new();
    let mut FW = Serializer::new(buf);
    FW.write_int(v as i64).unwrap();
    assert_eq!(FW.get_ref(), &control );
    FW.reset();
    v.serialize(&mut FW).unwrap();
    assert_eq!(FW.get_ref(), &control);

    // JS_MIN_SAFE_INT--
    let v: i64 = -9007199254740992;
    let control: Vec<u8> = vec![248, 255, 224, 0, 0, 0, 0, 0, 0];

    let buf: Vec<u8>  = Vec::new();
    let mut FW = Serializer::new(buf);
    FW.write_int(v as i64).unwrap();
    assert_eq!(FW.get_ref(), &control );
    FW.reset();
    v.serialize(&mut FW).unwrap();
    assert_eq!(FW.get_ref(), &control);

    // long max (i64)
    let v: i64 = 9223372036854775807;
    let control: Vec<u8> = vec![248, 127, 255, 255, 255, 255, 255, 255, 255];

    let buf: Vec<u8>  = Vec::new();
    let mut FW = Serializer::new(buf);
    FW.write_int(v as i64).unwrap();
    assert_eq!(FW.get_ref(), &control);
    FW.reset();
    v.serialize(&mut FW).unwrap();
    assert_eq!(FW.get_ref(), &control);

    // long min (i64)
    let v: i64 = -9223372036854775808;
    let control: Vec<u8> = vec![248, 128, 0, 0, 0, 0, 0, 0, 0];

    let buf: Vec<u8>  = Vec::new();
    let mut FW = Serializer::new(buf);
    FW.write_int(v as i64).unwrap();
    assert_eq!(FW.get_ref(), &control );
    FW.reset();
    v.serialize(&mut FW).unwrap();
    assert_eq!(FW.get_ref(), &control);

}

#[test]
fn bytes_test(){
    let buf: Vec<u8>  = Vec::new();
    let mut FW = Serializer::new(buf);

    // packed count
    let v: Vec<u8> = vec![255,254,253,0,1,2,3];
    let control: Vec<u8> = vec![215,255,254,253,0,1,2,3];
    FW.write_bytes(&v, 0, v.len()).unwrap();
    assert_eq!(FW.get_ref(), &control);

    // can't override built in serialize impl for vec<u8>
    // https://github.com/rust-lang/rust/issues/31844
    // soln -> https://docs.serde.rs/serde_bytes/
    let bb = serde_bytes::ByteBuf::from(v);
    FW.reset();
    bb.serialize(&mut FW).unwrap();
    assert_eq!(FW.get_ref(), &control);

    FW.reset();
    // unpacked length
    let v: Vec<u8> = vec![252,253,254,255,0,1,2,3,4];
    let control: Vec<u8> = vec![217, 9, 252, 253, 254, 255, 0, 1, 2, 3, 4];
    FW.write_bytes(&v, 0, v.len()).unwrap();
    assert_eq!(FW.get_ref(), &control);

}

