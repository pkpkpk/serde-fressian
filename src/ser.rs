use serde::ser::{self, Serialize};

use imp::RawOutput::*;
use imp::codes;
use imp::ranges;
use error::{Error, ErrorCode, Result};
use imp::io::{ByteWriter, IWriteBytes};
use value::{self, Value};
use imp::cache::{Cache};

pub struct Serializer<W> {
    writer: W,
    cache: Cache,
}

fn error<W,T>(ser: &Serializer<W>, reason: ErrorCode) -> Result<T>
    where W: IWriteBytes
{
    let position: usize = ser.writer.get_bytes_written();
    Err(Error::syntax(reason, position))
}

impl<W> Serializer<W>
where
    W: IWriteBytes,
{
    pub fn new(writer: W) -> Self {
        Serializer {
            writer: writer,
            cache: Cache::new(),
        }
    }
}

impl Serializer<ByteWriter<Vec<u8>>> {
    pub fn from_vec(v: Vec<u8>) -> Self {
        Serializer::new(ByteWriter::from_vec(v))
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

    pub fn into_inner(self) -> Vec<u8> {
        self.writer.into_inner()
    }
}

/// write to vec<u8>
pub fn to_vec<T>(value: &T) -> Result<Vec<u8>>
where
    T: Serialize,
{
    let buf = Vec::with_capacity(100);
    let mut serializer = Serializer::from_vec(buf);

    value.serialize(&mut serializer)?;
    Ok(serializer.into_inner())
}

/// write to vec<u8> with footer
pub fn to_vec_footer<T>(value: &T) -> Result<Vec<u8>>
where
    T: Serialize,
{
    let buf = Vec::with_capacity(100);
    let mut serializer = Serializer::from_vec(buf);
    value.serialize(&mut serializer)?;
    serializer.write_footer()?;
    Ok(serializer.into_inner())
}

impl<W> Serializer<W>
where
    W: IWriteBytes,
{

    // this will work for narrow fressian types, need to expand Value
    // to handle into_iter, other maps, sets, etc
    pub fn caching_serialize<V>(&mut self, value: V) -> Result<()>
        where V: Into<Value> + Serialize + Clone,
    {
        // Value::from(value.into()).serialize(CachingSerializer{ser: self})

        //this clones everything that it is given so that it can test the cache
        // --> this seems unecessary, should be able to borrow, test cache, clone only when unique,
        let V: Value = Value::from(value.clone().into()); /////////////////////// need refd Vals
        self.cache_value(value, V)
    }

    fn cache_value<V>(&mut self, inner: V, Val: Value) -> Result<()>
        where V: Serialize,
    {
        let cached_code: Option<u8> = self.cache.get(&Val);
        match cached_code {
            Some(code) => {
                if code < ranges::PRIORITY_CACHE_PACKED_END {
                    self.write_code( code + codes::PRIORITY_CACHE_PACKED_START )
                } else {
                    self.write_code(codes::GET_PRIORITY_CACHE)?;
                    self.write_int(code as i64)
                }
            }
            None => {
                let _ = self.cache.put(Val);
                self.write_code(codes::PUT_PRIORITY_CACHE)?;
                inner.serialize(self)
            }
        }
    }

    pub fn write_footer(&mut self) -> Result<()> {
        let length = self.writer.get_bytes_written();
        // self.clear_caches()
        RawOutput.write_raw_i32(&mut self.writer,codes::FOOTER_MAGIC as i32)?;
        RawOutput.write_raw_i32(&mut self.writer,length as i32)?; //?
        let checksum = 0; //rawOut.getChecksum().getValue()
        RawOutput.write_raw_i32(&mut self.writer, checksum)
        // self.reset();
    }

    pub fn write_code(&mut self, code: u8 ) -> Result<()>
    {
        RawOutput.write_code(&mut self.writer, code)
    }

    pub fn write_count(&mut self, count: usize) -> Result<()> {
        RawOutput.write_int(&mut self.writer, count as i64)
    }

    pub fn write_int(&mut self, i: i64 ) -> Result<()>{
        RawOutput.write_int(&mut self.writer, i)
    }

    pub fn write_null(&mut self) -> Result<()> {
        RawOutput.write_null(&mut self.writer)
    }

    pub fn write_boolean(&mut self, b: bool) -> Result<()>{
        RawOutput.write_boolean(&mut self.writer, b)
    }

    pub fn write_float(&mut self, f: f32) -> Result<()>{
        RawOutput.write_float(&mut self.writer,f)
    }

    pub fn write_double(&mut self, f: f64) -> Result<()>{
        RawOutput.write_double(&mut self.writer,f)
    }

    pub fn write_bytes(&mut self, bytes: &[u8], offset: usize, length: usize) -> Result<()>{
        RawOutput.write_bytes(&mut self.writer,bytes,offset,length)
    }

    pub fn write_string(&mut self, s: &str) -> Result<()> {
        RawOutput.write_string(&mut self.writer,s)
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

    fn serialize_u64(self, v: u64) -> Result<()> {
        if (std::i64::MAX as u64) < v {
            error(self, ErrorCode::IntTooLargeFori64)
        } else {
            self.write_int(v as i64)
        }
    }

    fn serialize_f32(self, v: f32) -> Result<()> { self.write_float(v) }

    fn serialize_f64(self, v: f64) -> Result<()> { self.write_double(v) }

    fn serialize_char(self, v: char) -> Result<()> { self.serialize_str(&v.to_string()) }

    #[inline]
    fn serialize_str(self, v: &str) -> Result<()> {
        // if cfg!(all(target_arch = "wasm32", target_os = "unknown")) {
        //     // bytelength + pointer
        //     self.write_code(codes::STR)?;
        //     if v.len() == 0 {
        //         self.write_count(0)
        //     } else {
        //         self.write_count(v.len())?;
        //         self.write_int(v.as_ptr() as i64)
        //     }
        // } else {
        //     self.write_string(v)
        // }
        self.write_string(v)
    }

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
            "SET" => {
                self.write_code(codes::SET)?;
                value.serialize(self)
            }
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
                error(self, ErrorCode::UnsupportedType)
            }
            _ => value.serialize(self)
        }
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        match _len {
            Some(n) => {
                self.write_list_header(n)?;
                Ok(Compound::LIST{ser: self, cache_elements: false, list_type: ListType::Fixed})
            }
            None => {
                self.begin_closed_list()?;
                Ok(Compound::LIST{ser: self, cache_elements: false, list_type: ListType::Closed})
            }
        }
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        match _len {
            Some(l) => {
                let length = 2 * l;
                self.write_code(codes::MAP)?;
                self.write_list_header(length)?;
                Ok(Compound::MAP{ser: self, list_type: ListType::Fixed})
            }
            None => {
                self.write_code(codes::MAP)?;
                self.begin_closed_list()?;
                Ok(Compound::MAP{ser: self, list_type: ListType::Closed})
            }
        }
    }

    fn serialize_struct(self, _name: &'static str, len: usize,) -> Result<Self::SerializeStruct> {
        self.serialize_map(Some(len))
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(self,_name: &'static str, len: usize,) -> Result<Self::SerializeTupleStruct> {
        match _name {
            "SYM" => {
                self.write_code(codes::SYM)?;
                Ok(Compound::LIST{ser: self, cache_elements: true, list_type: ListType::Fixed})
            }
            "KEY" => {
                self.write_code(codes::KEY)?;
                Ok(Compound::LIST{ser: self, cache_elements: true, list_type: ListType::Fixed})
            }
            _ => self.serialize_seq(Some(len))
        }
    }


    ///////////////////////////////////////////////////////////////////////
    // enums //////////////////////////////////////////////////////////////

    fn serialize_unit_variant(self,_name: &'static str,_variant_index: u32,variant: &'static str) -> Result<()> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        match name {
            "Result" => {
                match variant {
                    "Ok" => value.serialize(self),
                    "Err" => {
                        self.write_code(codes::ERROR)?;
                        value.serialize(self)
                    }
                    &_ => Err(Error::msg("serialize result failed to match enum".to_string()))
                }
            }
            _ => {
                self.write_list_header(2)?;
                variant.serialize(&mut *self)?;
                value.serialize(&mut *self)
            }
        }
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        variant.serialize(&mut *self)?;
        Ok(Compound::LIST{ser: self, cache_elements: false, list_type: ListType::Fixed})
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        variant.serialize(&mut *self)?;
        Ok(Compound::LIST{ser: self, cache_elements: false, list_type: ListType::Fixed})
    }

}

pub enum ListType {
    Fixed,
    Closed,
    Open
}

pub enum Compound<'a, W: 'a> {
    LIST {
        ser: &'a mut Serializer<W>,
        cache_elements: bool,
        list_type: ListType
    },
    MAP {
        ser: &'a mut Serializer<W>,
        // cache_keys: bool,
        list_type: ListType
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
        match *self {
            Compound::LIST { ref mut ser, cache_elements, ..} => {
                if cache_elements {
                    value.serialize(CachingSerializer{ser: ser})
                } else {
                    value.serialize(&mut **ser)
                }
            }

            Compound::MAP {ref mut ser, ..} => {
                value.serialize(&mut **ser)
            }
        }
    }

    fn end(self) -> Result<()> {
        match self {
            Compound::LIST{ser, list_type, ..} => {
                match list_type {
                    ListType::Fixed => Ok(()),
                    _ => ser.end_list()
                }
            }
            Compound::MAP{ser, list_type} => {
                match list_type {
                    ListType::Fixed => Ok(()),
                    _ => ser.end_list()
                }
            }
        }
    }
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

impl<'a,W> ser::SerializeMap for Compound<'a,W>
where
    W: IWriteBytes,
{
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where T: ?Sized + Serialize,
    {
        ser::SerializeSeq::serialize_element(self, key)
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where T: ?Sized + Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<()> { ser::SerializeSeq::end(self) }
}

impl<'a,W> ser::SerializeStruct for Compound<'a,W>
where
    W: IWriteBytes,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where T: ?Sized + Serialize,
    {
        ser::SerializeSeq::serialize_element(self, key)?;
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<()> { ser::SerializeSeq::end(self) }
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
        ser::SerializeSeq::serialize_element(self, key)?;
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<()> { ser::SerializeSeq::end(self) }
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
            // typed arrays must length. might be able to relax this see above
            None => error(self.ser, ErrorCode::UnsupportedTAType)
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
    fn serialize_i8(self, _value: i8) -> Result<()> { error(self.ser, ErrorCode::UnsupportedTAType) }

    #[inline]
    fn serialize_i16(self, _value: i16) -> Result<()> { error(self.ser, ErrorCode::UnsupportedTAType) }

    #[inline]
    fn serialize_u8(self, _value: u8) -> Result<()> { error(self.ser, ErrorCode::UnsupportedTAType) }

    #[inline]
    fn serialize_u16(self, _value: u16) -> Result<()> { error(self.ser, ErrorCode::UnsupportedTAType) }

    #[inline]
    fn serialize_u32(self, _value: u32) -> Result<()> { error(self.ser, ErrorCode::UnsupportedTAType) }

    #[inline]
    fn serialize_u64(self, _value: u64) -> Result<()> { error(self.ser, ErrorCode::UnsupportedTAType) }

    #[inline]
    fn serialize_char(self, _value: char) -> Result<()> { error(self.ser, ErrorCode::UnsupportedTAType) }

    #[inline]
    fn serialize_bytes(self, _data: &[u8]) -> Result<()> { error(self.ser, ErrorCode::UnsupportedTAType) }

    #[inline]
    fn serialize_unit(self) -> Result<()> { error(self.ser, ErrorCode::UnsupportedTAType) }

    #[inline]
    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> { error(self.ser, ErrorCode::UnsupportedTAType) }

    #[inline]
    fn serialize_unit_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str) -> Result<()> {
        error(self.ser, ErrorCode::UnsupportedTAType)
    }

    #[inline]
    fn serialize_newtype_struct<T>(self, _name: &'static str, _value: &T) -> Result<()>
    where T: ?Sized + Serialize, {
        error(self.ser, ErrorCode::UnsupportedTAType)
    }

    #[inline]
    fn serialize_newtype_variant<T>(self,_name: &'static str,_variant_index: u32, _variant: &'static str, _value: &T,) -> Result<()>
    where T: ?Sized + Serialize, {
        error(self.ser, ErrorCode::UnsupportedTAType)
    }

    #[inline]
    fn serialize_none(self) -> Result<()> {
        error(self.ser, ErrorCode::UnsupportedTAType)
    }

    #[inline]
    fn serialize_some<S>(self, _value: &S) -> Result<()>
    where S: ?Sized + Serialize, {
        error(self.ser, ErrorCode::UnsupportedTAType)
    }

    #[inline]
    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        error(self.ser, ErrorCode::UnsupportedTAType)
    }

    #[inline]
    fn serialize_tuple_struct(self,_name: &'static str, _len: usize,) -> Result<Self::SerializeTupleStruct> {
        error(self.ser, ErrorCode::UnsupportedTAType)
    }

    #[inline]
    fn serialize_tuple_variant(self,_name: &'static str,_variant_index: u32, _variant: &'static str,_len: usize, ) -> Result<Self::SerializeTupleVariant> {
         error(self.ser, ErrorCode::UnsupportedTAType)
    }

    #[inline]
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        error(self.ser, ErrorCode::UnsupportedTAType)
    }

    #[inline]
    fn serialize_struct( self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        error(self.ser, ErrorCode::UnsupportedTAType)
    }

    #[inline]
    fn serialize_struct_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _len: usize) -> Result<Self::SerializeStructVariant> {
        error(self.ser, ErrorCode::UnsupportedTAType)
    }

    #[inline]
    fn serialize_str(self, _value: &str) -> Result<()> {
        error(self.ser, ErrorCode::UnsupportedTAType)
    }
}
/////////////////////////////////////////////////////////////////////////////


struct CachingSerializer<'a, W: 'a>{
    ser: &'a mut Serializer<W>
}

impl<'a, W> ser::Serializer for CachingSerializer<'a, W>
where
    W: IWriteBytes,
{
    type Ok = ();
    type Error = Error;

    // type SerializeSeq = Compound<'a, W>;
    type SerializeSeq = ser::Impossible<(), Error>;
    type SerializeTuple = ser::Impossible<(), Error>;
    type SerializeTupleStruct = ser::Impossible<(), Error>;
    type SerializeTupleVariant = ser::Impossible<(), Error>;
    type SerializeMap = ser::Impossible<(), Error>;
    type SerializeStruct = ser::Impossible<(), Error>;
    type SerializeStructVariant = ser::Impossible<(), Error>;

    #[inline]
    fn serialize_bool(self, value: bool) -> Result<()> { value.serialize(self.ser) }

    #[inline]
    fn serialize_none(self) -> Result<()> { self.ser.serialize_none() }

    #[inline]
    fn serialize_unit(self) -> Result<()> { self.ser.serialize_none() }

    #[inline]
    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {self.ser.serialize_none()}

    #[inline]
    fn serialize_f32(self, _value: f32) -> Result<()> {
        if (_value == 0.0) | (_value == 1.0) {
            _value.serialize(self.ser)
        } else {
            self.ser.caching_serialize(_value)
        }
    }

    #[inline]
    fn serialize_f64(self, _value: f64) -> Result<()> {
        if (_value == 0.0) | (_value == 1.0) {
            _value.serialize(self.ser)
        } else {
            self.ser.caching_serialize(_value)
        }
    }

    #[inline]
    fn serialize_i8(self, _value: i8) -> Result<()> { self.ser.caching_serialize(_value) }

    #[inline]
    fn serialize_i16(self, _value: i16) -> Result<()> { self.ser.caching_serialize(_value) }

    #[inline]
    fn serialize_i32(self, _value: i32) -> Result<()> { self.ser.caching_serialize(_value) }

    #[inline]
    fn serialize_i64(self, _value: i64) -> Result<()> { self.ser.caching_serialize(_value) }

    #[inline]
    fn serialize_u8(self, _value: u8) -> Result<()> { self.ser.caching_serialize(_value) }

    #[inline]
    fn serialize_u16(self, _value: u16) -> Result<()> { self.ser.caching_serialize(_value) }

    #[inline]
    fn serialize_u32(self, _value: u32) -> Result<()> { self.ser.caching_serialize(_value) }

    #[inline]
    fn serialize_u64(self, _value: u64) -> Result<()> {
         if (std::i64::MAX as u64) < _value {
             error(self.ser, ErrorCode::IntTooLargeFori64)
         } else {
             self.ser.caching_serialize(_value as i64)
         }
    }

    #[inline]
    fn serialize_str(self, _value: &str) -> Result<()> {
        if _value.len() != 0 {
            self.ser.caching_serialize(_value.to_string()) /////////////////////////////////////////
        } else {
            _value.serialize(self.ser)
        }
    }

    #[inline]
    fn serialize_bytes(self, bytes: &[u8]) -> Result<()> { self.ser.caching_serialize(bytes) }

    #[inline]
    fn serialize_some<S>(self, value: &S) -> Result<()>
        where S: ?Sized + Serialize,
    {
         value.serialize(self)
    }

    ///////////////////////////////////////////////////////////////////////////////////////////


    #[inline]
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        // self.ser.write_code(codes::PUT_PRIORITY_CACHE)?;
        // self.ser.serialize_seq(_len)
        error(self.ser, ErrorCode::UnsupportedCacheType)
    }

    #[inline]
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        error(self.ser, ErrorCode::UnsupportedCacheType)
    }

    #[inline]
    fn serialize_newtype_struct<T>(self, _name: &'static str, _value: &T) -> Result<()>
    where T: ?Sized + Serialize, {
         // self.ser.write_code(codes::PUT_PRIORITY_CACHE)?;
         // self.ser.serialize_newtype_struct(_name, value)
         error(self.ser, ErrorCode::UnsupportedCacheType)
    }

    ///////////////////////////////////////////////////////////////////////////////////////////
    ///////////////////////////////////////////////////////////////////////////////////////////


    #[inline]
    fn serialize_newtype_variant<T>(self,_name: &'static str, _variant_index: u32, _variant: &'static str, _value: &T,) -> Result<()>
    where T: ?Sized + Serialize, {
         error(self.ser, ErrorCode::UnsupportedCacheType)
    }

    #[inline]
    fn serialize_char(self, _value: char) -> Result<()> {
        error(self.ser, ErrorCode::UnsupportedCacheType)
    }

    #[inline]
    fn serialize_unit_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str) -> Result<()> {
         error(self.ser, ErrorCode::UnsupportedCacheType)
    }

    #[inline]
    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        error(self.ser, ErrorCode::UnsupportedCacheType)
    }

    #[inline]
    fn serialize_tuple_struct(self,_name: &'static str, _len: usize,) -> Result<Self::SerializeTupleStruct> {
        error(self.ser, ErrorCode::UnsupportedCacheType)
    }

    #[inline]
    fn serialize_tuple_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str,_len: usize, ) -> Result<Self::SerializeTupleVariant> {
         error(self.ser, ErrorCode::UnsupportedCacheType)
    }

    #[inline]
    fn serialize_struct( self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        error(self.ser, ErrorCode::UnsupportedCacheType)
    }

    #[inline]
    fn serialize_struct_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _len: usize) -> Result<Self::SerializeStructVariant> {
        error(self.ser, ErrorCode::UnsupportedCacheType)
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