use std::collections::{BTreeSet, BTreeMap};
use std::fmt;

use serde::de::{
    self, Deserialize, DeserializeSeed, EnumAccess, IntoDeserializer,
    MapAccess, SeqAccess, VariantAccess, Visitor,
};

use ordered_float::OrderedFloat;

use imp::error::{Error};
use imp::codes;
use value::Value;
use INST::{INST};
use UUID::{UUID};
use URI::{URI};
use REGEX::{REGEX};
use SYM::{SYM};
use types::KEY::KEY;
use typed_arrays::*;


struct KEY_SEED;

impl<'de> de::DeserializeSeed<'de> for KEY_SEED {
    type Value = KEY;

    fn deserialize<D>(self, deserializer: D) -> Result<KEY, D::Error>
        where D: serde::Deserializer<'de>
    {
        KEY::deserialize(deserializer)
    }
}



impl<'de> Deserialize<'de> for Value {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Value, D::Error>
        where D: serde::Deserializer<'de>
    {
        struct ValueVisitor;

        impl<'de> de::DeserializeSeed<'de>  for ValueVisitor {
            type Value = i8;

            fn deserialize<D>(self, deserializer: D) -> Result<i8, D::Error>
                where D: serde::Deserializer<'de>
            {
                struct CodeVisitor;

                impl<'de> Visitor<'de> for CodeVisitor {
                    type Value = i8;

                    #[inline]
                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("code as i8")
                    }

                    fn visit_i8<E>(self, value: i8) -> Result<i8, E> {
                        Ok(value)
                    }
                }

                deserializer.deserialize_tuple_struct("CODE", 1, CodeVisitor)
            }
        }

        impl<'de> Visitor<'de> for ValueVisitor {
            type Value = Value;

            #[inline]
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("any valid fressian value")
            }

            #[inline]
            fn visit_seq<V>(self, mut seq: V) -> Result<Value, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let code: Option<i8> = seq.next_element_seed(self)?;

                if let Some(code) = code {
                    match code as u8 {
                        codes::NULL => Ok(Value::NULL),
                        codes::TRUE => Ok(Value::BOOL(true)),
                        codes::FALSE => Ok(Value::BOOL(false)),
                        0xFF | 0x00..=0x7f | codes::INT => {
                            let val: Option<i64> = seq.next_element()?;
                            match val {
                                Some(i) => {
                                    Ok(Value::INT(i))
                                },
                                None => Err(de::Error::custom("expected INT"))
                            }
                        }
                        codes::FLOAT => {
                            let val: Option<f32> = seq.next_element()?;
                            match val {
                                Some(f) => {
                                    Ok(Value::FLOAT(OrderedFloat::from(f)))
                                },
                                None => Err(de::Error::custom("missing float"))
                            }
                        }
                        codes::DOUBLE => {
                            let val: Option<f64> = seq.next_element()?;
                            match val {
                                Some(f) => {
                                    Ok(Value::DOUBLE(OrderedFloat::from(f)))
                                },
                                None => Err(de::Error::custom("missing double"))
                            }
                        }
                        codes::STRING_PACKED_LENGTH_START..=codes::STRING_PACKED_LENGTH_END
                        | codes::STRING => {
                            let val: Option<String> = seq.next_element()?;
                            match val {
                                Some(s) => {
                                    Ok(Value::STRING(s))
                                },
                                None => Err(de::Error::custom("missing double"))
                            }
                        }
                        codes::LIST_PACKED_LENGTH_START..=235
                        | codes::LIST
                        | codes::BEGIN_OPEN_LIST => {
                            let val: Option<Vec<Value>> = seq.next_element()?;
                            match val {
                                Some(v) => {
                                    Ok(Value::LIST(v))
                                },
                                None => Err(de::Error::custom("missing LIST"))
                            }
                        }
                        codes::MAP => {
                            let val: Option<BTreeMap<Value,Value>> = seq.next_element()?;
                            match val {
                                Some(m) => {
                                    Ok(Value::MAP(m))
                                },
                                None => Err(de::Error::custom("missing map"))
                            }
                        }
                        codes::KEY => {
                            let val: Option<KEY> = seq.next_element_seed(KEY_SEED)?;
                            match val {
                                Some(key) => {
                                    Ok(Value::KEY(key))
                                },
                                None => Err(de::Error::custom("missing KEY"))
                            }
                        }


                        _ => Err(de::Error::custom("UnmatchedCode"))
                    }
                } else {
                    Err(de::Error::custom("code == None"))
                }
            }
        }
        deserializer.deserialize_tuple(2, ValueVisitor)
    }
}

