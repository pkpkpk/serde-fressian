use std::collections::{BTreeSet, BTreeMap};
use serde::ser::Serialize;
use serde_bytes::ByteBuf;
use ordered_float::OrderedFloat;

// use error::{Error};
use inst::{INST};
use uuid::{UUID};
use uri::{URI};
use regex::{REGEX};
use sym::{SYM};
use key::{KEY};
use typed_arrays::*;
use set::{SET};

mod de;
// mod index;

/// Represents a Fressian value
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
    MAP(BTreeMap<Value, Value>),
    SET(SET<Value>),
    SYM(SYM),
    KEY(KEY),
    INST(INST),
    UUID(UUID),
    REGEX(REGEX),
    URI(URI),
    IntArray(Int_Array),
    LongArray(Long_Array),
    FloatArray(Float_Array),
    DoubleArray(Double_Array),
    BooleanArray(Boolean_Array)
    // BIGINT()
    // BIGDEC
    // OBJECT_ARRAY
    // RECORD
    // TAGGED_OBJECT
    // Rust types? Iter? Seq?
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
impl_into_value!(IntArray: Int_Array);
impl_into_value!(LongArray: Long_Array);
impl_into_value!(FloatArray: Float_Array);
impl_into_value!(DoubleArray: Double_Array);
impl_into_value!(BooleanArray: Boolean_Array);
impl_into_value!(BYTES: ByteBuf);

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

impl<'a> From<&'a[u8]> for Value {
    fn from(val: &'a[u8]) -> Value {
        Value::BYTES(ByteBuf::from(val))
    }
}

impl Serialize for Value {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        match *self {
            Value::NULL => serializer.serialize_unit(),
            Value::BOOL(b) => serializer.serialize_bool(b),
            Value::INT(ref i) => i.serialize(serializer),
            Value::FLOAT(ref f) => f.serialize(serializer),
            Value::DOUBLE(ref d) => d.serialize(serializer),
            Value::STRING(ref s) => serializer.serialize_str(s),
            Value::LIST(ref v) => v.serialize(serializer), // newtype with cache call?!
            Value::MAP(ref m) => m.serialize(serializer),
            Value::SET(ref s) => s.serialize(serializer),
            Value::KEY(ref v) => v.serialize(serializer),
            Value::SYM(ref v) => v.serialize(serializer),
            Value::INST(ref v) => v.serialize(serializer),
            Value::REGEX(ref v) => v.serialize(serializer),
            Value::URI(ref v) => v.serialize(serializer),
            Value::UUID(ref v) => v.serialize(serializer),
            Value::IntArray(ref v) => v.serialize(serializer),
            Value::LongArray(ref v) => v.serialize(serializer),
            Value::FloatArray(ref v) => v.serialize(serializer),
            Value::DoubleArray(ref v) => v.serialize(serializer),
            Value::BooleanArray(ref v) => v.serialize(serializer),
            Value::BYTES(ref v) => serializer.serialize_bytes(v.as_ref()),

            // CHAR(char)
            // BIGINT()
            // BIGDEC
            // UTF8(&'a str),
        }
    }
}
