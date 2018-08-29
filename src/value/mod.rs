
use std::collections::{BTreeSet, BTreeMap};
use std::cmp::{Ordering, Ord, PartialOrd};
// use std::fmt::{Display, Formatter};
// use std::f64;

use serde::ser::Serialize;
use serde_bytes::ByteBuf;

use ordered_float::OrderedFloat;

// this stuff all needs wrappers so we can have serialize/deserialize
// TODO explore remote attr, specialization so we can dump these
use INST::{INST};
use UUID::{UUID};
use URI::{URI};
use REGEX::{REGEX};
use SYM::{SYM};
use KEY::{KEY};
use typed_arrays::*;
use SET::{SET};

mod de;
mod ser;
// mod from;
// mod index;
// mod partial_eq;
// use self::ser::Serializer;

/// Represents a Fressian value
/// see serde_json + https://github.com/mozilla/mentat/blob/master/edn/src/types.rs
// mentat also has fleshed out symbols, kws, linkedlist
#[derive(Clone, PartialEq, PartialOrd, Ord, Eq, Hash, Debug)]
pub enum Value {
    NULL,
    BOOL(bool),
    INT(i64),
    FLOAT(OrderedFloat<f32>),
    DOUBLE(OrderedFloat<f64>),
    // CHAR(char)
    STRING(String),
    // UTF8(&'a str),
    BYTES(ByteBuf), // ideally should be &'a [u8]
    LIST(Vec<Value>),
    // gonna let the good people at mozilla make hard decisions for us:
    //   " We're using BTree{Set, Map} rather than Hash{Set, Map} because the BTree variants
    //    implement Hash. The Hash variants don't in order to preserve O(n) hashing
    //    time, which is hard given recursive data structures.
    //    See https://internals.rust-lang.org/t/implementing-hash-for-hashset-hashmap/3817/1 "
    MAP(BTreeMap<Value, Value>),
    SET(SET<Value>),
    SYM(SYM),
    KEY(KEY),
    INST(INST),
    UUID(UUID),
    REGEX(REGEX),
    URI(URI),
    INT_ARRAY(Int_Array),
    LONG_ARRAY(Long_Array),
    FLOAT_ARRAY(Float_Array),
    DOUBLE_ARRAY(Double_Array),
    BOOLEAN_ARRAY(Boolean_Array)
    // BIGINT()
    // BIGDEC
    // OBJECT_ARRAY
    // RECORD
    // TAGGED_OBJECT
}

macro_rules! impl_into_value {
    ($variant:ident : $T:ty) => {
        impl From<$T> for Value {
            #[inline]
            fn from(val: $T) -> Value {
                Value::$variant(val.into())
            }
        }
    }
}

impl_into_value!(STRING: String);
impl_into_value!(INT: i8);
impl_into_value!(INT: i16);
impl_into_value!(INT: i32);
impl_into_value!(INT: i64);
impl_into_value!(INT: u8);
impl_into_value!(INT: u16);
impl_into_value!(INT: u32);
impl_into_value!(DOUBLE: f64);
impl_into_value!(FLOAT: f32);
impl_into_value!(BOOL: bool);
impl_into_value!(KEY: KEY);
impl_into_value!(SYM: SYM);
impl_into_value!(INST: INST);
impl_into_value!(REGEX: REGEX);
impl_into_value!(UUID: UUID);
impl_into_value!(URI: URI);
impl_into_value!(INT_ARRAY: Int_Array);
impl_into_value!(LONG_ARRAY: Long_Array);
impl_into_value!(FLOAT_ARRAY: Float_Array);
impl_into_value!(DOUBLE_ARRAY: Double_Array);
impl_into_value!(BOOLEAN_ARRAY: Boolean_Array);


impl<T: Into<Value>> From<Vec<T>> for Value {
    fn from(val: Vec<T>) -> Value {
        Value::LIST(val.into_iter().map(Into::into).collect())
    }
}

impl From<SET<Value>> for Value
{
    #[inline]
    fn from(val: SET<Value>) -> Value {
        Value::SET(val)
    }
}

impl<T: Into<Value>> From<BTreeSet<T>> for Value {
    fn from(val: BTreeSet<T>) -> Value {
        let set: BTreeSet<Value> = val.into_iter().map(Into::into).collect();
        Value::SET(SET::from(set))
    }
}

impl<K,V> From<BTreeMap<K,V>> for Value
    where K: Into<Value>,
          V: Into<Value>,
{
    fn from(val: BTreeMap<K,V>) -> Value {
        Value::MAP(val.into_iter().map(|(k,v)|(k.into(), v.into())).collect())
    }
}

