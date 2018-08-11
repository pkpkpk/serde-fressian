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
use serde::de::{self, Deserialize};
use serde::Serialize;

use serde_fressian::ser::{Serializer};
use serde_fressian::de::{Deserializer, from_vec};



#[test]
fn de_test(){

    // boolean
    let mut fw = Serializer::new();
    let value = true;
    &value.serialize(&mut fw).unwrap();
    let mut rdr = Deserializer::from_vec(fw.get_ref());
    assert_eq!(Ok(value), bool::deserialize(&mut rdr));

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
}

#[test]
fn test_reset(){
    let mut fw = Serializer::new();

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
fn bytes_serialization_test(){
    let mut fw = Serializer::new();

    //// manually specifying FressianWriter.write_bytes()
    //-----  packed count
    let v: Vec<u8> = vec![255,254,253,0,1,2,3];
    let control: Vec<u8> = vec![215,255,254,253,0,1,2,3];
    fw.write_bytes(&v, 0, v.len()).unwrap();
    assert_eq!(&fw.to_vec(), &control);
    fw.reset();
    fw.write_bytes(v.as_slice(), 0, v.len()).unwrap();
    assert_eq!(&fw.to_vec(), &control);
    //---   unpacked length
    let v: Vec<u8> = vec![252,253,254,255,0,1,2,3,4];
    let control: Vec<u8> = vec![217, 9, 252, 253, 254, 255, 0, 1, 2, 3, 4];
    fw.reset();
    fw.write_bytes(&v, 0, v.len()).unwrap();
    assert_eq!(&fw.to_vec(), &control);
    fw.reset();
    fw.write_bytes(v.as_slice(), 0, v.len()).unwrap();
    assert_eq!(&fw.to_vec(), &control);

    //// I have no idea why this doesnt work
    // fw.reset();
    // &mut fw.serialize_bytes(v.as_slice()).unwrap();
    // assert_eq!(&fw.to_vec(), &control);


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
    let mut fw = Serializer::new();

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
    let mut fw = Serializer::new();

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

    let mut m: HashMap<String, u8> = HashMap::new();
    m.insert("a".to_string(), 0);
    m.insert("b".to_string(), 1);
    #[cfg(not(raw_UTF8))]
    let control: Vec<u8> = vec![192,232,219,97,0,219,98,1];
    #[cfg(raw_UTF8)]
    let control: Vec<u8> = vec![192, 232, 191, 1, 97, 0, 191, 1, 98, 1];
    fw.write_map(&m).unwrap();
    let buf = fw.to_vec();
    // assert_eq!(&buf, &control);

    assert_map_eq(&buf, &control, 4);

}

