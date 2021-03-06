#![allow(non_camel_case_types, non_snake_case)]

use std::collections::{BTreeMap};
use std::fmt;

use serde::de::{ self, Deserialize, SeqAccess, Visitor};

use ordered_float::OrderedFloat;
use serde_bytes::ByteBuf;

use crate::imp::codes;
use crate::value::Value;
use crate::inst::{INST};
use crate::uuid::{UUID};
use crate::uri::{URI};
use crate::regex::{REGEX};
use crate::sym::{SYM};
use crate::key::KEY;
use crate::typed_arrays::*;
use crate::set::{SET};

macro_rules! impl_seed {
    ($variant:ident : $T:ident) => {

        struct $variant;

        impl<'de> de::DeserializeSeed<'de> for $variant {
            type Value = $T;

            fn deserialize<D>(self, deserializer: D) -> Result<$T, D::Error>
                where D: serde::Deserializer<'de>
            {
                $T::deserialize(deserializer)
            }
        }
    }
}

impl_seed!(BYTES_SEED: ByteBuf);
impl_seed!(SYM_SEED: SYM);
impl_seed!(KEY_SEED: KEY);
impl_seed!(INST_SEED: INST);
impl_seed!(REGEX_SEED: REGEX);
impl_seed!(UUID_SEED: UUID);
impl_seed!(URI_SEED: URI);
impl_seed!(INT_ARRAY_SEED: IntArray);
impl_seed!(LONG_ARRAY_SEED: LongArray);
impl_seed!(FLOAT_ARRAY_SEED: FloatArray);
impl_seed!(DOUBLE_ARRAY_SEED: DoubleArray);
impl_seed!(BOOLEAN_ARRAY_SEED: BooleanArray);


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
                        codes::NULL => {
                            let _: Option<()> = seq.next_element()?;
                            Ok(Value::NULL)
                        },
                        codes::TRUE | codes::FALSE => {
                            let B: Option<bool> = seq.next_element()?;
                            match B {
                                Some(b) => Ok(Value::BOOL(b)),
                                None => Err(de::Error::custom("expected BOOL"))
                            }
                        },
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
                        codes::UTF8
                        |codes::STRING_PACKED_LENGTH_START..=codes::STRING_PACKED_LENGTH_END
                        | codes::STRING => {
                            let val: Option<String> = seq.next_element()?;
                            match val {
                                Some(s) => {
                                    Ok(Value::STRING(s))
                                },
                                None => Err(de::Error::custom("missing double"))
                            }
                        }
                        codes::BYTES_PACKED_LENGTH_START..=0xD8
                        | codes::BYTES => {
                            let val: Option<ByteBuf> = seq.next_element_seed(BYTES_SEED)?;
                            match val {
                                Some(bb) => {
                                    Ok(Value::BYTES(bb))
                                },
                                None => Err(de::Error::custom("missing BYTES"))
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
                        codes::SET => {
                            let val: Option<SET<Value>> = seq.next_element()?;
                            match val {
                                Some(v) => {
                                    Ok(Value::SET(v))
                                },
                                None => Err(de::Error::custom("missing map"))
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
                        codes::SYM => {
                            let val: Option<SYM> = seq.next_element_seed(SYM_SEED)?;
                            match val {
                                Some(sym) => {
                                    Ok(Value::SYM(sym))
                                },
                                None => Err(de::Error::custom("missing SYM"))
                            }
                        }
                        codes::INST => {
                            let val: Option<INST> = seq.next_element_seed(INST_SEED)?;
                            match val {
                                Some(inst) => {
                                    Ok(Value::INST(inst))
                                },
                                None => Err(de::Error::custom("missing INST"))
                            }
                        }
                        codes::REGEX => {
                            let val: Option<REGEX> = seq.next_element_seed(REGEX_SEED)?;
                            match val {
                                Some(regex) => {
                                    Ok(Value::REGEX(regex))
                                },
                                None => Err(de::Error::custom("missing REGEX"))
                            }
                        }
                        codes::UUID => {
                            let val: Option<UUID> = seq.next_element_seed(UUID_SEED)?;
                            match val {
                                Some(u) => {
                                    Ok(Value::UUID(u))
                                },
                                None => Err(de::Error::custom("missing UUID"))
                            }
                        }
                        codes::URI => {
                            let val: Option<URI> = seq.next_element_seed(URI_SEED)?;
                            match val {
                                Some(u) => {
                                    Ok(Value::URI(u))
                                },
                                None => Err(de::Error::custom("missing URI"))
                            }
                        }
                        codes::INT_ARRAY => {
                            let val: Option<IntArray> = seq.next_element_seed(INT_ARRAY_SEED)?;
                            match val {
                                Some(v) => {
                                    Ok(Value::from(v))
                                },
                                None => Err(de::Error::custom("missing INT_ARRAY"))
                            }
                        }
                        codes::LONG_ARRAY => {
                            let val: Option<LongArray> = seq.next_element_seed(LONG_ARRAY_SEED)?;
                            match val {
                                Some(v) => {
                                    Ok(Value::from(v))
                                },
                                None => Err(de::Error::custom("missing LONG_ARRAY"))
                            }
                        }
                        codes::FLOAT_ARRAY => {
                            let val: Option<FloatArray> = seq.next_element_seed(FLOAT_ARRAY_SEED)?;
                            match val {
                                Some(v) => {
                                    Ok(Value::from(v))
                                },
                                None => Err(de::Error::custom("missing FLOAT_ARRAY"))
                            }
                        }
                        codes::DOUBLE_ARRAY => {
                            let val: Option<DoubleArray> = seq.next_element_seed(DOUBLE_ARRAY_SEED)?;
                            match val {
                                Some(v) => {
                                    Ok(Value::from(v))
                                },
                                None => Err(de::Error::custom("missing DOUBLE_ARRAY"))
                            }
                        }
                        codes::BOOLEAN_ARRAY => {
                            let val: Option<BooleanArray> = seq.next_element_seed(BOOLEAN_ARRAY_SEED)?;
                            match val {
                                Some(v) => {
                                    Ok(Value::from(v))
                                },
                                None => Err(de::Error::custom("missing BOOLEAN_ARRAY"))
                            }
                        }
                        _ => Err(de::Error::custom(format!("Value UnmatchedCode:: {}", code as u8)))
                    }
                } else {
                    Err(de::Error::custom("code == None"))
                }
            }
        }
        deserializer.deserialize_tuple(2, ValueVisitor)
    }
}

