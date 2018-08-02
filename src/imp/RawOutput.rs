#![allow(exceeding_bitshifts)]

extern crate serde;

use imp::error::{Error, Result};
use imp::io::{ByteWriter};
use imp::codes;
use imp::ranges;
use std::cmp;

pub type RawOutput = ByteWriter; ///// make private

impl RawOutput {

    pub fn write_raw_i16(&mut self, i: i32) -> Result<()>{
        self.write_raw_byte(((i >>  8) & 0xFF) as u8)?;
        self.write_raw_byte(        (i & 0xFF) as u8)
    }

    pub fn write_raw_i24(&mut self, i: i32) -> Result<()>{
        self.write_raw_byte(((i >> 16) & 0xFF) as u8)?;
        self.write_raw_byte(((i >>  8) & 0xFF) as u8)?;
        self.write_raw_byte(        (i & 0xFF) as u8)
    }

    pub fn write_raw_i32(&mut self, i: i32) -> Result<()> {
        self.write_raw_byte(((i >> 24) & 0xFF) as u8)?;
        self.write_raw_byte(((i >> 16) & 0xFF) as u8)?;
        self.write_raw_byte(((i >>  8) & 0xFF) as u8)?;
        self.write_raw_byte(        (i & 0xFF) as u8)
    }

    /// requires exceeding_bitshifts
    pub fn write_raw_i40(&mut self, i: i64) -> Result<()> {
        self.write_raw_byte(((i >> 32) & 0xFF) as u8)?;
        self.write_raw_byte(((i >> 24) & 0xFF) as u8)?;
        self.write_raw_byte(((i >> 16) & 0xFF) as u8)?;
        self.write_raw_byte(((i >>  8) & 0xFF) as u8)?;
        self.write_raw_byte(        (i & 0xFF) as u8)
    }

    pub fn write_raw_i48(&mut self, i: i64) -> Result<()> {
        self.write_raw_byte(((i >> 40) & 0xFF) as u8)?;
        self.write_raw_byte(((i >> 32) & 0xFF) as u8)?;
        self.write_raw_byte(((i >> 24) & 0xFF) as u8)?;
        self.write_raw_byte(((i >> 16) & 0xFF) as u8)?;
        self.write_raw_byte(((i >>  8) & 0xFF) as u8)?;
        self.write_raw_byte(        (i & 0xFF) as u8)
    }

    pub fn write_raw_i64(&mut self, i: i64) -> Result<()> {
        self.write_raw_byte((i >> 56) as u8)?;
        self.write_raw_byte((i >> 48) as u8)?;
        self.write_raw_byte((i >> 40) as u8)?;
        self.write_raw_byte((i >> 32) as u8)?;
        self.write_raw_byte((i >> 24) as u8)?;
        self.write_raw_byte((i >> 16) as u8)?;
        self.write_raw_byte((i >>  8) as u8)?;
        self.write_raw_byte((i >>  0) as u8)
    }

    pub fn write_raw_float(&mut self, f: f32) -> Result<()> {
        self.write_raw_i32(f.to_bits() as i32)
    }

    pub fn write_raw_double(&mut self, f: f64) -> Result<()> {
        self.write_raw_i64(f.to_bits() as i64)
    }
}


fn bit_switch(l: i64) -> u8 {
    if l < 0 {
        (!l).leading_zeros() as u8
    } else {
        l.leading_zeros() as u8
    }
}

pub type FressianWriter = RawOutput;

impl FressianWriter {

    fn write_code(&mut self, code: u8) -> Result<()> {
        self.write_raw_byte(code)
    }

    pub fn write_int(&mut self, i: i64) -> Result<()> {
        match bit_switch(i) {
            1..=14 => {
                self.write_code(codes::INT)?;
                self.write_raw_i64(i)
            }

            15..=22 => {
                self.write_raw_byte(codes::INT_PACKED_7_ZERO.wrapping_add( (i >> 48) as u8 ))?;
                self.write_raw_i48(i)
            }

            23..=30 => {
                self.write_raw_byte(codes::INT_PACKED_6_ZERO.wrapping_add( (i >> 40) as u8 ))?;
                self.write_raw_i40(i)
            }

            31..=38 => {
                self.write_raw_byte(codes::INT_PACKED_5_ZERO.wrapping_add( (i >> 32) as u8 ))?;
                self.write_raw_i32(i as i32)
            }

            39..=44 => {
                self.write_raw_byte(codes::INT_PACKED_4_ZERO.wrapping_add( (i >> 24) as u8))?;
                self.write_raw_i24(i as i32)
            }

            45..=51 => {
                self.write_raw_byte(codes::INT_PACKED_3_ZERO.wrapping_add( (i >> 16) as u8))?;
                self.write_raw_i16(i as i32)
            }

            52..=57 => {
                self.write_raw_byte(codes::INT_PACKED_2_ZERO.wrapping_add( (i >> 8) as u8))?;
                self.write_raw_byte(i as u8)
            }

            58..=64 => {
                if i < -1 {
                    self.write_raw_byte(codes::INT_PACKED_2_ZERO.wrapping_add( (i >> 8) as u8))?;
                    self.write_raw_byte(i as u8)
                } else {
                    self.write_raw_byte(i as u8)
                }
            }

            _ => Err(serde::de::Error::custom("more than 64 bits in a long!"))///////////////////////////////
        }
    }

    pub fn write_float(&mut self, f: f32) -> Result<()> {
        self.write_code(codes::FLOAT)?;
        self.write_raw_float(f)
    }

    pub fn write_double(&mut self, f: f64) -> Result<()> {
        if f == 0.0 {
            self.write_code(codes::DOUBLE_0)
        } else if f == 1.0 {
            self.write_code(codes::DOUBLE_1)
        } else {
            self.write_code(codes::DOUBLE)?;
            self.write_raw_double(f)
        }
    }

    pub fn write_count(&mut self, count: usize) -> Result<()> {
        self.write_int(count as i64)
    }

    pub fn write_null(&mut self) -> Result<()> {
        self.write_code(codes::NULL)
    }

    pub fn write_boolean(&mut self, b: bool) -> Result<()> {
        if b {
            self.write_code(codes::TRUE)
        } else {
            self.write_code(codes::FALSE)
        }
    }

    pub fn write_bytes(&mut self, bytes: &[u8], offset: usize, length: usize) -> Result<()> {
        if length < ranges::BYTES_PACKED_LENGTH_END {
            self.write_raw_byte(codes::BYTES_PACKED_LENGTH_START + length as u8)?;
            self.write_raw_bytes(bytes, offset,length)
        } else {
            let mut length = length;
            let mut offset = offset;
            while ranges::BYTE_CHUNK_SIZE < length {
                self.write_code(codes::BYTES_CHUNK)?;
                self.write_count(ranges::BYTE_CHUNK_SIZE)?;
                self.write_raw_bytes(bytes, offset, ranges::BYTE_CHUNK_SIZE)?;
                offset += ranges::BYTE_CHUNK_SIZE;
                length -= ranges::BYTE_CHUNK_SIZE;
            };
            self.write_code(codes::BYTES)?;
            self.write_count(length)?;
            self.write_raw_bytes(bytes, offset, length)
        }
    }
}

mod test {
    use super::*;

    #[test]
    fn ints_test (){
        let mut fw = FressianWriter::from_vec(Vec::new());
        //Short/MIN_VALUE
        let v: i16 = -32768;
        let control: Vec<u8> = vec![103, 128, 0];
        fw.write_int(v as i64).unwrap();
        assert_eq!(&fw.to_vec(), &control);

        //Short/MAX_VALUE
        let v: i16 = 32767;
        let control: Vec<u8> = vec![104, 127, 255];
        fw.reset();
        fw.write_int(v as i64).unwrap();
        assert_eq!(&fw.to_vec(), &control);

        //Integer/MIN_VALUE
        let v: i32 = -2147483648;
        let control: Vec<u8> = vec![117, 128, 0, 0, 0];
        fw.reset();
        fw.write_int(v as i64).unwrap();
        assert_eq!(&fw.to_vec(), &control);

        //Integer/MAX_VALUE
        let v: i32 = 2147483647;
        let control: Vec<u8> = vec![118, 127, 255, 255, 255];
        fw.reset();
        fw.write_int(v as i64).unwrap();
        assert_eq!(&fw.to_vec(), &control);

        // min i40
        let v: i64 = -549755813887;
        let control: Vec<u8> = vec![121, 128, 0, 0, 0, 1];
        fw.reset();
        fw.write_int(v as i64).unwrap();
        assert_eq!(&fw.to_vec(), &control);

        // max i40
        let v: i64 = 549755813888;
        let control: Vec<u8> = vec![122, 128, 0, 0, 0, 0];
        fw.reset();
        fw.write_int(v as i64).unwrap();
        assert_eq!(&fw.to_vec(), &control);

        // max i48
        let v: i64 = 140737490000000;
        let control: Vec<u8> = vec![126, 128, 0, 0, 25, 24, 128];
        fw.reset();
        fw.write_int(v as i64).unwrap();
        assert_eq!(&fw.to_vec(), &control);

        // JS_MAX_SAFE_INT
        let v: i64 = 9007199254740991;
        let control: Vec<u8> = vec![248, 0, 31, 255, 255, 255, 255, 255, 255];
        fw.reset();
        fw.write_int(v as i64).unwrap();
        assert_eq!(&fw.to_vec(), &control);

        // JS_MAX_SAFE_INT++
        let v: i64 = 9007199254740992;
        let control: Vec<u8> = vec![248, 0, 32, 0, 0, 0, 0, 0, 0];
        fw.reset();
        fw.write_int(v as i64).unwrap();
        assert_eq!(&fw.to_vec(), &control);

        // JS_MIN_SAFE_INT
        let v: i64 = -9007199254740991;
        let control: Vec<u8> = vec![248, 255, 224, 0, 0, 0, 0, 0, 1];
        fw.reset();
        fw.write_int(v as i64).unwrap();
        assert_eq!(&fw.to_vec(), &control);

        // JS_MIN_SAFE_INT--
        let v: i64 = -9007199254740992;
        let control: Vec<u8> = vec![248, 255, 224, 0, 0, 0, 0, 0, 0];
        fw.reset();
        fw.write_int(v as i64).unwrap();
        assert_eq!(&fw.to_vec(), &control);

        // long max (i64)
        let v: i64 = 9223372036854775807;
        let control: Vec<u8> = vec![248, 127, 255, 255, 255, 255, 255, 255, 255];
        fw.reset();
        fw.write_int(v as i64).unwrap();
        assert_eq!(&fw.to_vec(), &control);

        // long min (i64)
        let v: i64 = -9223372036854775808;
        let control: Vec<u8> = vec![248, 128, 0, 0, 0, 0, 0, 0, 0];
        fw.reset();
        fw.write_int(v as i64).unwrap();
        assert_eq!(&fw.to_vec(), &control);
    }

    #[test]
    fn write_floats_test(){
        let mut fw = FressianWriter::from_vec(Vec::new());

        //Float/MIN_VALUE
        let v: f32 = 1.4E-45;
        let control: Vec<u8> = vec![249, 0, 0, 0, 1];
        fw.write_float(v).unwrap();
        assert_eq!(&fw.to_vec(), &control);

        fw.reset();

        //Float/MAX_VALUE
        let v: f32 = 3.4028235E38;
        let control: Vec<u8> = vec![249, 127, 127, 255, 255];
        fw.write_float(v).unwrap();
        assert_eq!(&fw.to_vec(), &control);

        fw.reset();

        // DOUBLE/MIN_VALUE
        let v: f64 = 4.9E-324;
        let control: Vec<u8> = vec![250, 0, 0, 0, 0, 0, 0, 0, 1];
        fw.write_double(v).unwrap();
        assert_eq!(&fw.to_vec(), &control);

        fw.reset();

        // DOUBLE/MAX_VALUE
        let v: f64 = 1.7976931348623157E308;
        let control: Vec<u8> = vec![250, 127, 239, 255, 255, 255, 255, 255, 255 ];
        fw.write_double(v).unwrap();
        assert_eq!(&fw.to_vec(), &control);
    }

    #[test]
    fn write_bytes_test(){
        let mut fw = FressianWriter::from_vec(Vec::new());

        // packed count
        let v: Vec<u8> = vec![255,254,253,0,1,2,3];
        let control: Vec<u8> = vec![215,255,254,253,0,1,2,3];
        fw.write_bytes(v.as_slice(), 0, v.len()).unwrap();
        assert_eq!(&fw.to_vec(), &control);

        fw.reset();

        // unpacked length
        let v: Vec<u8> = vec![252,253,254,255,0,1,2,3,4];
        let control: Vec<u8> = vec![217, 9, 252, 253, 254, 255, 0, 1, 2, 3, 4];
        fw.write_bytes(v.as_slice(), 0, v.len()).unwrap();
        assert_eq!(&fw.to_vec(), &control);

        //missing chunked
    }

}

