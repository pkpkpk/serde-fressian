use serde::de::{
    self, Deserialize, DeserializeSeed, EnumAccess, IntoDeserializer,
    MapAccess, SeqAccess, VariantAccess, Visitor,
};

use imp::io::{ByteReader};
use error::{Error, ErrorCode, Result};
use imp::RawInput::{RawInput};
use imp::codes;
use value::{self, Value};
use std::collections::{HashMap};

pub struct Deserializer<'de>{
    rdr: ByteReader<'de>,
    rawIn: RawInput,
    cache_next: bool,
    priority_cache: Vec<Value>
}

fn error<T>(de: &Deserializer, reason: ErrorCode) -> Result<T>
{
    let position: usize = de.rdr.get_bytes_read();
    Err(Error::syntax(reason, position))
}

impl<'de> Deserializer<'de>
{
    pub fn from_bytes(bytes: &'de [u8]) -> Self {
        Deserializer {
            rdr: ByteReader::new(bytes),
            rawIn: RawInput,
            cache_next: false,
            priority_cache: Vec::<Value>::new()
        }
    }

    pub fn from_vec(v: &'de Vec<u8>) -> Self {
        Deserializer::from_bytes(v.as_slice())
    }

    /// abstract out as fressian reader
    pub fn read_next_code(&mut self) -> Result<i8> {
        RawInput.read_next_code(&mut self.rdr)
    }

    fn peek_next_code(&mut self) -> Result<i8> {
        RawInput.peek_next_code(&mut self.rdr)
    }

    fn add_priority_cache(&mut self, value: Value) {
        self.priority_cache.push(value)
    }

    fn get_priority_cache(&self, index: usize) -> Option<&Value> {
        self.priority_cache.get(index)
    }

    // reset
}

pub fn from_bytes<'a, T>(s: &'a [u8]) -> Result<T>
where
    T: de::Deserialize<'a>,
{
    let mut deserializer = Deserializer::from_bytes(s);
    T::deserialize(&mut deserializer)
}

pub fn from_vec<'a, T>(v: &'a Vec<u8>) -> Result<T>
    where T: Deserialize<'a>,
{
    let mut deserializer = Deserializer::from_bytes(v.as_slice());
    T::deserialize(&mut deserializer)
}

use std::io;

pub fn from_reader<R, T>(mut rdr: R) -> Result<T>
where
    R: io::Read,
    T: de::DeserializeOwned,
{
    let mut bytes = Vec::new();
    match rdr.read_to_end(&mut bytes){
        Ok(_) => from_bytes(&bytes),
        Err(e) => Err(Error::io(e))
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
                visitor.visit_bool(self.rawIn.read_boolean_code(&mut self.rdr, code)?)
            }
            codes::INT | 0x00..=0x7f | 0xFF => {
                visitor.visit_i64(self.rawIn.read_int_code(&mut self.rdr, code)?)
            }

            codes::FLOAT => {
                visitor.visit_f32(self.rawIn.read_float_code(&mut self.rdr, code)?)
            }

            codes::DOUBLE | codes::DOUBLE_0 | codes::DOUBLE_1  => {
                visitor.visit_f64(self.rawIn.read_double_code(&mut self.rdr, code)?)
            }

            codes::BYTES | codes::BYTES_PACKED_LENGTH_START..=215 => {
                visitor.visit_bytes(self.rawIn.read_bytes_code(&mut self.rdr, code)?)
            }

            codes::BYTES_CHUNK => {
                error(self, ErrorCode::UnsupportedType) ///////////////////////////////////////////
            }

            codes::STRING_PACKED_LENGTH_START..=225 => {
                let length = code as u8 - codes::STRING_PACKED_LENGTH_START;
                let string: String = self.rawIn.read_fressian_string(&mut self.rdr, length as usize)?;

                if self.cache_next {
                    self.cache_next = false;
                    self.add_priority_cache(Value::STRING(string.clone()))
                }

                visitor.visit_string(string)
            }

            codes::STRING => {
                let length = self.rawIn.read_count(&mut self.rdr)?;
                let string: String = self.rawIn.read_fressian_string(&mut self.rdr, length as usize)?;

                if self.cache_next {
                    self.cache_next = false;
                    self.add_priority_cache(Value::STRING(string.clone()))
                }

                visitor.visit_string(string)
            }

            codes::STRING_CHUNK => {
                error(self, ErrorCode::UnsupportedType)////////////////////////////////////////////
            }

            codes::UTF8 => {
                let length = self.rawIn.read_count(&mut self.rdr)?;
                let string = self.rawIn.read_raw_utf8(&mut self.rdr, length as usize)?;

                if self.cache_next {
                    self.cache_next = false;
                    self.add_priority_cache(Value::STRING(string.clone()))
                }

                visitor.visit_string(string)
            }

            codes::LIST_PACKED_LENGTH_START..=235 => {
                let length = code as u8 - codes::LIST_PACKED_LENGTH_START;
                visitor.visit_seq(FixedListReader::new(self, length as usize))
            }

            codes::LIST => {
                let length = self.rawIn.read_count(&mut self.rdr)?;
                visitor.visit_seq(FixedListReader::new(self, length as usize))
            }

            codes::BEGIN_CLOSED_LIST => {
                visitor.visit_seq(ClosedListReader::new(self))
            }

            codes::BEGIN_OPEN_LIST => {
                visitor.visit_seq(OpenListReader::new(self))
            }

            codes::MAP => {
                let list_code = self.read_next_code()?;
                match list_code as u8 {
                    codes::LIST_PACKED_LENGTH_START..=235 => {
                        let length = list_code as u8 - codes::LIST_PACKED_LENGTH_START;
                        visitor.visit_map(FixedListReader::new(self, length as usize))
                    }

                    codes::LIST => {
                        let length = self.rawIn.read_count(&mut self.rdr)?;
                        visitor.visit_map(FixedListReader::new(self, length as usize))
                    }

                    codes::BEGIN_CLOSED_LIST => {
                        visitor.visit_map(ClosedListReader::new(self))
                    }

                    _ => error(self, ErrorCode::MapExpectedListCode)

                }
            }

            codes::SET => {
                visit_list(self, visitor)
            }

            codes::INST => {
                visitor.visit_i64(self.rawIn.read_int(&mut self.rdr)?)
            }

            codes::UUID => {
                visitor.visit_bytes(self.rawIn.read_bytes(&mut self.rdr)?)
            }

            codes::URI => {
                // Url crate wants &str
                visitor.visit_string(self.rawIn.read_string(&mut self.rdr)?)
            }

            codes::REGEX => {
                visitor.visit_string(self.rawIn.read_string(&mut self.rdr)?)
            }

            codes::SYM => {
                // expect  PUT_PRIORITY_CACHE | STRING | PUT_PRIORITY_CACHE | STRING
                visitor.visit_seq(FixedListReader::new(self, 2))
            }

            codes::KEY => {
                // expect  PUT_PRIORITY_CACHE | STRING | PUT_PRIORITY_CACHE | STRING
                visitor.visit_seq(FixedListReader::new(self, 2)) //////////////
            }

            codes::OBJECT_ARRAY => {
                error(self, ErrorCode::UnsupportedType) //// discard?
            }

            codes::INT_ARRAY
            | codes::LONG_ARRAY
            | codes::FLOAT_ARRAY
            | codes::DOUBLE_ARRAY
            | codes::BOOLEAN_ARRAY
            => {
                let length = self.rawIn.read_count(&mut self.rdr)?;
                visitor.visit_seq(FixedListReader::new(self, length as usize))
            }

            codes::PUT_PRIORITY_CACHE => {
                self.cache_next = true;
                self.deserialize_any(visitor)
            }

            codes::PRIORITY_CACHE_PACKED_START..=159 => {
                let index = code as u8 - codes::PRIORITY_CACHE_PACKED_START;
                if let Some(ref val) = self.get_priority_cache(index as usize) {
                    match val {
                        Value::STRING(s) => {
                            visitor.visit_string(s.clone())
                        }
                        _ => Err(Error::msg("unsupported cached Value type".to_string())) //need value formatting
                    }
                } else {
                    Err(Error::msg("missing cached object".to_string()))
                }
            }


            _ => error(self, ErrorCode::UnmatchedCode(code as u8)),
        }
    }

    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf unit unit_struct seq map struct identifier ignored_any
        enum newtype_struct
        // option tuple tuple_struct
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        match self.peek_next_code()? as u8 {
            codes::NULL => {
                let _ = self.read_next_code()?;
                visitor.visit_none()
            }
            _ => {
                visitor.visit_some(self)
            }
        }
    }

    fn deserialize_tuple_struct<V>(self, name: &'static str, len: usize, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        match name {
            "CODE" => {
                // this exists for deserialize Value lookahead
                //this will choke on cache codes, need to peek until next value code ///////////////////////////////
                visitor.visit_i8(self.peek_next_code()?)
            }
            _ => self.deserialize_seq(visitor)
        }
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value>
        where V: Visitor<'de>,
    {
        //peeking the code increments count by 1
        // so a simple CODE:DATA pair is length 2
        // accomodating CODE:DATA:DATA (sym, kw,) etc needs longer
        // is there a reason to cap? reason not to?
        visitor.visit_seq(FixedListReader::new(self, 3))
    }
}

////////////////////////////////////////////////////////////////////

fn visit_list<'a, 'de, V>(de: &'a mut Deserializer<'de>, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
{
    let list_code = de.read_next_code()?;
    match list_code as u8 {
        codes::LIST_PACKED_LENGTH_START..=235 => {
            let length = list_code as u8 - codes::LIST_PACKED_LENGTH_START;
            visitor.visit_seq(FixedListReader::new(de, length as usize))
        }

        codes::LIST => {
            let length = de.rawIn.read_count(&mut de.rdr)?;
            visitor.visit_seq(FixedListReader::new(de, length as usize))
        }

        codes::BEGIN_CLOSED_LIST => {
            visitor.visit_seq(ClosedListReader::new(de))
        }

        codes::BEGIN_OPEN_LIST => {
            visitor.visit_seq(OpenListReader::new(de))
        }

        _ => error(de, ErrorCode::ExpectedListCode)
    }
}


struct FixedListReader<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
    length: usize,
    items_read: usize,
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
            Ok(None) => error(self.de, ErrorCode::UnexpectedEof),

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
            error(self.de, ErrorCode::AttemptToReadPastEnd)
        } else if self.de.peek_next_code()? as u8 == codes::END_COLLECTION{
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
            Ok(None) => error(self.de, ErrorCode::UnexpectedEof),

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
            return error(self.de, ErrorCode::AttemptToReadPastEnd)
        };

        let next_code = self.de.peek_next_code();

        if next_code.is_ok() {
            let code = next_code.unwrap();
            if code as u8 == codes::END_COLLECTION {
                self.finished = true;
                let _ = self.de.read_next_code()?;
                Ok(None)
            } else {
                seed.deserialize(&mut *self.de).map(Some)
            }
        } else {
            let err = next_code.unwrap_err();
            if err.is_eof() {
                self.finished = true;
                Ok(None)
            } else {
                Err(err)
            }
        }
    }
}


