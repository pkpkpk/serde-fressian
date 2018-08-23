use std::collections::{BTreeSet, BTreeMap};
use std::fmt;

use serde::de::{
    self, Deserialize, DeserializeSeed, EnumAccess, IntoDeserializer,
    MapAccess, SeqAccess, VariantAccess, Visitor,
};

use ordered_float::OrderedFloat;

use imp::error::{Error};
use value::Value;

impl<'de> Deserialize<'de> for Value {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Value, D::Error>
        where D: serde::Deserializer<'de>
    {
        struct ValueVisitor;

        impl<'de> Visitor<'de> for ValueVisitor {
            type Value = Value;

            #[inline]
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("any valid fressian value")
            }

            #[inline]
            fn visit_bool<E>(self, value: bool) -> Result<Value, E> {
                Ok(Value::BOOL(value))
            }

            #[inline]
            fn visit_i64<E>(self, value: i64) -> Result<Value, E> {
                Ok(Value::INT(value))
            }

            #[inline]
            fn visit_f32<E>(self, value: f32) -> Result<Value, E> {
                Ok(Value::FLOAT(OrderedFloat::from(value)))
            }

            #[inline]
            fn visit_f64<E>(self, value: f64) -> Result<Value, E> {
                Ok(Value::DOUBLE(OrderedFloat::from(value)))
            }

            #[inline]
            fn visit_str<E>(self, value: &str) -> Result<Value, E>
            where
                E: serde::de::Error,
            {
                self.visit_string(String::from(value))
            }

            #[inline]
            fn visit_string<E>(self, value: String) -> Result<Value, E> {
                Ok(Value::STRING(value))
            }

            #[inline]
            fn visit_none<E>(self) -> Result<Value, E> {
                Ok(Value::NULL)
            }

            #[inline]
            fn visit_some<D>(self, deserializer: D) -> Result<Value, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                // would be nice if fressian had option flag
                Deserialize::deserialize(deserializer)
            }

            #[inline]
            fn visit_unit<E>(self) -> Result<Value, E> {
                Ok(Value::NULL)
            }

            #[inline]
            fn visit_seq<V>(self, mut visitor: V) -> Result<Value, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let mut vec = Vec::new();

                while let Some(elem) = try!(visitor.next_element()) {
                    vec.push(elem);
                }

                Ok(Value::LIST(vec)) // list map set
            }

            #[inline]
            fn visit_map<V>(self, mut visitor: V) -> Result<Value, V::Error>
            where
                V: MapAccess<'de>,
            {
                // let mut values = Map::new();
                let mut values: BTreeMap<Value, Value> = BTreeMap::new();

                while let Some((key, value)) = try!(visitor.next_entry()) {
                    values.insert(key, value);
                }

                Ok(Value::MAP(values))
            }
        }

        deserializer.deserialize_any(ValueVisitor)
    }
}
