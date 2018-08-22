
use std::fmt;

use serde::de::{
    self, Deserialize, DeserializeSeed, EnumAccess, IntoDeserializer,
    MapAccess, SeqAccess, VariantAccess, Visitor,
};

use ordered_float::OrderedFloat;

use value::Value;

impl<'de> Deserialize<'de> for Value {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Value, D::Error>
        where D: serde::Deserializer<'de>
    {
        struct ValueVisitor;

        impl<'de> Visitor<'de> for ValueVisitor {
            type Value = Value;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("any valid fressian value")
            }

            fn visit_bool<E>(self, value: bool) -> Result<Value, E> {
                Ok(Value::BOOL(value))
            }

            fn visit_i64<E>(self, value: i64) -> Result<Value, E> {
                Ok(Value::INT(value))
            }

            fn visit_f32<E>(self, value: f32) -> Result<Value, E> {
                Ok(Value::FLOAT(OrderedFloat::from(value)))
            }

            fn visit_f64<E>(self, value: f64) -> Result<Value, E> {
                Ok(Value::DOUBLE(OrderedFloat::from(value)))
            }
        }

        deserializer.deserialize_any(ValueVisitor)
    }
}