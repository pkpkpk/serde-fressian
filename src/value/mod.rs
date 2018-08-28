
use std::collections::{BTreeSet, BTreeMap};
use std::cmp::{Ordering, Ord, PartialOrd};
use std::fmt::{Display, Formatter};
use std::f64;

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

mod de;
mod ser;
// mod from;
// mod index;
// mod partial_eq;

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
    // BYTES(&[u8]),
    LIST(Vec<Value>),
    // gonna let the good people at mozilla make hard decisions for us:
    //   " We're using BTree{Set, Map} rather than Hash{Set, Map} because the BTree variants
    //    implement Hash. The Hash variants don't in order to preserve O(n) hashing
    //    time, which is hard given recursive data structures.
    //    See https://internals.rust-lang.org/t/implementing-hash-for-hashset-hashmap/3817/1 "
    SET(BTreeSet<Value>),
    MAP(BTreeMap<Value, Value>),
    SYM(SYM),
    KEY(KEY),
    INST(INST),
    UUID(UUID),
    REGEX(REGEX),
    URI(URI),
    // BIGINT()
    // BIGDEC
    INT_ARRAY(Int_Array),
    LONG_ARRAY(Long_Array),
    FLOAT_ARRAY(Float_Array),
    DOUBLE_ARRAY(Double_Array),
    BOOLEAN_ARRAY(Boolean_Array)
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
