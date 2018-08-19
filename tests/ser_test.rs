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
fn de_test(){

    // boolean
    let mut fw = Serializer::from_vec(Vec::new());
    let value = true;
    &value.serialize(&mut fw).unwrap();
    let mut rdr = Deserializer::from_vec(fw.get_ref());
    assert_eq!(Ok(value), bool::deserialize(&mut rdr));

    /////////////////////////////////////////////////////////////

    let mut fw = Serializer::from_vec(Vec::new());
    let value = ();
    &value.serialize(&mut fw).unwrap();
    let mut rdr = Deserializer::from_vec(fw.get_ref());
    assert_eq!(Ok(value), <()>::deserialize(&mut rdr));

    // (api/write [nil nil nil])
    let value: Vec<u8> = vec![231,247,247,247];
    let control: Vec<()> = vec![(),(),()];
    let t: Vec<()> = serde_fressian::de::from_vec(&value).unwrap();
    assert_eq!(control, t);

    /////////////////////////////////////////////////////////////

    // packed list of numbers
    // (api/write [0 1 2 3])
    let value: Vec<u8> = vec![232,0,1,2,3];
    let control: Vec<i64> = vec![0,1,2,3];
    let t: Vec<i64> = serde_fressian::de::from_vec(&value).unwrap();
    assert_eq!(control, t);

    ///////////////////////////////////////////////////////////

    // packed list of numbers, with -1
    // (api/write [-1 0 1 2 3])
    let value: Vec<u8> = vec![233,255,0,1,2,3];
    let control: Vec<i64> = vec![-1, 0, 1, 2, 3];
    let t: Vec<i64> = serde_fressian::de::from_vec(&value).unwrap();
    assert_eq!(control, t);

    ///////////////////////////////////////////////////////////

    // unpacked list numbers
    // (api/write [-4 -3 -2 -1 0 1 2 3 4])
    let value: Vec<u8> = vec![236,9,79,252,79,253,79,254,255,0,1,2,3,4];
    let control: Vec<i64> = vec![-4, -3, -2, -1, 0, 1, 2, 3, 4];
    let t: Vec<i64> = serde_fressian::de::from_vec(&value).unwrap();
    assert_eq!(control, t);

    ///////////////////////////////////////////////////////////////

    //closed list
    // (write-as-closed [-3 -2 -1 0 1 2 3])
    let value: Vec<u8> = vec![237,79,253,79,254,255,0,1,2,3,253];
    let control: Vec<i64> = vec![-3,-2,-1, 0, 1, 2, 3];
    let t: Vec<i64> = serde_fressian::de::from_vec(&value).unwrap();
    assert_eq!(control, t);

    //test if END_COLLECTION check for ClosedListReader works
    let value: Vec<u8> =vec![230,237,79,253,79,254,255,0,1,2,3,253,231,80,99,80,100,80,101];
    let control: Vec<Vec<i64>> = vec![vec![-3,-2,-1, 0, 1, 2, 3], vec![99, 100, 101]];
    let t: Vec<Vec<i64>> = serde_fressian::de::from_vec(&value).unwrap();
    assert_eq!(control, t);

    ///////////////////////////////////////////////////////////////

    //open list
    // (write-as-open [-2 -1 0 1 2])
    let value: Vec<u8> = vec![238,79,254,255,0,1,2];
    let control: Vec<i64> = vec![-2,-1, 0, 1, 2];
    let t: Vec<i64> = serde_fressian::de::from_vec(&value).unwrap();
    assert_eq!(control, t);

    ///////////////////////////////////////////////////////////////

    //map
    //(api/write {"foo" "bar"})
    let value: Vec<u8> = vec![192,230,221,102,111,111,221,98,97,114];
    let mut control: HashMap<String,String> = HashMap::new();
    control.insert("foo".to_string(), "bar".to_string());
    let t: HashMap<String,String> = serde_fressian::de::from_vec(&value).unwrap();
    assert_eq!(control, t);

    ////////////////////////////////////////////////////////////////////

    //map; closed list
    // (write-as-closed {"thom" "jonny" "phil" "colin"})
    let value: Vec<u8> = vec![192,237,222,116,104,111,109,223,106,111,110,110,121,222,112,104,105,108,223,99,111,108,105,110,253];
    let mut control: HashMap<String,String> = HashMap::new();
    control.insert("thom".to_string(), "jonny".to_string());
    control.insert("phil".to_string(), "colin".to_string());
    let t: HashMap<String,String> = serde_fressian::de::from_vec(&value).unwrap();
    assert_eq!(control, t);

    ////////////////////////////////////////////////////////////////////

    //set (api/write #{"thom" "jonny" "phil" "colin" "ed"})
    let value: Vec<u8> = vec![193,233,223,99,111,108,105,110,220,101,100,222,116,104,111,109,223,106,111,110,110,121,222,112,104,105,108];
    let mut control: HashSet<String> = HashSet::new();
    control.insert("thom".to_string());
    control.insert("jonny".to_string());
    control.insert("phil".to_string());
    control.insert("colin".to_string());
    control.insert("ed".to_string());
    let t: HashSet<String> = serde_fressian::de::from_vec(&value).unwrap();
    assert_eq!(control, t);

    ////////////////////////////////////////////////////////////////////

    // (write-as-closed #{"thom" "jonny" "phil" "colin" "ed"})
    let value: Vec<u8> = vec![193,237,223,99,111,108,105,110,220,101,100,222,116,104,111,109,223,106,111,110,110,121,222,112,104,105,108,253];
    let t: HashSet<String> = serde_fressian::de::from_vec(&value).unwrap();
    assert_eq!(control, t);
}

#[test]
fn test_reset(){
    let mut fw = Serializer::from_vec(Vec::new());

    let v: Vec<i64> = vec![-2, -1, 0, 1, 2];
    let control: Vec<u8> = vec![233,79,254,255,0,1,2];
    // fw.write_list(&v).unwrap();
    &v.serialize(&mut fw).unwrap();
    assert_eq!(fw.to_vec(), control);
    fw.reset();
    let control: Vec<u8> = vec![];
    assert_eq!(fw.to_vec(), control);
}


#[test]
fn bytes_serialization_test(){
    let mut fw = Serializer::from_vec(Vec::new());

    //// manually specifying FressianWriter.write_bytes()
    //-----  packed count
    let v: Vec<u8> = vec![255,254,253,0,1,2,3];
    let control: Vec<u8> = vec![215,255,254,253,0,1,2,3];
    fw.write_bytes(&v, 0, v.len()).unwrap();
    assert_eq!(fw.to_vec(), control);
    fw.reset();
    fw.write_bytes(v.as_slice(), 0, v.len()).unwrap();
    assert_eq!(fw.to_vec(), control);
    //---   unpacked length
    let v: Vec<u8> = vec![252,253,254,255,0,1,2,3,4];
    let control: Vec<u8> = vec![217, 9, 252, 253, 254, 255, 0, 1, 2, 3, 4];
    fw.reset();
    fw.write_bytes(&v, 0, v.len()).unwrap();
    assert_eq!(fw.to_vec(), control);
    fw.reset();
    fw.write_bytes(v.as_slice(), 0, v.len()).unwrap();
    assert_eq!(fw.to_vec(), control);

    // By default, serde writes Vec<u8> + &[u8] as lists when calling .serialize()
    // - see https://serde.rs/impl-serialize.html#other-special-cases
    // - can't override built in serialize impl?
    // https://github.com/rust-lang/rust/issues/31844
    // soln -> https://docs.serde.rs/serde_bytes/
    let bb = serde_bytes::ByteBuf::from(v);
    fw.reset();
    bb.serialize(&mut fw).unwrap();
    assert_eq!(&fw.to_vec(), &control);

}


#[test]
fn list_test(){
    let mut fw = Serializer::from_vec(Vec::new());

    let v: Vec<i64> = vec![-2, -1, 0, 1, 2];
    let control: Vec<u8> = vec![233,79,254,255,0,1,2];
    // fw.write_list(&v).unwrap();
    &v.serialize(&mut fw).unwrap();
    assert_eq!(fw.to_vec(), control);
    fw.reset();
    &v.serialize(&mut fw);
    assert_eq!(fw.to_vec(), control);

    fw.reset();

    let v: Vec<String> = vec!["i".to_string(), "am".to_string(), "a".to_string(), "reasonable".to_string(), "man".to_string(), "get".to_string(), "off".to_string(), "my".to_string(), "case".to_string()];
    #[cfg(not(raw_UTF8))]
    let control: Vec<u8> = vec![236,9,219,105,220,97,109,219,97,227,10,114,101,97,115,111,110,97,98,108,101,221,109,97,110,221,103,101,116,221,111,102,102,220,109,121,222,99,97,115,101];
    #[cfg(raw_UTF8)]
    let control: Vec<u8> = vec![236,9,191,1,105,191,2,97,109,191,1,97,191,10,114,101,97,115,111,110,97,98,108,101,191,3,109,97,110,191,3,103,101,116,191,3,111,102,102,191,2,109,121,191,4,99,97,115,101];

    // fw.write_list(&v).unwrap();
    &v.serialize(&mut fw).unwrap();
    assert_eq!(fw.to_vec(), control);
    fw.reset();
    &v.serialize(&mut fw);
    assert_eq!(fw.to_vec(), control);

    fw.reset();

    let sub_v0: Vec<String> = vec!["some".to_string()];
    let sub_v1: Vec<String> = vec!["nested".to_string()];
    let sub_v2: Vec<String> = vec!["shit".to_string()];
    let v: Vec<Vec<String>> = vec![sub_v0, sub_v1, sub_v2];
    #[cfg(not(raw_UTF8))]
    let control: Vec<u8> = vec![231,229,222,115,111,109,101,229,224,110,101,115,116,101,100,229,222,115,104,105,116];
    #[cfg(raw_UTF8)]
    let control: Vec<u8> = vec![231,229,191,4,115,111,109,101,229,191,6,110,101,115,116,101,100,229,191,4,115,104,105,116];
    // fw.write_list(&v).unwrap();
    &v.serialize(&mut fw).unwrap();
    assert_eq!(fw.to_vec(), control);

    fw.reset();
    &v.serialize(&mut fw);
    assert_eq!(fw.to_vec(), control);
}



const MAP: u8 = 0xC0; //192
const LIST_PACKED_LENGTH_START: u8 = 0xE4; //228
const LIST: u8 = 0xEC; //236

// hashmap iter has notdet ordering, so cannot compare bytes
// need to unpack manually
fn assert_map_eq(a: &Vec<u8>, b: &Vec<u8>, count: i32) {
    let mut rdr_a = Deserializer::from_vec(a);
    let mut rdr_b = Deserializer::from_vec(b);

    // check map flag
    assert_eq!(rdr_a.read_next_code().unwrap(), MAP as i8);
    assert_eq!(rdr_b.read_next_code().unwrap(), MAP as i8);

    let expected_list_signal: i8 = match count {
        0..=8 => {
            (LIST_PACKED_LENGTH_START + count as u8) as i8
        }
        _ => LIST as i8
    };

    // check LIST flag
    assert_eq!(rdr_a.read_next_code().unwrap(), expected_list_signal);
    assert_eq!(rdr_b.read_next_code().unwrap(), expected_list_signal);

    if  expected_list_signal == LIST as i8 {
        // check count
    };

    // pair off rest as map entry tuples, put in sets and compare

}

#[test]
fn map_test() {
    let mut fw = Serializer::from_vec(Vec::new());

    let mut map: HashMap<String, u8> = HashMap::new();
    map.insert("a".to_string(), 0);
    #[cfg(not(raw_UTF8))]
    let control: Vec<u8> = vec![192,230,219,97,0];
    #[cfg(raw_UTF8)]
    let control: Vec<u8> = vec![192, 230, 191, 1, 97, 0];
    map.serialize(&mut fw).unwrap();
    assert_eq!(fw.to_vec(), control);

    fw.reset();

    let mut map: HashMap<String, u8> = HashMap::new();
    map.insert("a".to_string(), 0);
    map.insert("b".to_string(), 1);
    #[cfg(not(raw_UTF8))]
    let control: Vec<u8> = vec![192,232,219,97,0,219,98,1];
    #[cfg(raw_UTF8)]
    let control: Vec<u8> = vec![192, 232, 191, 1, 97, 0, 191, 1, 98, 1];
    map.serialize(&mut fw).unwrap();
    let buf = fw.to_vec();
    // assert_eq!(&buf, &control);

    assert_map_eq(&buf, &control, 4);

}


#[test]
fn ints_test (){
    let mut fw = Serializer::from_vec(Vec::new());
    //Short/MIN_VALUE
    let v: i16 = -32768;
    let control: Vec<u8> = vec![103, 128, 0];
    fw.write_int(v as i64).unwrap();
    assert_eq!(&fw.to_vec(), &control);

    //Short/MAX_VALUE
    let v: i16 = 32767;
    let control: Vec<u8> = vec![104, 127, 255];
    fw.reset();
    fw.write_int(v as i64).unwrap();
    assert_eq!(&fw.to_vec(), &control);

    //Integer/MIN_VALUE
    let v: i32 = -2147483648;
    let control: Vec<u8> = vec![117, 128, 0, 0, 0];
    fw.reset();
    fw.write_int(v as i64).unwrap();
    assert_eq!(&fw.to_vec(), &control);

    //Integer/MAX_VALUE
    let v: i32 = 2147483647;
    let control: Vec<u8> = vec![118, 127, 255, 255, 255];
    fw.reset();
    fw.write_int(v as i64).unwrap();
    assert_eq!(&fw.to_vec(), &control);

    // min i40
    let v: i64 = -549755813887;
    let control: Vec<u8> = vec![121, 128, 0, 0, 0, 1];
    fw.reset();
    fw.write_int(v as i64).unwrap();
    assert_eq!(&fw.to_vec(), &control);

    // max i40
    let v: i64 = 549755813888;
    let control: Vec<u8> = vec![122, 128, 0, 0, 0, 0];
    fw.reset();
    fw.write_int(v as i64).unwrap();
    assert_eq!(&fw.to_vec(), &control);

    // max i48
    let v: i64 = 140737490000000;
    let control: Vec<u8> = vec![126, 128, 0, 0, 25, 24, 128];
    fw.reset();
    fw.write_int(v as i64).unwrap();
    assert_eq!(&fw.to_vec(), &control);

    // JS_MAX_SAFE_INT
    let v: i64 = 9007199254740991;
    let control: Vec<u8> = vec![248, 0, 31, 255, 255, 255, 255, 255, 255];
    fw.reset();
    fw.write_int(v as i64).unwrap();
    assert_eq!(&fw.to_vec(), &control);

    // JS_MAX_SAFE_INT++
    let v: i64 = 9007199254740992;
    let control: Vec<u8> = vec![248, 0, 32, 0, 0, 0, 0, 0, 0];
    fw.reset();
    fw.write_int(v as i64).unwrap();
    assert_eq!(&fw.to_vec(), &control);

    // JS_MIN_SAFE_INT
    let v: i64 = -9007199254740991;
    let control: Vec<u8> = vec![248, 255, 224, 0, 0, 0, 0, 0, 1];
    fw.reset();
    fw.write_int(v as i64).unwrap();
    assert_eq!(&fw.to_vec(), &control);

    // JS_MIN_SAFE_INT--
    let v: i64 = -9007199254740992;
    let control: Vec<u8> = vec![248, 255, 224, 0, 0, 0, 0, 0, 0];
    fw.reset();
    fw.write_int(v as i64).unwrap();
    assert_eq!(&fw.to_vec(), &control);

    // long max (i64)
    let v: i64 = 9223372036854775807;
    let control: Vec<u8> = vec![248, 127, 255, 255, 255, 255, 255, 255, 255];
    fw.reset();
    fw.write_int(v as i64).unwrap();
    assert_eq!(&fw.to_vec(), &control);

    // long min (i64)
    let v: i64 = -9223372036854775808;
    let control: Vec<u8> = vec![248, 128, 0, 0, 0, 0, 0, 0, 0];
    fw.reset();
    fw.write_int(v as i64).unwrap();
    assert_eq!(&fw.to_vec(), &control);
}

#[test]
fn write_floats_test(){
    let mut fw = Serializer::from_vec(Vec::new());

    //Float/MIN_VALUE
    let v: f32 = 1.4E-45;
    let control: Vec<u8> = vec![249, 0, 0, 0, 1];
    fw.write_float(v).unwrap();
    assert_eq!(&fw.to_vec(), &control);

    fw.reset();

    //Float/MAX_VALUE
    let v: f32 = 3.4028235E38;
    let control: Vec<u8> = vec![249, 127, 127, 255, 255];
    fw.write_float(v).unwrap();
    assert_eq!(&fw.to_vec(), &control);

    fw.reset();

    // DOUBLE/MIN_VALUE
    let v: f64 = 4.9E-324;
    let control: Vec<u8> = vec![250, 0, 0, 0, 0, 0, 0, 0, 1];
    fw.write_double(v).unwrap();
    assert_eq!(&fw.to_vec(), &control);

    fw.reset();

    // DOUBLE/MAX_VALUE
    let v: f64 = 1.7976931348623157E308;
    let control: Vec<u8> = vec![250, 127, 239, 255, 255, 255, 255, 255, 255 ];
    fw.write_double(v).unwrap();
    assert_eq!(&fw.to_vec(), &control);
}

#[test]
fn write_bytes_test(){
    let mut fw = Serializer::from_vec(Vec::new());

    // packed count
    let v: Vec<u8> = vec![255,254,253,0,1,2,3];
    let control: Vec<u8> = vec![215,255,254,253,0,1,2,3];
    fw.write_bytes(v.as_slice(), 0, v.len()).unwrap();
    assert_eq!(&fw.to_vec(), &control);

    fw.reset();

    // unpacked length
    let v: Vec<u8> = vec![252,253,254,255,0,1,2,3,4];
    let control: Vec<u8> = vec![217, 9, 252, 253, 254, 255, 0, 1, 2, 3, 4];
    fw.write_bytes(v.as_slice(), 0, v.len()).unwrap();
    assert_eq!(&fw.to_vec(), &control);

    //missing chunked
}

#[test]
fn write_string_test(){
    let mut fw = Serializer::from_vec(Vec::new());

    let v = "".to_string();
    #[cfg(not(raw_UTF8))]
    let control: Vec<u8> = vec![218];
    #[cfg(raw_UTF8)]
    let control: Vec<u8> = vec![191,0];
    fw.write_string(&v).unwrap();
    assert_eq!(&fw.to_vec(), &control);

    fw.reset();

    let v = "hola".to_string();
    #[cfg(not(raw_UTF8))]
    let control: Vec<u8> = vec![222,104,111,108,97];
    #[cfg(raw_UTF8)]
    let control: Vec<u8> = vec![191,4,104,111,108,97];
    fw.write_string(&v).unwrap();
    assert_eq!(&fw.to_vec(), &control);

    fw.reset();

    let v = "é❤️ßℝ東京東京😉 😎 🤔 😐 🙄".to_string();
    #[cfg(not(raw_UTF8))]
    let control: Vec<u8> = vec![227,60,101,204,129,226,157,164,239,184,143,195,159,226,132,157,230,157,177,228,186,172,230,157,177,228,186,172,237,160,189,237,184,137,32,237,160,189,237,184,142,32,237,160,190,237,180,148,32,237,160,189,237,184,144,32,237,160,189,237,185,132];
    #[cfg(raw_UTF8)]
    let control: Vec<u8> = vec![191,50,101,204,129,226,157,164,239,184,143,195,159,226,132,157,230,157,177,228,186,172,230,157,177,228,186,172,240,159,152,137,32,240,159,152,142,32,240,159,164,148,32,240,159,152,144,32,240,159,153,132];
    fw.write_string(&v).unwrap();
    assert_eq!(&fw.to_vec(), &control);

    // missing chunked
}
