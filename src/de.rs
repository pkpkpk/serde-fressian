use std::ops::{AddAssign, MulAssign, Neg};

use serde::de::{
    self, Deserialize, DeserializeSeed, EnumAccess, IntoDeserializer,
    MapAccess, SeqAccess, VariantAccess, Visitor,
};

use imp::error::{Error, Result};
use imp::RawInput::{RawInput};
use imp::codes;

// use uuid::Uuid;


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

}


impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let code = self.read_next_code()?;

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
                visitor.visit_seq(FixedListReader::new(self, length as usize))
            }

            codes::LIST => {
                let length = self.rawIn.read_count()?;
                visitor.visit_seq(FixedListReader::new(self, length as usize))
            }

            codes::BEGIN_CLOSED_LIST => {
                visitor.visit_seq(ClosedListReader::new(self))
            }

            codes::BEGIN_OPEN_LIST => {
                visitor.visit_seq(OpenListReader::new(self))
            }

            //////////////////////////////////////////////////////////////////////

            codes::MAP => {
                let list_code = self.read_next_code()?;
                match list_code as u8 {
                    codes::LIST_PACKED_LENGTH_START..=235 => {
                        let length = list_code as u8 - codes::LIST_PACKED_LENGTH_START;
                        visitor.visit_map(FixedListReader::new(self, length as usize))
                    }

                    codes::LIST => {
                        let length = self.rawIn.read_count()?;
                        visitor.visit_map(FixedListReader::new(self, length as usize))
                    }

                    codes::BEGIN_CLOSED_LIST => {
                        visitor.visit_map(ClosedListReader::new(self))
                    }

                    _ => {
                        Err(Error::Message("malformed LIST body of MAP".to_string()))
                    }
                }
            }

            codes::SET => {
                let list_code = self.read_next_code()?;
                match list_code as u8 {
                    codes::LIST_PACKED_LENGTH_START..=235 => {
                        let length = list_code as u8 - codes::LIST_PACKED_LENGTH_START;
                        visitor.visit_seq(FixedListReader::new(self, length as usize))
                    }

                    codes::LIST => {
                        let length = self.rawIn.read_count()?;
                        visitor.visit_seq(FixedListReader::new(self, length as usize))
                    }

                    codes::BEGIN_CLOSED_LIST => {
                        visitor.visit_seq(ClosedListReader::new(self))
                    }

                    _ => {
                        Err(Error::Message("malformed LIST body of SET".to_string()))
                    }
                }
            }

            //////////////////////////////////////////////////////////////////////

            codes::INST => {
                visitor.visit_i64(self.rawIn.read_int()?) //millisecs
            }



            //////////////////////////////////////////////////////////////////////
            //char
            //footer
            // use num::BigInt;
            // uuid
            // typed arrays
            // records
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




struct FixedListReader<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
    length: usize,
    items_read: usize
}

impl<'a, 'de> FixedListReader<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>, length: usize) -> Self {
        FixedListReader {
            de,
            length: length,
            items_read: 0,
        }
    }

    fn inc_items_read(&mut self) {
        self.items_read += 1;
    }
}


impl<'de, 'a> SeqAccess<'de> for FixedListReader<'a, 'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        if self.items_read >= self.length {
            Ok(None)
        } else {
            seed.deserialize(&mut *self.de)
                .and_then(|v|{
                    self.inc_items_read();
                    Ok(Some(v))
                })
        }
    }
}

impl<'de, 'a> MapAccess<'de> for FixedListReader<'a, 'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: DeserializeSeed<'de>,
    {
        self.next_element_seed(seed)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: DeserializeSeed<'de>,
    {
        match self.next_element_seed(seed) {
            Ok(Some(v)) => {
                Ok(v)
            }
            Ok(None) => {
                Err(Error::Message("premature EOF when trying to deserialize map value".to_string()))
            }
            Err(err) => {
                Err(err)
            }
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

struct ClosedListReader<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
    finished: bool
}

impl<'a, 'de> ClosedListReader<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>) -> Self {
        ClosedListReader { de, finished: false }
    }
}

impl<'de, 'a> SeqAccess<'de> for ClosedListReader<'a, 'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        if self.finished {
            Err(Error::Message("attempted reading past list end".to_string()))
        } else if self.de.rawIn.peek_next_code()? as u8 == codes::END_COLLECTION{
            self.finished = true;
            let _ = self.de.read_next_code()?;
            Ok(None)
        } else {
            seed.deserialize(&mut *self.de).map(Some)
        }
    }
}

impl<'de, 'a> MapAccess<'de> for ClosedListReader<'a, 'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: DeserializeSeed<'de>,
    {
        self.next_element_seed(seed)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: DeserializeSeed<'de>,
    {
        match self.next_element_seed(seed) {
            Ok(Some(v)) => {
                Ok(v)
            }
            Ok(None) => {
                Err(Error::Message("premature EOF when trying to deserialize map value".to_string()))
            }
            Err(err) => {
                Err(err)
            }
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

struct OpenListReader<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
    finished: bool
}

impl<'a, 'de> OpenListReader<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>) -> Self {
        OpenListReader { de, finished: false }
    }
}

impl<'de, 'a> SeqAccess<'de> for OpenListReader<'a, 'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        if self.finished {
            return Err(Error::Message("attempted reading past list end".to_string()))
        };

        let next_code = self.de.rawIn.peek_next_code();

        match next_code {
            Ok(code) => {
                if code as u8 == codes::END_COLLECTION {
                    self.finished = true;
                    let _ = self.de.read_next_code()?;
                    Ok(None)
                } else {
                    seed.deserialize(&mut *self.de).map(Some)
                }
            }
            Err(Error::Eof) => {
                self.finished = true;
                Ok(None)
            }
            Err(err) => {
                Err(err)
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
