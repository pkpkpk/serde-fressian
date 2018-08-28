use std::collections::{BTreeSet, BTreeMap};
use std::fmt;

use serde::ser::{self, Serialize};

use value::Value;

use INST::{INST};
use UUID::{UUID};
use URI::{URI};
use REGEX::{REGEX};
use SYM::{SYM};
use types::KEY::KEY;
use typed_arrays::*;


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
            Value::LIST(ref v) => v.serialize(serializer),
            Value::MAP(ref m) => m.serialize(serializer),
            Value::SET(ref s) => s.serialize(serializer),
            Value::KEY(ref v) => v.serialize(serializer),
            Value::SYM(ref v) => v.serialize(serializer),
            Value::INST(ref v) => v.serialize(serializer),
            Value::REGEX(ref v) => v.serialize(serializer),
            Value::URI(ref v) => v.serialize(serializer),
            Value::UUID(ref v) => v.serialize(serializer),
            Value::INT_ARRAY(ref v) => v.serialize(serializer),
            Value::LONG_ARRAY(ref v) => v.serialize(serializer),
            Value::FLOAT_ARRAY(ref v) => v.serialize(serializer),
            Value::DOUBLE_ARRAY(ref v) => v.serialize(serializer),
            Value::BOOLEAN_ARRAY(ref v) => v.serialize(serializer),
            Value::BYTES(ref v) => serializer.serialize_bytes(v.as_ref()),

            // CHAR(char)
            // BIGINT()
            // BIGDEC
            // UTF8(&'a str),
        }
    }
}

