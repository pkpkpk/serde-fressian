use std::ops::{AddAssign, MulAssign, Neg};

use serde::de::{
    self, Deserialize, DeserializeSeed, EnumAccess, IntoDeserializer,
    MapAccess, SeqAccess, VariantAccess, Visitor,
};

use imp::error::{Error, Result};
use imp::RawInput::{RawInput};
use imp::codes;

pub struct Deserializer<'a>{
    rawIn: RawInput<'a>
}

impl<'a> Deserializer<'a> {

    pub fn from_vec(v: &'a Vec<u8>) -> Deserializer {
        Deserializer {
            rawIn: RawInput::from_vec(v)
        }
    }

    pub fn read_next_code(&mut self) -> Result<i8> {
        self.rawIn.read_next_code()
    }

    pub fn read_int(&mut self) -> Result<i64> {
        self.rawIn.read_int()
    }

    // pub fn read_int_code(&mut self, code: i8) -> Result<i64> {
    //     self.rawIn.read_int_code(code)
    // }

    pub fn read_float(&mut self) -> Result<f32> {
        self.rawIn.read_float()
    }

    pub fn read_double(&mut self) -> Result<f64> {
        self.rawIn.read_double()
    }

}



// Deserialization refers to mapping that JSON value into Serde's data
// model by invoking one of the `Visitor` methods. In the case of JSON and
// bool that mapping is straightforward so the distinction may seem silly,
// but in other cases Deserializers sometimes perform non-obvious mappings.
// For example the TOML format has a Datetime type and Serde's data model
// does not. In the `toml` crate, a Datetime in the input is deserialized by
// mapping it to a Serde data model "struct" type with a special name and a
// single field containing the Datetime represented as a string.

// Refer to the "Understanding deserializer lifetimes" page for information
// about the three deserialization flavors of strings in Serde.

// An absent optional is represented as the JSON `null` and a present
// optional is represented as just the contained value.
//
// As commented in `Serializer` implementation, this is a lossy
// representation. For example the values `Some(())` and `None` both
// serialize as just `null`. Unfortunately this is typically what people
// expect when working with JSON. Other formats are encouraged to behave
// more intelligently if possible.

// In Serde, unit means an anonymous value containing no data.

// Unit struct means a named value containing no data.

// As is done here, serializers are encouraged to treat newtype structs as
// insignificant wrappers around the data they contain. That means not
// parsing anything other than the contained value.

// Deserialization of compound types like sequences and maps happens by
// passing the visitor an "Access" object that gives it the ability to
// iterate through the data contained in the sequence.

// Tuples look just like sequences in JSON. Some formats may be able to
// represent tuples more efficiently.
//
// As indicated by the length parameter, the `Deserialize` implementation
// for a tuple in the Serde data model is required to know the length of the
// tuple before even looking at the input data.

// Tuple structs look just like sequences in JSON.

//deserialize_map
// Much like `deserialize_seq` but calls the visitors `visit_map` method
// with a `MapAccess` implementation, rather than the visitor's `visit_seq`
// method with a `SeqAccess` implementation.

//fn deserialize_struct
// Structs look just like maps in JSON.
//
// Notice the `fields` parameter - a "struct" in the Serde data model means
// that the `Deserialize` implementation is required to know what the fields
// are before even looking at the input data. Any key-value pairing in which
// the fields cannot be known ahead of time is probably a map.

// An identifier in Serde is the type that identifies a field of a struct or
// the variant of an enum. In JSON, struct fields and enum variants are
// represented as strings. In other formats they may be represented as
// numeric indices.

//fn deserialize_ignored_any
// Like `deserialize_any` but indicates to the `Deserializer` that it makes
// no difference which `Visitor` method is called because the data is
// ignored.
//
// Some deserializers are able to implement this more efficiently than
// `deserialize_any`, for example by rapidly skipping over matched
// delimiters without paying close attention to the data in between.
//
// Some formats are not able to implement this at all. Formats that can
// implement `deserialize_any` and `deserialize_ignored_any` are known as
// self-describing.

// /// Represents a JSON value
#[derive(Clone, PartialEq, PartialOrd)]
pub enum FressianValue {
    Null,
    Bool(bool),
    Int(i64),
    Float(f32),
    Double(f32),
    String(String),
    // Str(&str),
    // Array(Vec<Value>),
    // Object(BTreeMap<String, Value>),
}

// const PACKED_LIST_RANGE: std::ops::RangeInclusive<u8> = {
//     let end = codes::LIST_PACKED_LENGTH_START + 7;
//     codes::LIST_PACKED_LENGTH_START..=end
// }; 234

const PACKED_LIST_RANGE: std::ops::RangeInclusive<u8> =  codes::LIST_PACKED_LENGTH_START..=235;


impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let code = self.read_next_code()?;

        println!("code: {}", code);

        match code as u8 {
            codes::NULL => {
                visitor.visit_unit()
            }

            codes::TRUE | codes::FALSE => {
                visitor.visit_bool(self.rawIn.read_boolean_code(code)?)
            }
            codes::INT | 0x00..=0x7f | 0xFF => {
                visitor.visit_i64(self.rawIn.read_int_code(code)?)
            }

            codes::FLOAT => {
                visitor.visit_f32(self.rawIn.read_float_code(code)?)
            }

            codes::DOUBLE | codes::DOUBLE_0 | codes::DOUBLE_1  => {
                visitor.visit_f64(self.rawIn.read_double_code(code)?)
            }

            codes::BYTES | codes::BYTES_PACKED_LENGTH_START..=215 => {
                visitor.visit_bytes(self.rawIn.read_bytes_code(code)?)
            }

            // codes::BYTES_CHUNK => {///////////////////////////////////////////////////////////////
            //     self.internal_read_chunked_bytes()
            // }

            codes::STRING_PACKED_LENGTH_START..=225 => {
                let length = code as u8 - codes::STRING_PACKED_LENGTH_START;
                visitor.visit_string(self.rawIn.read_fressian_string(length as usize)?)
            }

            codes::STRING => {
                let length = self.rawIn.read_count()?;
                visitor.visit_string(self.rawIn.read_fressian_string(length as usize)?)
            }

            // codes::STRING_CHUNK => {///////////////////////////////////////////////////////////////
            //
            // }

            codes::UTF8 => {
                let length = self.rawIn.read_count()?;
                visitor.visit_str(self.rawIn.read_raw_utf8(length as usize)?)
            }

            //////////////////////////////////////////////////////////////////////

            codes::LIST_PACKED_LENGTH_START..=235 => {
                let length = code as u8 - codes::LIST_PACKED_LENGTH_START;
                println!("got codes::LIST_PACKED, length : {}", length);
                visitor.visit_seq(ListReader::new(self, length as usize))
            }

            codes::LIST => {
                let length = self.rawIn.read_count()?;
                println!("got codes::LIST, length : {}", length);
                visitor.visit_seq(ListReader::new(self, length as usize))
            }

            //char
            //put cache, get cache, PRIORITY_CACHE_PACKED_START...

            _ => Err(Error::UnmatchedCode(code as u8)),
        }
    }

    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct
         seq
         tuple
        tuple_struct map struct enum identifier ignored_any
    }

}


//need open/closed list reading
struct ListReader<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
    length: usize,
    items_read: usize
}

impl<'a, 'de> ListReader<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>, length: usize) -> Self {
        ListReader {
            de,
            length: length,
            items_read: 0,
        }
    }

    fn inc_items_read(&mut self) {
        self.items_read += 1;
    }
}


impl<'de, 'a> SeqAccess<'de> for ListReader<'a, 'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        //only applicable for sized lists. open frame lists need separate logic
        if self.items_read >= self.length {
            Ok(None)
        } else {
            match seed.deserialize(&mut *self.de) {
                Ok(v) => {
                    self.inc_items_read();
                    Ok(Some(v))
                }
                Err(e) => {
                    Err(e)
                }
            }
        }
    }
}


///////////////////////////////////////////////////////////////////////////////////////////////////

pub fn from_vec<'a, T>(v: &'a Vec<u8>) -> Result<T>
where
    T: Deserialize<'a>,
{
    let mut deserializer = Deserializer::from_vec(v);
    T::deserialize(&mut deserializer)
}
