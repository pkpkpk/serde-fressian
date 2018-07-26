extern crate serde;

use serde::ser::{self, Serialize};
use imp::RawOutput::*;
use imp::Codes;
use imp::ranges;
use std::cmp;

mod error {
    pub use serde::de::value::Error;
    pub type Result<T> = ::std::result::Result<T, Error>;
}

use self::error::{Error, Result};

fn bit_switch(l: i64) -> u8 {
    if l < 0 {
        (!l).leading_zeros() as u8
    } else {
        l.leading_zeros() as u8
    }
}

fn add_byte_at_index(v: &mut Vec<u8>, index: &mut usize, byte: u8){
    let length = v.len();
    if *index <= length {
        v.push(byte);
        *index += 1;
    } else {
        assert!(*index < length);
        v[*index] = byte;
        *index += 1;
    }
}

fn encoding_size(ch: u32) -> usize {
    if ch <= 0x007f{
        return 1;
    } else if ch > 0x07ff {
        return 3;
    } else {
        return 2;
    }
}

// used by write-string to pack each utf8 char
fn write_char(ch: u32, buffer: &mut Vec<u8>, buf_pos: &mut usize){
    match encoding_size(ch) {
        1 => {
            add_byte_at_index(buffer, buf_pos, ch as u8);
        },
        2 => {
            add_byte_at_index(buffer, buf_pos, (0xc0 | ch as u32 >> 6 & 0x1f) as u8);
            add_byte_at_index(buffer, buf_pos, (0x80 | ch as u32 >> 0 & 0x3f) as u8);
        },
        _ => {
            add_byte_at_index(buffer, buf_pos, (0xe0 | ch as u32 >> 12 & 0x0f) as u8);
            add_byte_at_index(buffer, buf_pos, (0x80 | ch as u32 >>  6 & 0x3f) as u8);
            add_byte_at_index(buffer, buf_pos, (0x80 | ch as u32 >>  0 & 0x3f) as u8);
        }
    }
}

pub struct Serializer {
    rawOut: RawOutput
}

impl Serializer{
    pub fn new(out: Vec<u8>) -> Serializer {
        Serializer {
            rawOut: RawOutput::new(out),
        }
    }

    pub fn get_bytes_written(&self) -> u64 {
        self.rawOut.getBytesWritten()
    }

    pub fn write_footer(&mut self) -> Result<()> {
        let length = self.get_bytes_written();
        // self.clear_caches()
        self.rawOut.writeRawInt32(Codes::FOOTER_MAGIC as i32)?;
        self.rawOut.writeRawInt32(length as i32)?; //////////////////////////////
        let checksum = 0; //rawOut.getChecksum().getValue()
        self.rawOut.writeRawInt32(checksum)
        // self.reset();
    }

    pub fn into_inner(self) -> Vec<u8> { self.rawOut.into_inner() }

    /// Get ref to out
    pub fn get_ref(&self) -> &Vec<u8> {
        &self.rawOut.get_ref()
    }

    pub fn reset(&mut self) {
        self.rawOut.reset();
    }

    ////////////////////////////////////////////////////////////////////////////

    fn write_code(&mut self, code: u8) -> Result<()> {
        self.rawOut.write_raw_byte(code)
    }

    pub fn write_count(&mut self, count: usize) -> Result<()> {
        self.write_int(count as i64)
    }

    pub fn write_null(&mut self) -> Result<()> {
        self.write_code(Codes::NULL)
    }

    pub fn begin_open_list(&mut self) -> Result<()> {
        // if (0 != rawOut.getBytesWritten())
        //     throw new IllegalStateException("openList must be called from the top level, outside any footer context.");
        self.write_code(Codes::BEGIN_OPEN_LIST)
    }

    pub fn begin_closed_list(&mut self) -> Result<()> {
        self.write_code(Codes::BEGIN_CLOSED_LIST)
    }

    pub fn end_list(&mut self) -> Result<()> {
        self.write_code(Codes::END_COLLECTION)
    }

    pub fn write_boolean(&mut self, b: bool) -> Result<()> {
        if b {
            self.write_code(Codes::TRUE)
        } else {
            self.write_code(Codes::FALSE)
        }
    }


    pub fn write_float(&mut self, f: f32) -> Result<()> {
        self.write_code(Codes::FLOAT)?;
        self.rawOut.writeRawFloat(f)
    }

    pub fn write_double(&mut self, f: f64) -> Result<()> {
        if f == 0.0 {
            self.write_code(Codes::DOUBLE_0)
        } else if f == 1.0 {
            self.write_code(Codes::DOUBLE_1)
        } else {
            self.write_code(Codes::DOUBLE)?;
            self.rawOut.writeRawDouble(f)
        }
    }

    pub fn write_int(&mut self, i: i64) -> Result<()> {
        match bit_switch(i) {
            1..=14 => {
                self.write_code(Codes::INT)?;
                self.rawOut.writeRawInt64(i)
            }

            15..=22 => {
                self.rawOut.write_raw_byte(Codes::INT_PACKED_7_ZERO.wrapping_add( (i >> 48) as u8 ))?;
                self.rawOut.writeRawInt48(i)
            }

            23..=30 => {
                self.rawOut.write_raw_byte(Codes::INT_PACKED_6_ZERO.wrapping_add( (i >> 40) as u8 ))?;
                self.rawOut.writeRawInt40(i)
            }

            31..=38 => {
                self.rawOut.write_raw_byte(Codes::INT_PACKED_5_ZERO.wrapping_add( (i >> 32) as u8 ))?;
                self.rawOut.writeRawInt32(i as i32)
            }

            39..=44 => {
                self.rawOut.write_raw_byte(Codes::INT_PACKED_4_ZERO.wrapping_add( (i >> 24) as u8))?;
                self.rawOut.writeRawInt24(i as i32)
            }

            45..=51 => {
                self.rawOut.write_raw_byte(Codes::INT_PACKED_3_ZERO.wrapping_add( (i >> 16) as u8))?;
                self.rawOut.writeRawInt16(i as i32)
            }

            52..=57 => {
                self.rawOut.write_raw_byte(Codes::INT_PACKED_2_ZERO.wrapping_add( (i >> 8) as u8))?;
                self.rawOut.write_raw_byte(i as u8)
            }

            58..=64 => {
                if i < -1 {
                    self.rawOut.write_raw_byte(Codes::INT_PACKED_2_ZERO.wrapping_add( (i >> 8) as u8))?;
                    self.rawOut.write_raw_byte(i as u8)
                } else {
                    self.rawOut.write_raw_byte(i as u8)
                }
            }

            _ => Err(serde::de::Error::custom("more than 64 bits in a long!"))
        }
    }

    pub fn write_string(&mut self, s: &str) -> Result<()> {
        let CHAR_LENGTH: usize = s.chars().count();

        if CHAR_LENGTH == 0 {
            self.rawOut.write_raw_byte(Codes::STRING_PACKED_LENGTH_START);
        } else {
            // chars > 0xFFFF are actually 2 chars in java, need a separate string length
            // to write the appropriate code into the bytes
            let mut JCHAR_LENGTH = CHAR_LENGTH;
            let mut string_pos: usize = 0;
            let mut jstring_pos: usize = 0;
            let mut iter = itertools::put_back(s.chars());

            // let maxBufNeeded: usize = cmp::min(65536, CHAR_LENGTH * 3);
            // ^ silently fails, should be using char count. compiler bug?
            let maxBufNeeded: usize = cmp::min(65536, s.len() * 3);
            let mut buffer: Vec<u8> = Vec::with_capacity(maxBufNeeded); //abstract out into stringbuffer, re-use

            while string_pos < CHAR_LENGTH {
                let mut buf_pos = 0;
                loop {

                    let ch: Option<char> = iter.next();

                    match ch {
                        Some(ch) => {
                            let encodingSize = encoding_size(ch as u32);

                            if (buf_pos + encodingSize) < maxBufNeeded {
                                if 0xFFFF < ch as u32 {
                                    // must emulate java chars:
                                    // supplementary characters are represented as a pair of char values
                                    //  - the high-surrogates range, (\uD800-\uDBFF)
                                    //  - the low-surrogates range (\uDC00-\uDFFF)
                                    let mut utf16_bytes: Vec<u16> =  vec![0; 2];
                                    ch.encode_utf16(&mut utf16_bytes);
                                    write_char(utf16_bytes[0] as u32, &mut buffer, &mut buf_pos);
                                    write_char(utf16_bytes[1] as u32, &mut buffer, &mut buf_pos);
                                    string_pos += 1; //a 1 rust char...
                                    jstring_pos += 2; // equivalent to eating 2 java chars
                                    JCHAR_LENGTH += 1; // track extra java char we created
                                    continue;
                                } else {
                                    write_char(ch as u32, &mut buffer, &mut buf_pos);
                                    string_pos += 1;
                                    jstring_pos += 1;
                                    continue;
                                }
                            } else {
                                iter.put_back(ch);
                                break;
                            }
                        }
                        None  => { break }
                    }
                }
                if buf_pos < ranges::STRING_PACKED_LENGTH_END {
                    self.rawOut.write_raw_byte(Codes::STRING_PACKED_LENGTH_START.wrapping_add( buf_pos as u8))?;
                } else if jstring_pos == JCHAR_LENGTH {
                    self.write_code(Codes::STRING)?;
                    self.write_count(buf_pos)?;
                } else {
                    self.write_code(Codes::STRING_CHUNK)?;
                    self.write_count(buf_pos)?;
                }
                self.rawOut.write_raw_bytes(&buffer,0,buf_pos)?;
            }
        }

        Ok(())
    }

    pub fn write_bytes(&mut self, bytes: &Vec<u8>, offset: usize, length: usize) -> Result<()> {
        if length < ranges::BYTES_PACKED_LENGTH_END {
            self.rawOut.write_raw_byte(Codes::BYTES_PACKED_LENGTH_START + length as u8)?;
            self.rawOut.write_raw_bytes(bytes, offset,length)
        } else {
            let mut length = length;
            let mut offset = offset;
            while ranges::BYTE_CHUNK_SIZE < length {
                self.write_code(Codes::BYTES_CHUNK)?;
                self.write_count(ranges::BYTE_CHUNK_SIZE)?;
                self.rawOut.write_raw_bytes(bytes, offset, ranges::BYTE_CHUNK_SIZE)?;
                offset += ranges::BYTE_CHUNK_SIZE;
                length -= ranges::BYTE_CHUNK_SIZE;
            };
            self.write_code(Codes::BYTES)?;
            self.write_count(length)?;
            self.rawOut.write_raw_bytes(bytes, offset, length)
        }
    }
}

pub fn to_vec<T>(value: &T) -> Result<Vec<u8>>
where
    T: Serialize,
{
    let buf = Vec::with_capacity(100);
    let mut serializer = Serializer {
        rawOut: RawOutput::new(buf)
    };

    value.serialize(&mut serializer)?;
    Ok(serializer.rawOut.into_inner())
}


impl<'a> ser::Serializer for &'a mut Serializer{
    type Ok = ();
    type Error = Error;
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<()> {
        self.write_boolean(v)
    }

    fn serialize_i8(self, v: i8) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i16(self, v: i16) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i32(self, v: i32) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i64(self, v: i64) -> Result<()> {
        self.write_int(v)
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u16(self, v: u16) -> Result<()> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u32(self, v: u32) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_u64(self, v: u64) -> Result<()> {/////////////////////////////////////////////////
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<()> {
        self.write_float(v)
    }

    fn serialize_f64(self, v: f64) -> Result<()> {
        self.write_double(v)
    }

    fn serialize_char(self, v: char) -> Result<()> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_str(self, v: &str) -> Result<()> {
        println!("serializing string::{}",v);
        self.write_string(&v.to_string())
    }

    fn serialize_bytes(self, bytes: &[u8]) -> Result<()> {
        self.write_bytes(&bytes.to_vec(), 0, bytes.len())
    }

    // An absent optional is represented as the JSON `null`.
    fn serialize_none(self) -> Result<()> {
        self.serialize_unit()
    }

    // A present optional is represented as just the contained value. Note that
    // this is a lossy representation. For example the values `Some(())` and
    // `None` both serialize as just `null`.
    fn serialize_some<S>(self, value: &S) -> Result<()>
    where
        S: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    // In Serde, unit means an anonymous value containing no data
    fn serialize_unit(self) -> Result<()> {
        self.write_null()
    }

    // Unit struct means a named value containing no data.
    // There is no need to serialize the name in most formats.
    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        self.serialize_unit()
    }


    // When serializing a unit variant (or any other kind of variant), formats
    // can choose whether to keep track of it by index or by name. Binary
    // formats typically use the index of the variant and human-readable formats
    // typically use the name.
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<()> {
        self.serialize_str(variant)
    }


    // As is done here, serializers are encouraged to treat newtype structs as
    // insignificant wrappers around the data they contain.
    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }


    // Note that newtype variant (and all of the other variant serialization
    // methods) refer exclusively to the "externally tagged" enum
    // representation.
    //
    // Serialize this to JSON in externally tagged form as `{ NAME: VALUE }`.
    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        variant.serialize(&mut *self)?;
        value.serialize(&mut *self)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        let length = _len.unwrap();
        if (length as u8) < ranges::LIST_PACKED_LENGTH_END {
            self.rawOut.write_raw_byte(Codes::LIST_PACKED_LENGTH_START.wrapping_add( length as u8))?;
        } else{
            self.write_code(Codes::LIST)?;
            self.write_count(length)?;
        }
        Ok(self)
    }

    // Tuples look just like sequences in JSON. Some formats may be able to
    // represent tuples more efficiently by omitting the length, since tuple
    // means that the corresponding `Deserialize implementation will know the
    // length without needing to look at the serialized data.
    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        self.serialize_seq(Some(len))
    }


    // Tuple structs look just like sequences in JSON.
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        self.serialize_seq(Some(len))
    }


    // Tuple variants are represented in JSON as `{ NAME: [DATA...] }`. Again
    // this method is only responsible for the externally tagged representation.
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        variant.serialize(&mut *self)?;
        Ok(self)
    }


    // Maps are represented in JSON as `{ K: V, K: V, ... }`.
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        self.write_code(Codes::MAP)?;
        let length = 2 * _len.unwrap();
        if (length as u8) < ranges::LIST_PACKED_LENGTH_END {
            self.rawOut.write_raw_byte(Codes::LIST_PACKED_LENGTH_START.wrapping_add( length as u8))?;
        } else{
            self.write_code(Codes::LIST)?;
            self.write_count(length)?;
        }
        Ok(self)
    }

    // Structs look just like maps in JSON. In particular, JSON requires that we
    // serialize the field names of the struct. Other formats may be able to
    // omit the field names when serializing structs because the corresponding
    // Deserialize implementation is required to know what the keys are without
    // looking at the serialized data.
    fn serialize_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct> {
        self.serialize_map(Some(len))
    }

    // Struct variants are represented in JSON as `{ NAME: { K: V, ... } }`.
    // This is the externally tagged representation.
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        variant.serialize(&mut *self)?;
        Ok(self)
    }
}


// The following 7 impls deal with the serialization of compound types like
// sequences and maps. Serialization of such types is begun by a Serializer
// method and followed by zero or more calls to serialize individual elements of
// the compound type and one call to end the compound type.
//
// This impl is SerializeSeq so these methods are called after `serialize_seq`
// is called on the Serializer.
impl<'a> ser::SerializeSeq for &'a mut Serializer {
    // Must match the `Ok` type of the serializer.
    type Ok = ();
    // Must match the `Error` type of the serializer.
    type Error = Error;

    // Serialize a single element of the sequence.
    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    // Close the sequence.
    fn end(self) -> Result<()> { Ok(()) }
}

// Same thing but for tuples.
impl<'a> ser::SerializeTuple for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> { Ok(()) }
}

// Same thing but for tuple structs.
impl<'a> ser::SerializeTupleStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> { Ok(()) }
}

impl<'a> ser::SerializeTupleVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> { Ok(()) }
}

// Some `Serialize` types are not able to hold a key and value in memory at the
// same time so `SerializeMap` implementations are required to support
// `serialize_key` and `serialize_value` individually.
//
// There is a third optional method on the `SerializeMap` trait. The
// `serialize_entry` method allows serializers to optimize for the case where
// key and value are both available simultaneously. In JSON it doesn't make a
// difference so the default behavior for `serialize_entry` is fine.
impl<'a> ser::SerializeMap for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    // The Serde data model allows map keys to be any serializable type. JSON
    // only allows string keys so the implementation below will produce invalid
    // JSON if the key serializes as something other than a string.
    //
    // A real JSON serializer would need to validate that map keys are strings.
    // This can be done by using a different Serializer to serialize the key
    // (instead of `&mut **self`) and having that other serializer only
    // implement `serialize_str` and return an error on any other data type.
    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where T: ?Sized + Serialize,
    {
        key.serialize(&mut **self)
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> { Ok(()) }
}

// Structs are like maps in which the keys are constrained to be compile-time constant strings
impl<'a> ser::SerializeStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where T: ?Sized + Serialize,
    {
        key.serialize(&mut **self)?;
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> { Ok(()) }
}

impl<'a> ser::SerializeStructVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where T: ?Sized + Serialize,
    {
        key.serialize(&mut **self)?;
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> { Ok(()) }
}
