use serde::ser::{self, Serialize};

use std::cmp;

use imp::RawOutput::*;
use imp::codes;
use imp::ranges;
use imp::error::{Error, Result};
use imp::io::{ByteWriter, IWriteBytes};

pub struct Serializer<W>{
    writer: W,
    rawOut: RawOutput
}

impl<W> Serializer<W>
where
    W: IWriteBytes,
{
    pub fn new(writer: W) -> Self {
        Serializer {
            writer: writer,
            rawOut: RawOutput
        }
    }
}

impl Serializer<ByteWriter<Vec<u8>>> {
    pub fn from_vec(v: Vec<u8>) -> Self {
        Serializer {
            writer: ByteWriter::from_vec(v),
            rawOut: RawOutput
        }
    }

    pub fn reset(&mut self) {
        self.writer.reset()
    }

    pub fn get_ref(&self) -> &Vec<u8> {
        self.writer.get_ref()
    }

    pub fn to_vec(&mut self) -> Vec<u8> {
        self.writer.to_vec()
    }
}

// write a value to bytes, skipping writer creation
pub fn to_vec<T>(value: &T) -> Result<Vec<u8>>
where
    T: Serialize,
{
    let buf = Vec::with_capacity(100);
    let mut serializer = Serializer::from_vec(buf);

    value.serialize(&mut serializer)?;
    Ok(serializer.to_vec())
}


pub trait FressianWriter {

    fn write_code(&mut self, code: u8 ) -> Result<()>;

    fn write_count(&mut self, count: usize) -> Result<()>;

    fn write_int(&mut self, i: i64 ) -> Result<()>;

    fn write_null(&mut self) -> Result<()>;

    fn write_boolean(&mut self, b: bool) -> Result<()>;

    fn write_float(&mut self, f: f32) -> Result<()>;

    fn write_double(&mut self, f: f64) -> Result<()>;

    fn write_bytes(&mut self, bytes: &[u8], offset: usize, length: usize) -> Result<()>;

    fn write_string(&mut self, s: &str) -> Result<()>;

    // fn write_footer(&mut self) -> Result<()>;

    fn begin_open_list(&mut self) -> Result<()>;

    fn begin_closed_list(&mut self) -> Result<()>;

    fn end_list(&mut self) -> Result<()>;

    fn write_list_header(&mut self, length: usize) -> Result<()>;

    fn write_list<I>(&mut self, iter: I) -> Result<()>
    where
        I: IntoIterator,
        <I as IntoIterator>::Item: Serialize;

    fn write_map<K,V,I>(&mut self, iter:I) -> Result<()>
    where K: Serialize,
          V: Serialize,
          I: IntoIterator<Item = (K,V)>;

    fn write_set<I, V>(&mut self, iter: I) -> Result<()>
    where V: Serialize + std::cmp::Eq + std::hash::Hash,
          I: IntoIterator<Item = V>;
}


impl<W> FressianWriter for Serializer<W>
where
    W: IWriteBytes,
{

    fn write_code(&mut self, code: u8 ) -> Result<()>
    {
        self.rawOut.write_code(&mut self.writer, code)
    }

    fn write_count(&mut self, count: usize) -> Result<()> {
        self.rawOut.write_int(&mut self.writer, count as i64)
    }

    fn write_int(&mut self, i: i64 ) -> Result<()>{
        self.rawOut.write_int(&mut self.writer,i)
    }

    fn write_null(&mut self) -> Result<()> {
        self.rawOut.write_null(&mut self.writer)
    }

    fn write_boolean(&mut self, b: bool) -> Result<()>{
        self.rawOut.write_boolean(&mut self.writer, b)
    }

    fn write_float(&mut self, f: f32) -> Result<()>{
        self.rawOut.write_float(&mut self.writer,f)
    }

    fn write_double(&mut self, f: f64) -> Result<()>{
        self.rawOut.write_double(&mut self.writer,f)
    }

    fn write_bytes(&mut self, bytes: &[u8], offset: usize, length: usize) -> Result<()>{
        self.rawOut.write_bytes(&mut self.writer,bytes,offset,length)
    }

    fn write_string(&mut self, s: &str) -> Result<()> {
        self.rawOut.write_string(&mut self.writer,s)
    }

    // fn write_footer(&mut self) -> Result<()> {
    //     let length = self.rawOut.get_bytes_written();
    //     // self.clear_caches()
    //     self.rawOut.write_raw_i32(&mut self.writer,codes::FOOTER_MAGIC as i32)?;
    //     self.rawOut.write_raw_i32(&mut self.writer,length as i32)?; //?
    //     let checksum = 0; //rawOut.getChecksum().getValue()
    //     self.rawOut.write_raw_i32(&mut self.writer,checksum)
    //     // self.reset();
    // }

    fn begin_open_list(&mut self) -> Result<()> {
        self.write_code(codes::BEGIN_OPEN_LIST)
    }

    fn begin_closed_list(&mut self) -> Result<()> {
        self.write_code(codes::BEGIN_CLOSED_LIST)
    }

    fn end_list(&mut self) -> Result<()> {
        self.write_code(codes::END_COLLECTION)
    }

    fn write_list_header(&mut self, length: usize) -> Result<()>{
        if (length as u8) < ranges::LIST_PACKED_LENGTH_END {
            self.write_code(codes::LIST_PACKED_LENGTH_START.wrapping_add( length as u8))
        } else {
            self.write_code(codes::LIST)?;
            self.write_count(length)
        }
    }

    fn write_list<I>(&mut self, iter: I) -> Result<()>
    where
        I: IntoIterator,
        <I as IntoIterator>::Item: Serialize,
    {
        let iter = iter.into_iter();
        let len: Option<usize> = iter.len_hint();
        match len {
            Some(length) => {
                self.write_list_header(length)?;
                for item in iter {
                    item.serialize(&mut *self)?;
                }
                Ok(())
            }
            None => {
                self.begin_closed_list()?;
                for item in iter {
                    item.serialize(&mut *self)?;
                }
                self.end_list()?;
                Ok(())
            }
        }
    }

    fn write_map<K,V,I>(&mut self, iter:I) -> Result<()>
    where K: Serialize,
          V: Serialize,
          I: IntoIterator<Item = (K,V)>,
    {
        let iter = iter.into_iter();
        let len: Option<usize> = iter.len_hint();
        match len {
            Some(l) => {
                let length = l * 2;
                self.write_code(codes::MAP)?;
                self.write_list_header(length)?;
                for (k,v) in iter {
                    k.serialize(&mut *self)?;
                    v.serialize(&mut *self)?;
                }
                Ok(())
            }
            None => {
                self.write_code(codes::MAP)?;
                self.begin_closed_list()?;
                for (k,v) in iter {
                    k.serialize(&mut *self)?;
                    v.serialize(&mut *self)?;
                }
                self.end_list()?;
                Ok(())
            }
        }
    }

    fn write_set<I, V>(&mut self, iter: I) -> Result<()>
    where V: Serialize + std::cmp::Eq + std::hash::Hash,
          I: IntoIterator<Item = V>,
    {
        self.write_code(codes::SET)?;
        self.write_list(iter)
    }
}

impl<'a, W> ser::Serializer for &'a mut Serializer<W>
where
    W: IWriteBytes,
{
    type Ok = ();
    type Error = Error;
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<()> { self.write_boolean(v) }

    fn serialize_none(self) -> Result<()> { self.write_null() }

    fn serialize_i8(self, v: i8) -> Result<()> { self.write_int(i64::from(v)) }

    fn serialize_i16(self, v: i16) -> Result<()> { self.write_int(i64::from(v)) }

    fn serialize_i32(self, v: i32) -> Result<()> { self.write_int(i64::from(v)) }

    fn serialize_i64(self, v: i64) -> Result<()> { self.write_int(v) }

    fn serialize_u8(self, v: u8) -> Result<()> { self.write_int(i64::from(v)) }

    fn serialize_u16(self, v: u16) -> Result<()> { self.write_int(i64::from(v)) }

    fn serialize_u32(self, v: u32) -> Result<()> { self.write_int(i64::from(v)) }

    fn serialize_u64(self, v: u64) -> Result<()> {/////////////////////////////////////////////////
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<()> { self.write_float(v) }

    fn serialize_f64(self, v: f64) -> Result<()> { self.write_double(v) }

    fn serialize_char(self, v: char) -> Result<()> { self.serialize_str(&v.to_string()) }

    fn serialize_str(self, v: &str) -> Result<()> { self.write_string(v) }

    fn serialize_bytes(self, bytes: &[u8]) -> Result<()> { self.write_bytes(bytes, 0, bytes.len()) }

    fn serialize_some<S>(self, value: &S) -> Result<()>
    where
        S: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<()> { self.write_null() }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> { self.serialize_unit() }

    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        match _name {
            "INST" => {
                self.write_code(codes::INST)?;
                value.serialize(self)
            }
            "UUID" => {
                self.write_code(codes::UUID)?;
                value.serialize(self)
            }
            "URI" => {
                self.write_code(codes::URI)?;
                value.serialize(self)
            }
            _ => value.serialize(self)
        }
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        match _len {
            Some(n) => {
                self.write_list_header(n)?;
                Ok(self)
            }
            None => {
                Err(Error::Message(
                    "cannot use serde::ser::serialize on uncounted sequences at this time.
                     If known to be finite length, use serializer.write_list().
                     If indeterminate length, use serializer.begin_open_list()".to_string()))
            }
        }
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        match _len {
            Some(l) => {
                let length = 2 * l;
                self.write_code(codes::MAP)?;
                self.write_list_header(length)?;
                Ok(self)
            }
            None => {
                Err(Error::Message(
                    "cannot use serde::ser::serialize on uncounted sequences at this time.
                     If known to be finite length, use serializer.write_map().
                     If indeterminate length, write codes::MAP and serializer.begin_open_list()".to_string()))
            }
        }
    }

    fn serialize_struct(self, _name: &'static str, len: usize,) -> Result<Self::SerializeStruct> {
        self.serialize_map(Some(len))
    }

    // keyword?
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<()> {
        self.serialize_str(variant)
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

    // Tuples look just like sequences in JSON.
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


impl<'a,W> ser::SerializeSeq for &'a mut Serializer<W>
where
    W: IWriteBytes,
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> { Ok(()) }
}

impl<'a,W> ser::SerializeTuple for &'a mut Serializer<W>
where
    W: IWriteBytes,
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> { Ok(()) }
}

impl<'a,W> ser::SerializeTupleStruct for &'a mut Serializer<W>
where
    W: IWriteBytes,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> { Ok(()) }
}

impl<'a,W> ser::SerializeTupleVariant for &'a mut Serializer<W>
where
    W: IWriteBytes,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> { Ok(()) }
}


// missing optional `serialize_entry` method allows serializers to optimize for the case where kv both available
impl<'a,W> ser::SerializeMap for &'a mut Serializer<W>
where
    W: IWriteBytes,
{
    type Ok = ();
    type Error = Error;

    // The Serde data model allows map keys to be any serializable type.
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
impl<'a,W> ser::SerializeStruct for &'a mut Serializer<W>
where
    W: IWriteBytes,
{
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

impl<'a,W> ser::SerializeStructVariant for &'a mut Serializer<W>
where
    W: IWriteBytes,
{
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

//////////////////////////////////////////////////////
// from serde/src/ser/mod.rs
trait LenHint: Iterator {
    fn len_hint(&self) -> Option<usize>;
}

impl<I> LenHint for I
where
    I: Iterator,
{
    #[cfg(not(feature = "unstable"))]
    fn len_hint(&self) -> Option<usize> {
        iterator_len_hint(self)
    }

    #[cfg(feature = "unstable")]
    default fn len_hint(&self) -> Option<usize> {
        iterator_len_hint(self)
    }
}

#[cfg(feature = "unstable")]
impl<I> LenHint for I
where
    I: ExactSizeIterator,
{
    fn len_hint(&self) -> Option<usize> {
        Some(self.len())
    }
}

fn iterator_len_hint<I>(iter: &I) -> Option<usize>
where
    I: Iterator,
{
    match iter.size_hint() {
        (lo, Some(hi)) if lo == hi => Some(lo),
        _ => None,
    }
}

