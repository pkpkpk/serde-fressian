use serde::ser::{self, Serialize};

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

//write to vec<u8> with footer
pub fn to_vec_footer<T>(value: &T) -> Result<Vec<u8>>
where
    T: Serialize,
{
    let buf = Vec::with_capacity(100);
    let mut serializer = Serializer::from_vec(buf);
    value.serialize(&mut serializer)?;
    serializer.write_footer()?;
    Ok(serializer.to_vec())
}

impl<W> Serializer<W>
where
    W: IWriteBytes,
{
    pub fn write_footer(&mut self) -> Result<()> {
        let length = self.writer.get_bytes_written();
        // self.clear_caches()
        self.rawOut.write_raw_i32(&mut self.writer,codes::FOOTER_MAGIC as i32)?;
        self.rawOut.write_raw_i32(&mut self.writer,length as i32)?; //?
        let checksum = 0; //rawOut.getChecksum().getValue()
        self.rawOut.write_raw_i32(&mut self.writer, checksum)
        // self.reset();
    }

    pub fn write_code(&mut self, code: u8 ) -> Result<()>
    {
        self.rawOut.write_code(&mut self.writer, code)
    }

    pub fn write_count(&mut self, count: usize) -> Result<()> {
        self.rawOut.write_int(&mut self.writer, count as i64)
    }

    pub fn write_int(&mut self, i: i64 ) -> Result<()>{
        self.rawOut.write_int(&mut self.writer,i)
    }

    pub fn write_null(&mut self) -> Result<()> {
        self.rawOut.write_null(&mut self.writer)
    }

    pub fn write_boolean(&mut self, b: bool) -> Result<()>{
        self.rawOut.write_boolean(&mut self.writer, b)
    }

    pub fn write_float(&mut self, f: f32) -> Result<()>{
        self.rawOut.write_float(&mut self.writer,f)
    }

    pub fn write_double(&mut self, f: f64) -> Result<()>{
        self.rawOut.write_double(&mut self.writer,f)
    }

    pub fn write_bytes(&mut self, bytes: &[u8], offset: usize, length: usize) -> Result<()>{
        self.rawOut.write_bytes(&mut self.writer,bytes,offset,length)
    }

    pub fn write_string(&mut self, s: &str) -> Result<()> {
        self.rawOut.write_string(&mut self.writer,s)
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
            self.write_code(codes::LIST_PACKED_LENGTH_START.wrapping_add( length as u8))
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

impl<'a, W> ser::Serializer for &'a mut Serializer<W>
where
    W: IWriteBytes,
{
    type Ok = ();
    type Error = Error;
    type SerializeSeq = Compound<'a, W>;
    type SerializeTuple = Compound<'a, W>;
    type SerializeTupleStruct = Compound<'a, W>;
    type SerializeTupleVariant = Compound<'a, W>;
    type SerializeMap = Compound<'a, W>;
    type SerializeStruct = Compound<'a, W>;
    type SerializeStructVariant = Compound<'a, W>;


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
            "REGEX" => {
                self.write_code(codes::REGEX)?;
                value.serialize(self)
            }
            "INT_ARRAY" => {
                self.write_code(codes::INT_ARRAY)?;
                value.serialize(TASerializer{ser: self})
            }
            "LONG_ARRAY" => {
                self.write_code(codes::LONG_ARRAY)?;
                value.serialize(TASerializer{ser: self})
            }
            "FLOAT_ARRAY" => {
                self.write_code(codes::FLOAT_ARRAY)?;
                value.serialize(TASerializer{ser: self})
            }
            "DOUBLE_ARRAY" => {
                self.write_code(codes::DOUBLE_ARRAY)?;
                value.serialize(TASerializer{ser: self})
            }
            "BOOLEAN_ARRAY" => {
                self.write_code(codes::BOOLEAN_ARRAY)?;
                value.serialize(TASerializer{ser: self})
            }
            "OBJECT_ARRAY" => {
                Err(Error::UnsupportedType)
            }
            _ => value.serialize(self)
        }
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        match _len {
            Some(n) => {
                self.write_list_header(n)?;
                Ok(Compound::LIST{ser: self, cache_elements: false})
            }
            None => {
                // might be able to swing this with .end_list() method on an open-list compound variant
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
                Ok(Compound::MAP{ser: self})
            }
            None => {
                // might be able to swing this with .end_list() method on an open-list compound variant////////
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
    fn serialize_unit_variant(self,_name: &'static str,_variant_index: u32,variant: &'static str) -> Result<()> {
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
    fn serialize_tuple_struct(self,_name: &'static str, len: usize,) -> Result<Self::SerializeTupleStruct> {
        match _name {
            "SYM" => {
                self.write_code(codes::SYM)?;
                Ok(Compound::LIST{ser: self, cache_elements: true})
            }
            "KEY" => {
                self.write_code(codes::KEY)?;
                Ok(Compound::LIST{ser: self, cache_elements: true})
            }
            _ => self.serialize_seq(Some(len))
        }
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
        Ok(Compound::LIST{ser: self, cache_elements: false})
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
        Ok(Compound::LIST{ser: self, cache_elements: false})
    }
}



pub enum Compound<'a, W: 'a> {
    LIST {
        ser: &'a mut Serializer<W>,
        cache_elements: bool
    },
    MAP {
        ser: &'a mut Serializer<W>
    }
}

impl<'a,W> ser::SerializeSeq for Compound<'a,W>
where
    W: IWriteBytes,
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where T: ?Sized + Serialize,
    {
        // value.serialize(&mut *self.ser)
        match *self {
            Compound::LIST {
                ref mut ser,
                cache_elements,
            } => {
                if cache_elements {
                    value.serialize(CachingSerializer{ser: ser})
                } else {
                    value.serialize(&mut **ser)
                }
            }

            Compound::MAP {ref mut ser} => {
                value.serialize(&mut **ser)
            }
        }

    }

    fn end(self) -> Result<()> { Ok(()) }
}

impl<'a,W> ser::SerializeTuple for Compound<'a,W>
where
    W: IWriteBytes,
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where T: ?Sized + Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<()> { Ok(()) }
}

impl<'a,W> ser::SerializeTupleStruct for Compound<'a,W>
where
    W: IWriteBytes,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where T: ?Sized + Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<()> { Ok(()) }
}

impl<'a,W> ser::SerializeTupleVariant for Compound<'a,W>
where
    W: IWriteBytes,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where T: ?Sized + Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<()> { Ok(()) }
}


// missing optional `serialize_entry` method allows serializers to optimize for the case where kv both available
impl<'a,W> ser::SerializeMap for Compound<'a,W>
where
    W: IWriteBytes,
{
    type Ok = ();
    type Error = Error;

    // The Serde data model allows map keys to be any serializable type.
    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where T: ?Sized + Serialize,
    {
        // key.serialize(&mut *self.ser)
        ser::SerializeSeq::serialize_element(self, key)
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where T: ?Sized + Serialize,
    {
        // value.serialize(&mut *self.ser)
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<()> { Ok(()) }
}

// Structs are like maps in which the keys are constrained to be compile-time constant strings
impl<'a,W> ser::SerializeStruct for Compound<'a,W>
where
    W: IWriteBytes,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where T: ?Sized + Serialize,
    {
        // key.serialize(&mut *self.ser)?;
        // value.serialize(&mut *self.ser)
        ser::SerializeSeq::serialize_element(self, key)?;
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<()> { Ok(()) }
}

impl<'a,W> ser::SerializeStructVariant for Compound<'a,W>
where
    W: IWriteBytes,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where T: ?Sized + Serialize,
    {
        // key.serialize(&mut *self.ser)?;
        // value.serialize(&mut *self.ser)
        ser::SerializeSeq::serialize_element(self, key)?;
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<()> { Ok(()) }
}



/////////////////////////////////////////////////////////////////////////////

// naive no packing seq writer for typed arrays
struct TASerializer<'a, W: 'a>{
    ser: &'a mut Serializer<W>
}

pub struct TACompound<'a, W: 'a> {
    ser: &'a mut Serializer<W>,
}

impl<'a,W> ser::SerializeSeq for TACompound<'a, W>
where
    W: IWriteBytes,
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where T: ?Sized + Serialize,
    {
        value.serialize(&mut *self.ser)
    }

    fn end(self) -> Result<()> { Ok(()) }
}


impl<'a, W> ser::Serializer for TASerializer<'a, W>
where
    W: IWriteBytes,
{
    type Ok = ();
    type Error = Error;

    type SerializeSeq = TACompound<'a, W>;


    #[inline]
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        match _len {
            Some(n) => {
                RawOutput.write_int(&mut self.ser.writer, n as i64)?;
                Ok(TACompound{ser: self.ser})
            }
            None => {
                Err(Error::Syntax)
            }
        }
    }

    #[inline]
    fn serialize_bool(self, _value: bool) -> Result<()> {
        self.ser.write_boolean(_value)
    }

    #[inline]
    fn serialize_i32(self, _value: i32) -> Result<()> {
        self.ser.write_int(_value as i64)
    }

    #[inline]
    fn serialize_i64(self, _value: i64) -> Result<()> {
        self.ser.write_int(_value as i64)
    }

    #[inline]
    fn serialize_f32(self, _value: f32) -> Result<()> {
        self.ser.write_float(_value)
    }

    #[inline]
    fn serialize_f64(self, _value: f64) -> Result<()> {
        self.ser.write_double(_value as f64)
    }


    type SerializeTuple = ser::Impossible<(), Error>;
    type SerializeTupleStruct = ser::Impossible<(), Error>;
    type SerializeTupleVariant = ser::Impossible<(), Error>;
    type SerializeMap = ser::Impossible<(), Error>;
    type SerializeStruct = ser::Impossible<(), Error>;
    type SerializeStructVariant = ser::Impossible<(), Error>;

    #[inline]
    fn serialize_i8(self, _value: i8) -> Result<()> { Err(Error::UnsupportedType) }

    #[inline]
    fn serialize_i16(self, _value: i16) -> Result<()> { Err(Error::UnsupportedType) }

    #[inline]
    fn serialize_u8(self, _value: u8) -> Result<()> { Err(Error::UnsupportedType) }

    #[inline]
    fn serialize_u16(self, _value: u16) -> Result<()> { Err(Error::UnsupportedType) }

    #[inline]
    fn serialize_u32(self, _value: u32) -> Result<()> { Err(Error::UnsupportedType) }

    #[inline]
    fn serialize_u64(self, _value: u64) -> Result<()> { Err(Error::UnsupportedType) }

    #[inline]
    fn serialize_char(self, _value: char) -> Result<()> { Err(Error::UnsupportedType) }

    #[inline]
    fn serialize_bytes(self, data: &[u8]) -> Result<()> { Err(Error::UnsupportedType) }

    #[inline]
    fn serialize_unit(self) -> Result<()> { Err(Error::UnsupportedType) }

    #[inline]
    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> { Err(Error::UnsupportedType) }

    #[inline]
    fn serialize_unit_variant(self, _name: &'static str, _variant_index: u32, variant: &'static str) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<()>
    where T: ?Sized + Serialize, {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_newtype_variant<T>(self,_name: &'static str,_variant_index: u32,variant: &'static str,value: &T,) -> Result<()>
    where T: ?Sized + Serialize, {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_none(self) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_some<S>(self, value: &S) -> Result<()>
    where S: ?Sized + Serialize, {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_tuple_struct(self,_name: &'static str, len: usize,) -> Result<Self::SerializeTupleStruct> { Err(Error::UnsupportedType)}

    #[inline]
    fn serialize_tuple_variant(self,_name: &'static str,_variant_index: u32,variant: &'static str,_len: usize, ) -> Result<Self::SerializeTupleVariant> {
         Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_struct( self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_struct_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _len: usize) -> Result<Self::SerializeStructVariant> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_str(self, _value: &str) -> Result<()> {
        Err(Error::UnsupportedType)
    }
}
/////////////////////////////////////////////////////////////////////////////

// writes PUT_PRIORITY_CACHE and then defers serialization back to original deserializer
struct CachingSerializer<'a, W: 'a>{
    ser: &'a mut Serializer<W>
}


impl<'a, W> ser::Serializer for CachingSerializer<'a, W>
where
    W: IWriteBytes,
{
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Compound<'a, W>;
    type SerializeTuple = ser::Impossible<(), Error>;
    type SerializeTupleStruct = ser::Impossible<(), Error>;
    type SerializeTupleVariant = ser::Impossible<(), Error>;
    type SerializeMap = ser::Impossible<(), Error>;
    type SerializeStruct = ser::Impossible<(), Error>;
    type SerializeStructVariant = ser::Impossible<(), Error>;

    #[inline]
    fn serialize_bool(self, _value: bool) -> Result<()> {
        _value.serialize(self.ser)
    }

    #[inline]
    fn serialize_none(self) -> Result<()> {
         // serialized as null
         self.ser.serialize_none()
    }

    #[inline]
    fn serialize_some<S>(self, value: &S) -> Result<()>
    where S: ?Sized + Serialize, {
         self.ser.write_code(codes::PUT_PRIORITY_CACHE)?;
         value.serialize(self.ser)
    }

    #[inline]
    fn serialize_unit(self) -> Result<()> {
         // serialized as null
         self.ser.serialize_unit()
    }

    #[inline]
    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        // serialized as null
        self.ser.serialize_unit_struct(_name)
    }

    #[inline]
    fn serialize_f32(self, _value: f32) -> Result<()> {
        if !( (_value == 0.0) | (_value == 1.0)) {
            self.ser.write_code(codes::PUT_PRIORITY_CACHE)?;
        }
        _value.serialize(self.ser)
    }

    #[inline]
    fn serialize_f64(self, _value: f64) -> Result<()> {
        if !( (_value == 0.0) | (_value == 1.0)) {
            self.ser.write_code(codes::PUT_PRIORITY_CACHE)?;
        }
        _value.serialize(self.ser)
    }

    #[inline]
    fn serialize_str(self, _value: &str) -> Result<()> {
        if _value.len() != 0 {
            self.ser.write_code(codes::PUT_PRIORITY_CACHE)?;
        }
        _value.serialize(self.ser)
    }

    /////////////////////////////////////////////////////////////
    /////////////////////////////////////////////////////////////

    #[inline]
    fn serialize_i8(self, _value: i8) -> Result<()> {
        self.ser.write_code(codes::PUT_PRIORITY_CACHE)?;
        _value.serialize(self.ser)
    }

    #[inline]
    fn serialize_i16(self, _value: i16) -> Result<()> {
        self.ser.write_code(codes::PUT_PRIORITY_CACHE)?;
        _value.serialize(self.ser)
    }

    #[inline]
    fn serialize_i32(self, _value: i32) -> Result<()> {
        self.ser.write_code(codes::PUT_PRIORITY_CACHE)?;
        _value.serialize(self.ser)
    }

    #[inline]
    fn serialize_i64(self, _value: i64) -> Result<()> {
        self.ser.write_code(codes::PUT_PRIORITY_CACHE)?;
        _value.serialize(self.ser)
    }

    #[inline]
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        self.ser.write_code(codes::PUT_PRIORITY_CACHE)?;
        self.ser.serialize_seq(_len)
    }

    #[inline]
    fn serialize_u8(self, _value: u8) -> Result<()> {
        self.ser.write_code(codes::PUT_PRIORITY_CACHE)?;
        _value.serialize(self.ser)
    }

    #[inline]
    fn serialize_u16(self, _value: u16) -> Result<()> {
         self.ser.write_code(codes::PUT_PRIORITY_CACHE)?;
         _value.serialize(self.ser)
    }

    #[inline]
    fn serialize_u32(self, _value: u32) -> Result<()> {
         self.ser.write_code(codes::PUT_PRIORITY_CACHE)?;
         _value.serialize(self.ser)
    }

    #[inline]
    fn serialize_u64(self, _value: u64) -> Result<()> {
         // self.ser.write_code(codes::PUT_PRIORITY_CACHE)?;
         // _value.serialize(self.ser)
         Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_char(self, _value: char) -> Result<()> {
        // self.ser.write_code(codes::PUT_PRIORITY_CACHE)?;
        // _value.serialize(self.ser)
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_bytes(self, data: &[u8]) -> Result<()> {
         self.ser.write_code(codes::PUT_PRIORITY_CACHE)?;
         self.ser.serialize_bytes(data)
    }

    #[inline]
    fn serialize_unit_variant(self, _name: &'static str, _variant_index: u32, variant: &'static str) -> Result<()> {
         self.ser.write_code(codes::PUT_PRIORITY_CACHE)?;
         // _value.serialize(self.ser)
         self.ser.serialize_unit_variant(_name, _variant_index, variant)
    }

    #[inline]
    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<()>
    where T: ?Sized + Serialize, {
         self.ser.write_code(codes::PUT_PRIORITY_CACHE)?;
         self.ser.serialize_newtype_struct(_name, value)
    }

    #[inline]
    fn serialize_newtype_variant<T>(self,_name: &'static str,_variant_index: u32,variant: &'static str,value: &T,) -> Result<()>
    where T: ?Sized + Serialize, {
         self.ser.write_code(codes::PUT_PRIORITY_CACHE)?;
         self.ser.serialize_newtype_variant(_name, _variant_index, variant, value)
    }

    #[inline]
    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_tuple_struct(self,_name: &'static str, len: usize,) -> Result<Self::SerializeTupleStruct> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_tuple_variant(self,_name: &'static str,_variant_index: u32,variant: &'static str,_len: usize, ) -> Result<Self::SerializeTupleVariant> {
         Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_struct( self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        Err(Error::UnsupportedType)
    }

    #[inline]
    fn serialize_struct_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _len: usize) -> Result<Self::SerializeStructVariant> {
        Err(Error::UnsupportedType)
    }

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