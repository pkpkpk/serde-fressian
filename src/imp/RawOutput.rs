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

fn encoding_size(ch: u32) -> usize {
    if ch <= 0x007f{
        return 1;
    } else if ch > 0x07ff {
        return 3;
    } else {
        return 2;
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

pub type FressianWriter = RawOutput;

impl FressianWriter {

    pub fn write_code(&mut self, code: u8) -> Result<()> {
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

    #[cfg(not(raw_UTF8))]
    pub fn write_string(&mut self, s: &str) -> Result<()> {
        let char_length: usize = s.chars().count();

        if char_length == 0 {
            self.write_raw_byte(codes::STRING_PACKED_LENGTH_START)?;
        } else {
            // chars > 0xFFFF are actually 2 chars in java, need a separate string length
            // to write the appropriate code into the bytes
            let mut j_char_length = char_length;
            let mut string_pos: usize = 0;
            let mut j_string_pos: usize = 0;
            let mut iter = itertools::put_back(s.chars());

            // let maxBufNeeded: usize = cmp::min(65536, CHAR_LENGTH * 3);
            // ^ silently fails, should be using char count. compiler bug?
            let max_buf_needed: usize = cmp::min(65536, s.len() * 3);
            let mut buffer: Vec<u8> = Vec::with_capacity(max_buf_needed); //abstract out into stringbuffer, re-use

            while string_pos < char_length {
                let mut buf_pos = 0;
                loop {

                    let ch: Option<char> = iter.next();

                    match ch {
                        Some(ch) => {
                            let enc_size = encoding_size(ch as u32);

                            if (buf_pos + enc_size) < max_buf_needed {
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
                                    j_string_pos += 2; // equivalent to eating 2 java chars
                                    j_char_length += 1; // track extra java char we created
                                    continue;
                                } else {
                                    write_char(ch as u32, &mut buffer, &mut buf_pos);
                                    string_pos += 1;
                                    j_string_pos += 1;
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
                    self.write_raw_byte(codes::STRING_PACKED_LENGTH_START.wrapping_add( buf_pos as u8))?;
                } else if j_string_pos == j_char_length {
                    self.write_code(codes::STRING)?;
                    self.write_count(buf_pos)?;
                } else {
                    self.write_code(codes::STRING_CHUNK)?;
                    self.write_count(buf_pos)?;
                }
                self.write_raw_bytes(&buffer,0,buf_pos)?;
            }
        }

        Ok(())
    }

    #[cfg(raw_UTF8)]
    pub fn write_string(&mut self, s: &str) -> Result<()> {
        let bytes = s.as_bytes();
        let length = bytes.len();
        self.write_code(codes::UTF8)?;
        self.write_count(length)?;
        self.write_raw_bytes(&bytes.to_vec(), 0, length)
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

    #[test]
    fn write_string_test(){
        let mut fw = FressianWriter::from_vec(Vec::new());

        let v = "".to_string();
        #[cfg(not(raw_UTF8))]
        let control: Vec<u8> = vec![218];
        #[cfg(raw_UTF8)]
        let control: Vec<u8> = vec![191,0];
        fw.write_string(&v).unwrap();
        assert_eq!(&fw.to_vec(), &control);

        fw.reset();

        let v = "hola".to_string();
        #[cfg(not(raw_UTF8))]
        let control: Vec<u8> = vec![222,104,111,108,97];
        #[cfg(raw_UTF8)]
        let control: Vec<u8> = vec![191,4,104,111,108,97];
        fw.write_string(&v).unwrap();
        assert_eq!(&fw.to_vec(), &control);

        fw.reset();

        let v = "é❤️ßℝ東京東京😉 😎 🤔 😐 🙄".to_string();
        #[cfg(not(raw_UTF8))]
        let control: Vec<u8> = vec![227,60,101,204,129,226,157,164,239,184,143,195,159,226,132,157,230,157,177,228,186,172,230,157,177,228,186,172,237,160,189,237,184,137,32,237,160,189,237,184,142,32,237,160,190,237,180,148,32,237,160,189,237,184,144,32,237,160,189,237,185,132];
        #[cfg(raw_UTF8)]
        let control: Vec<u8> = vec![191,50,101,204,129,226,157,164,239,184,143,195,159,226,132,157,230,157,177,228,186,172,230,157,177,228,186,172,240,159,152,137,32,240,159,152,142,32,240,159,164,148,32,240,159,152,144,32,240,159,153,132];
        fw.write_string(&v).unwrap();
        assert_eq!(&fw.to_vec(), &control);

        // missing chunked
    }
}

