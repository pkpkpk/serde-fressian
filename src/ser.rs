extern crate serde;

use serde::ser::{self, Serialize};

use std::cmp;

use imp::RawOutput::*;
use imp::codes;
use imp::ranges;
use imp::error::{Error, Result};

pub type Serializer = RawOutput;

impl Serializer{
    pub fn new(out: Vec<u8>) -> Serializer {
        RawOutput::from_vec(out)
    }

    pub fn write_footer(&mut self) -> Result<()> {
        let length = self.get_bytes_written();
        // self.clear_caches()
        self.write_raw_i32(codes::FOOTER_MAGIC as i32)?;
        self.write_raw_i32(length as i32)?; //?
        let checksum = 0; //rawOut.getChecksum().getValue()
        self.write_raw_i32(checksum)
        // self.reset();
    }

    pub fn begin_open_list(&mut self) -> Result<()> {
        self.write_code(codes::BEGIN_OPEN_LIST)
    }

    pub fn begin_closed_list(&mut self) -> Result<()> {
        self.write_code(codes::BEGIN_CLOSED_LIST)
    }

    pub fn end_list(&mut self) -> Result<()> {
        self.write_code(codes::END_COLLECTION)
    }

    pub fn write_list_header(&mut self, length: usize) -> Result<()>{
        if (length as u8) < ranges::LIST_PACKED_LENGTH_END {
            self.write_raw_byte(codes::LIST_PACKED_LENGTH_START.wrapping_add( length as u8))
        } else {
            self.write_code(codes::LIST)?;
            self.write_count(length)
        }
    }

    pub fn write_list<I>(&mut self, iter: I) -> Result<()>
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

    pub fn write_map<K,V,I>(&mut self, iter:I) -> Result<()>
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

    pub fn write_set<I, V>(&mut self, iter: I) -> Result<()>
    where V: Serialize + std::cmp::Eq + std::hash::Hash,
          I: IntoIterator<Item = V>,
    {
        self.write_code(codes::SET)?;
        self.write_list(iter)
    }
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

    fn serialize_str(self, v: &str) -> Result<()> { self.write_string(&v.to_string()) }

    fn serialize_bytes(self, bytes: &[u8]) -> Result<()> {
        self.write_bytes(&bytes.to_vec(), 0, bytes.len())
    }



    fn serialize_some<S>(self, value: &S) -> Result<()>
    where
        S: ?Sized + Serialize,
    {
        value.serialize(self)
    }

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
        match _len {
            Some(n) => {
                self.write_list_header(n)?;
                Ok(self)
            }
            None => {
                Err(serde::de::Error::custom(
                    "cannot use serde::ser::serialize on uncounted sequences at this time.
                     If known to be finite use serializer.write_list().
                     If indet, use begin_open_list & end_list manually."))
            }
        }
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        match _len {
            Some(n) => {
                let length = 2 * _len.unwrap();
                self.write_code(codes::MAP)?;
                self.write_list_header(length)?;
                Ok(self)
            }
            None => {
                Err(serde::de::Error::custom(
                    "cannot use serde::ser::serialize on uncounted sequences at this time.
                     If map, use serializer.write_list().
                     If known to be finite use serializer.write_list().
                     If indet, use begin_open_list & end_list manually."))
            }
        }
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

// called after `serialize_seq` is called on the Serializer.
impl<'a> ser::SerializeSeq for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    // Serialize a single element of the sequence.
    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> { Ok(()) }
}

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


// There is a third optional method on the `SerializeMap` trait. The
// `serialize_entry` method allows serializers to optimize for the case where
// key and value are both available simultaneously.
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

//////////////////////////////////////////////////////



// // write a value to bytes, skipping writer creation
// pub fn to_vec<T>(value: &T) -> Result<Vec<u8>>
// where
//     T: Serialize,
// {
//     let buf = Vec::with_capacity(100);
//     let mut serializer = Serializer {
//         rawOut: RawOutput::from_vec(buf)
//     };
//
//     value.serialize(&mut serializer)?;
//     Ok(serializer.rawOut.into_inner())
// }




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