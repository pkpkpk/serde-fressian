#![allow(overflowing_literals)]

use imp::codes;
use imp::error::{Error, Result};
use imp::io::{ByteReader};
use byteorder::*;
use std::convert::TryFrom;

pub struct RawInput<'a>  {
    rdr: ByteReader<'a>
}

impl<'a> RawInput<'a> {
    pub fn from_vec(v: &'a Vec<u8>) -> RawInput {
        RawInput {
            rdr: ByteReader::from_vec(v)
        }
    }

    pub fn reset(&mut self) {
        self.rdr.reset()
    }

    pub fn read_u8(&mut self) -> Result<&u8> {
        self.rdr.read_u8()
    }

    pub fn read_i8(&mut self) -> Result<i8> {
        self.rdr.read_i8()
    }

    fn read_raw_bytes(&mut self, length: usize) -> Result<&[u8]>{
        self.rdr.read_bytes(length)
    }

    fn read_raw_i16(&mut self) -> Result<i64> {
        let high = *self.read_u8()? as i64;
        let low  = *self.read_u8()? as i64;
        Ok( (high << 8) + low)
    }

    fn read_raw_i24(&mut self) -> Result<i64> {
        let a = *self.read_u8()? as i64;
        let b = *self.read_u8()? as i64;
        let c = *self.read_u8()? as i64;
        Ok((a << 16) + (b << 8) + c)
    }

    fn read_raw_i32(&mut self) -> Result<i64> {
        let a = *self.read_u8()? as i64 & std::u8::MAX as i64;
        let b = *self.read_u8()? as i64 & std::u8::MAX as i64;
        let c = *self.read_u8()? as i64 & std::u8::MAX as i64;
        let d = *self.read_u8()? as i64 & std::u8::MAX as i64;
        Ok( ((a << 24) | (b << 16) | (c << 8) | d) & std::u32::MAX as i64)
    }

    fn read_raw_i40(&mut self) -> Result<i64> {
        let high = *self.read_u8()? as i64;
        let low = self.read_raw_i32()?;
        Ok( (high << 32) + low )
    }

    fn read_raw_i48(&mut self) -> Result<i64> {
        let high = *self.read_u8()? as i64;
        let low = self.read_raw_i40()?;
        Ok( (high << 40) + low )
    }

    fn read_raw_i64(&mut self) -> Result<i64> {
        let a = *self.read_u8()? as i64 & std::u8::MAX as i64;
        let b = *self.read_u8()? as i64 & std::u8::MAX as i64;
        let c = *self.read_u8()? as i64 & std::u8::MAX as i64;
        let d = *self.read_u8()? as i64 & std::u8::MAX as i64;
        let e = *self.read_u8()? as i64 & std::u8::MAX as i64;
        let f = *self.read_u8()? as i64 & std::u8::MAX as i64;
        let g = *self.read_u8()? as i64 & std::u8::MAX as i64;
        let h = *self.read_u8()? as i64 & std::u8::MAX as i64;
        Ok( (a << 56) | (b << 48) | (c << 40) | (d << 32) | (e << 24) | (f << 16) | (g << 8) | h )
    }

    pub fn read_int(&mut self) -> Result<i64> {
        let code = *self.read_u8()?;
        self.read_int_code(code as i8)
    }

    pub fn read_int_code(&mut self, code: i8) -> Result<i64> {
        match code  {
            // 0xFF => {
            //     Ok(-1)
            // },
            // 0 to 63
            0x00..=0x3F => {
                Ok((code & 0xFF) as i64)
            },
            // 64 to 95
            0x40..=0x5F => {
                let packing = ((code - codes::INT_PACKED_2_ZERO as i8) as i64) << 8;
                let r = *self.read_u8()? as i64;
                Ok(packing | r)
            }
            // 96 to 111
            0x60..=0x6f => {
                let packing = ((code - codes::INT_PACKED_3_ZERO as i8) as i64) << 16;
                let r = self.read_raw_i16()?;
                Ok(packing | r)
            }
            // 112 115
            0x70..=0x73 => {
                let packing = ((code - codes::INT_PACKED_4_ZERO as i8) as i64) << 24;
                let r = self.read_raw_i24()?;
                Ok(packing | r)
            }
            //116 119
            0x74..=0x77 => {
                let packing = ((code - codes::INT_PACKED_5_ZERO as i8) as i64) << 32;
                let r = self.read_raw_i32()?;
                Ok(packing | r)
            }
            //120 to 123
            0x78..=0x7B => {
                let packing = ((code - codes::INT_PACKED_6_ZERO as i8) as i64) << 40;
                let r = self.read_raw_i40()?;
                Ok(packing | r)
            }
            // 124 to 127
            0x7C..=0x7F =>{
                let packing = ((code - codes::INT_PACKED_7_ZERO as i8) as i64) << 48;
                let r = self.read_raw_i48()?;
                Ok(packing | r)
            }
            code if code == (codes::INT as i8) => {
                self.read_raw_i64()
            }
            _ => {
                Err(Error::Syntax) // "expected i64..."///////////////////////////////////////////
            }
        }
    }

    pub fn read_next_code(&mut self) -> Result<i8> {
        self.read_i8()
    }

    fn read_i32(&mut self) -> Result<i32>{
        Ok(self.read_int()? as i32)
    }

    pub fn read_count(&mut self) -> Result<i32>{ //////// coercion to i32 seems pointlessly complicated
        self.read_i32()
    }

    fn read_raw_float(&mut self) -> Result<f32> {
        let bytes = self.read_raw_bytes(4)?;
        let f = byteorder::BigEndian::read_f32(bytes);
        Ok(f)
    }

    fn read_raw_double(&mut self) -> Result<f64> {
        let bytes = self.read_raw_bytes(8)?;
        let d = byteorder::BigEndian::read_f64(bytes);
        Ok(d)
    }

    pub fn read_double_code(&mut self, code: i8) -> Result<f64>{
        match code as u8 {
            codes::DOUBLE => {
                self.read_raw_double()
            }
            codes::DOUBLE_0 => {
                Ok(0.0)
            }
            codes::DOUBLE_1 => {
                Ok(1.0)
            }
            _ => Err(Error::Syntax) // expected double, code///////////////////////////////////////
        }
    }

    pub fn read_double(&mut self) -> Result<f64> {
        let code = *self.read_u8()?;
        self.read_double_code(code as i8)
    }

    pub fn read_float_code(&mut self, code: i8) -> Result<f32> {
        match code as u8 {
            codes::FLOAT => {
                self.read_raw_float()
            }
            _ => Err(Error::Syntax) // expected float, code////////////////////////////////////////
        }
    }

    pub fn read_float(&mut self) -> Result<f32> {
        let code = *self.read_u8()?;
        self.read_float_code(code as i8)
    }

    pub fn read_boolean_code(&mut self, code: i8) -> Result<bool> {
        match code as u8 {
            codes::TRUE => {
                Ok(true)
            },
            codes::FALSE => {
                Ok(false)
            }
            _ => Err(Error::Syntax) // expected boolean, code//////////////////////////////////////
        }
    }

    pub fn read_boolean(&mut self) -> Result<bool> {
        let code = self.read_i8()?;
        self.read_boolean_code(code)
    }

    // have reconstruct in buffer, doesnt make sense to returns a slice of a new buffer
    // so this returns a vec whereas read_bytes returns slice view on input.
    fn internal_read_chunked_bytes(&mut self) -> Result<Vec<u8>> {
        let mut buffer: Vec<u8> = Vec::with_capacity(65536);
        let mut code: u8 = codes::BYTES_CHUNK;
        while code == codes::BYTES_CHUNK {
            let count = self.read_count()?;
            buffer.extend_from_slice(self.read_raw_bytes(count as usize)?);
            code = self.read_next_code()? as u8;
        }
        if code != codes::BYTES {
            Err(Error::Syntax)//expected conclusion of chunked bytes///////////////////////////////
        } else {
            // Ok(buffer.as_slice())
            Ok(buffer)
        }
    }

    // this will need to return some wrapper over &[u8] + vec<u8> to support chunked bytes
    pub fn read_bytes_code(&mut self, code: i8) -> Result<&[u8]> {
        match code as u8 {
            codes::BYTES_PACKED_LENGTH_START..=codes::BYTES_PACKED_LENGTH_END => {
                self.read_raw_bytes( (code as u8 - codes::BYTES_PACKED_LENGTH_START) as usize)
            }
            codes::BYTES => {
                let count = self.read_count()?;
                self.read_raw_bytes(count as usize)
            }
            // codes::BYTES_CHUNK => {///////////////////////////////////////////////////////////////
            //     self.internal_read_chunked_bytes()
            // }
            _ => Err(Error::Syntax) // expected bytes, code//////////////////////////////////////
        }
    }

    // this reads of the `fressian bytes` value type, not literal bytes from the reader.
    pub fn read_bytes(&mut self) -> Result<&[u8]> {
        let code = *self.read_u8()?;
        self.read_bytes_code(code as i8)
    }

    /// should this be fn of bytes only?
    pub fn read_raw_utf8(&mut self, length: usize) -> Result<&str>{
        // let length = self.read_count()?;
        let bytes = self.read_raw_bytes(length)?;
        let s: &str = unsafe {
            std::str::from_utf8_unchecked(bytes)
        };
        Ok(s)
    }

    pub fn read_fressian_string(&mut self, length: usize) -> Result<String> {
        if length == 0 {
            Ok("".to_string())
        } else {
            let bytes = self.read_raw_bytes(length)?;
            let length = bytes.len();
            let mut buf: Vec<u16> = Vec::with_capacity(length); //prob min ascii is good guess?
            let mut pos = 0;
            let mut res: Result<()> = Ok(());

            while pos < length && res.is_ok() {
                let ch = bytes[pos] & 0xff;
                pos += 1;
                match ch >> 4 {
                    0..=7 => {
                        buf.push(ch as u16)
                    }
                    12 | 13 => {
                        let ch0 = ch as u32;
                        let ch1 = bytes[pos] as u32 & 0xff;
                        pos += 1;
                        let n = ( (ch0 & 0x1f as u32) << 6 | (ch1 & 0x3f as u32));
                        buf.push(n as u16)
                    }
                    14 => {
                        let ch0 = ch as u32;
                        let ch1 = bytes[pos] as u32;
                        let ch2 = bytes[pos + 1] as u32;
                        pos += 2;
                        let n: u32 = ((ch0 & 0x0f) << 12 | (ch1 & 0x3f) << 6 | ch2 & 0x3f);
                        buf.push(n as u16)
                    }
                    _ => {
                        res = Err(Error::Syntax) /////  throw new RuntimeException(String.format("Invalid UTF-8: %X", ch));/////
                    }
                }
            };

            res.and({
                let s = String::from_utf16(buf.as_slice());
                if s.is_ok(){
                    Ok(s.unwrap())
                } else {
                    Err(Error::Syntax) /////  throw new RuntimeException(String.format("Invalid UTF-8: %X", ch));/////
                }
            })
        }

    }

}


///////////////////////////////////////////////////////////////////////////////////////////////////

mod test {
    use super::*;

    #[test]
    fn read_raw_ints_test(){

        // {:n 0, :switch 64, :fn :raw-byte, :ubytes [0]}
        let data: Vec<u8> = vec![0];
        let mut rdr = RawInput::from_vec(&data);
        assert_eq!(Ok(0), rdr.read_int());

        // {:n 80, :switch 57, :fn :raw-byte, :ubytes [80 80]}
        let data: Vec<u8> = vec![80,80];
        let mut rdr = RawInput::from_vec(&data);
        assert_eq!(Ok(80), rdr.read_int());

        // {:n -80, :switch 57, :fn :raw-byte, :ubytes [79 176], :bytes [79 -80]}
        let data: Vec<u8> = vec![79,176];
        let mut rdr = RawInput::from_vec(&data);
        assert_eq!(Ok(-80), rdr.read_int());

        // {:n 4096, :switch 51, :fn :raw-i16, :ubytes [104 16 0], :bytes [104 16 0]}
        let data: Vec<u8> = vec![104, 16, 0];
        let mut rdr = RawInput::from_vec(&data);
        assert_eq!(Ok(4096), rdr.read_int());

        // {:n -4096, :switch 52, :fn :raw-byte, :ubytes [64 0], :bytes [64 0]}
        let data: Vec<u8> = vec![64, 0];
        let mut rdr = RawInput::from_vec(&data);
        assert_eq!(Ok(-4096), rdr.read_int());


        // {:n 32768, :switch 48, :fn :raw-i16, :ubytes [104 128 0], :bytes [104 -128 0] :form "Short/MAX_VALUE"}
        let data: Vec<u8> = vec![104, 128, 0];
        let mut rdr = RawInput::from_vec(&data);
        assert_eq!(Ok(32768), rdr.read_int());

        // {:value -32768, :form "Short/MIN_VALUE", :n -32768, :switch 49, :fn :raw-i16, :ubytes [103 128 0]}
        let data: Vec<u8> = vec![103, 128, 0];
        let mut rdr = RawInput::from_vec(&data);
        assert_eq!(Ok(-32768), rdr.read_int());

        // {:n -16777216, :switch 40, :fn :raw-i24, :ubytes [113 0 0 0], :bytes [113 0 0 0]}
        let data: Vec<u8> = vec![113, 0, 0, 0];
        let mut rdr = RawInput::from_vec(&data);
        assert_eq!(Ok(-16777216), rdr.read_int());

        // {:n 16777216, :switch 39, :fn :raw-i24, :ubytes [115 0 0 0], :bytes [115 0 0 0]}
        let data: Vec<u8> = vec![115, 0, 0, 0];
        let mut rdr = RawInput::from_vec(&data);
        assert_eq!(Ok(16777216), rdr.read_int());

        // {:value -2147483648, :form "Integer/MIN_VALUE", :switch 33, :fn :raw-i32, :ubytes [117 128 0 0 0], :bytes [117 -128 0 0 0]}
        let data: Vec<u8> = vec![117, 128, 0, 0, 0];
        let mut rdr = RawInput::from_vec(&data);
        assert_eq!(Ok(-2147483648), rdr.read_int());

        // {:value 2147483647, :form "Integer/MAX_VALUE", :switch 33, :fn :raw-i32, :ubytes [118 127 255 255 255], :bytes [118 127 -1 -1 -1]}
        let data: Vec<u8> = vec![118, 127, 255, 255, 255];
        let mut rdr = RawInput::from_vec(&data);
        assert_eq!(Ok(2147483647), rdr.read_int());

        // {:value -549755813887, :form "(long -549755813887)", :switch 25, :fn :raw-i40, :ubytes [121 128 0 0 0 1], :bytes [121 -128 0 0 0 1]}
        let data: Vec<u8> = vec![121, 128, 0,0, 0, 1];
        let mut rdr = RawInput::from_vec(&data);
        assert_eq!(Ok(-549755813887), rdr.read_int());

        // {:value 549755813888, :form "(long 549755813888)", :switch 24, :fn :raw-i40, :ubytes [122 128 0 0 0 0], :bytes [122 -128 0 0 0 0]}
        let data: Vec<u8> = vec![122, 128, 0,0, 0, 0];
        let mut rdr = RawInput::from_vec(&data);
        assert_eq!(Ok(549755813888), rdr.read_int());

        // {:value 140737490000000, :form "(long 1.4073749E14)", :switch 16, :fn :raw-i48, :ubytes [126 128 0 0 25 24 128], :bytes [126 -128 0 0 25 24 -128]}
        let data: Vec<u8> = vec![126, 128, 0, 0, 25, 24, 128];
        let mut rdr = RawInput::from_vec(&data);
        assert_eq!(Ok(140737490000000), rdr.read_int());

        // {:value 9007199254740991, :form "(long  9007199254740991)", :switch 11, :fn :raw-i64, :ubytes [248 0 31 255 255 255 255 255 255], :bytes [-8 0 31 -1 -1 -1 -1 -1 -1]}
        let data: Vec<u8> = vec![248, 0, 31, 255, 255, 255, 255, 255, 255];
        let mut rdr = RawInput::from_vec(&data);
        assert_eq!(Ok(9007199254740991), rdr.read_int());

        // {:value 9007199254740992, :form "(long 9007199254740992)", :switch 10, :fn :raw-i64, :ubytes [248 0 32 0 0 0 0 0 0], :bytes [-8 0 32 0 0 0 0 0 0]}
        let data: Vec<u8> = vec![248, 0, 32, 0, 0, 0, 0, 0, 0];
        let mut rdr = RawInput::from_vec(&data);
        assert_eq!(Ok(9007199254740992), rdr.read_int());

        // {:value -9007199254740991, :form "(long -9007199254740991)", :switch 11, :fn :raw-i64, :ubytes [248 255 224 0 0 0 0 0 1], :bytes [-8 -1 -32 0 0 0 0 0 1]}
        let data: Vec<u8> = vec![248, 255, 224, 0, 0, 0, 0, 0, 1];
        let mut rdr = RawInput::from_vec(&data);
        assert_eq!(Ok(-9007199254740991), rdr.read_int());

        // {:value -9007199254740993, :form "(long -9007199254740993)", :switch 10, :fn :raw-i64, :ubytes [248 255 223 255 255 255 255 255 255], :bytes [-8 -1 -33 -1 -1 -1 -1 -1 -1]}
        let data: Vec<u8> = vec![248, 255, 223, 255, 255, 255, 255, 255, 255];
        let mut rdr = RawInput::from_vec(&data);
        assert_eq!(Ok(-9007199254740993), rdr.read_int());

        // {:value 9223372036854775807, :form "Long/MAX_VALUE", :switch 1, :fn :raw-i64, :ubytes [248 127 255 255 255 255 255 255 255], :bytes [-8 127 -1 -1 -1 -1 -1 -1 -1]}
        let data: Vec<u8> = vec![248, 127, 255, 255, 255, 255, 255, 255, 255];
        let mut rdr = RawInput::from_vec(&data);
        assert_eq!(Ok(9223372036854775807), rdr.read_int());

        // {:value -9223372036854775808, :form "Long/MIN_VALUE", :switch 1, :fn :raw-i64, :ubytes [248 128 0 0 0 0 0 0 0], :bytes [-8 -128 0 0 0 0 0 0 0]}
        let data: Vec<u8> = vec![248, 128, 0, 0, 0, 0, 0, 0, 0];
        let mut rdr = RawInput::from_vec(&data);
        assert_eq!(Ok(-9223372036854775808), rdr.read_int());

    }

    #[test]
    fn read_floats_test (){
        // {:form "(float 32.2)", :bytes [-7 66 0 -52 -51], :ubytes [249 66 0 204 205], :byte-count 5, :footer false, :value 32.2}
        let data: Vec<u8> = vec![249, 66, 0, 204, 205];
        let mut rdr = RawInput::from_vec(&data);
        assert_eq!(Ok(32.2), rdr.read_float());

        // {:form "(float Float/MIN_VALUE)", :bytes [-7 0 0 0 1], :ubytes [249 0 0 0 1], :byte-count 5, :footer false, :value 1.4E-45}
        let data: Vec<u8> = vec![249, 0, 0, 0, 1];
        let mut rdr = RawInput::from_vec(&data);
        assert_eq!(Ok(1.4E-45), rdr.read_float());

        // {:form "(float Float/MAX_VALUE)", :bytes [-7 127 127 -1 -1], :ubytes [249 127 127 255 255], :byte-count 5, :footer false, :value 3.4028235E38}
        let data: Vec<u8> = vec![249, 127, 127, 255, 255];
        let mut rdr = RawInput::from_vec(&data);
        assert_eq!(Ok(3.4028235E38), rdr.read_float());


        // {:form "(java.lang.Double. 4.9E-324)", :bytes [-6 0 0 0 0 0 0 0 1], :ubytes [250 0 0 0 0 0 0 0 1], :byte-count 9, :footer false, :value 4.9E-324}
        let data: Vec<u8> = vec![250, 0, 0, 0, 0, 0, 0, 0, 1];
        let mut rdr = RawInput::from_vec(&data);
        let control: f64 = 4.9E-324;
        assert_eq!(Ok(control), rdr.read_double());

        // {:form "9.8461319849314E10", :bytes [-6 66 54 -20 -64 -126 -87 80 98], :ubytes [250 66 54 236 192 130 169 80 98], :byte-count 9, :footer false, :value 9.8461319849314E10}
        let data: Vec<u8> = vec![250, 66, 54, 236, 192, 130, 169, 80, 98];
        let mut rdr = RawInput::from_vec(&data);
        let control: f64 = 9.8461319849314E10;
        assert_eq!(Ok(control), rdr.read_double());

        // {:form "0.0", :bytes [-5], :ubytes [251], :byte-count 1, :footer false, :value 0.0}
        let data: Vec<u8> = vec![251];
        let mut rdr = RawInput::from_vec(&data);
        let control: f64 = 0.0;
        assert_eq!(Ok(control), rdr.read_double());

        // {:form "1.0", :bytes [-4], :ubytes [252], :byte-count 1, :footer false, :value 1.0}
        let data: Vec<u8> = vec![252];
        let mut rdr = RawInput::from_vec(&data);
        let control: f64 = 1.0;
        assert_eq!(Ok(control), rdr.read_double());

    }

    #[test]
    fn read_boolean_test() {
        // {:form "true", :bytes [-11], :ubytes [245], :byte-count 1, :footer false, :value true}
        let data: Vec<u8> = vec![245];
        let mut rdr = RawInput::from_vec(&data);
        assert_eq!(Ok(true), rdr.read_boolean());

        // {:form "false", :bytes [-10], :ubytes [246], :byte-count 1, :footer false, :value false}
        let data: Vec<u8> = vec![246];
        let mut rdr = RawInput::from_vec(&data);
        assert_eq!(Ok(false), rdr.read_boolean());
    }

    #[test]
    fn read_bytes_test() {
        // {:form "(byte-array [-2 -1 0 1 2])", :bytes [-43 -2 -1 0 1 2], :ubytes [213 254 255 0 1 2], :byte-count 6, :footer false, :input [-2 -1 0 1 2]}
        let data: Vec<u8> = vec![213, 254, 255, 0, 1, 2];
        let control: &[u8] = &[254, 255, 0, 1, 2];
        let mut rdr = RawInput::from_vec(&data);
        assert_eq!(Ok(control), rdr.read_bytes());

        // {:form "(byte-array [-4 -3 -2 -1 0 1 2 3 4])", :bytes [-39 9 -4 -3 -2 -1 0 1 2 3 4], :ubytes [217 9 252 253 254 255 0 1 2 3 4], :byte-count 11, :footer false, :input [-4 -3 -2 -1 0 1 2 3 4]}
        // unpacked length
        let data: Vec<u8> = vec![217, 9, 252, 253, 254, 255, 0, 1, 2, 3, 4];
        let control: &[u8] = &[252, 253, 254, 255, 0, 1, 2, 3, 4];
        let mut rdr = RawInput::from_vec(&data);
        assert_eq!(Ok(control), rdr.read_bytes());

        //missing packed bytes
    }

    fn read_string(rdr: &mut RawInput) -> Result<String> {
        let code = rdr.read_next_code()? as u8;
        match code {
            codes::STRING_PACKED_LENGTH_START..=codes::STRING_PACKED_LENGTH_END => {
                rdr.read_fressian_string((code - codes::STRING_PACKED_LENGTH_START) as usize)
            }
            codes::STRING => {
                let length = rdr.read_count()?;
                rdr.read_fressian_string(length as usize)
            }
            // codes::STRING_CHUNK => {
            //
            // }
            codes::UTF8 => {
                let length = rdr.read_count()?;
                rdr.read_raw_utf8(length as usize).and_then(|s: &str| Ok( s.to_string() ) )
            }
            _ => { Err(Error::Syntax)}
        }
    }

    #[test]
    fn read_fressian_string_test() {

        let data: Vec<u8> = vec![218];
        let control = "".to_string();
        let mut rdr = RawInput::from_vec(&data);
        assert_eq!(Ok(control), read_string(&mut rdr));

        // {:form "\"hola\"", :bytes [-34 104 111 108 97], :ubytes [222 104 111 108 97], :byte-count 5, :footer false, :value "hola"}
        let data: Vec<u8> = vec![222, 104, 111, 108, 97];
        let control = "hola".to_string();
        let mut rdr = RawInput::from_vec(&data);
        assert_eq!(Ok(control), read_string(&mut rdr));

        // {:form "\"é\"", :bytes [-35 101 -52 -127], :ubytes [221 101 204 129], :byte-count 4, :footer false, :value "é"}
        let data: Vec<u8> = vec![221, 101, 204, 129];
        let control = "é".to_string();
        let mut rdr = RawInput::from_vec(&data);
        assert_eq!(Ok(control), read_string(&mut rdr));

        // {:value "❤️", :bytes [-32 -30 -99 -92 -17 -72 -113], :ubytes [224 226 157 164 239 184 143], :byte-count 7, :footer false}
        let data: Vec<u8> = vec![224, 226, 157, 164, 239, 184, 143];
        let control = "❤️".to_string();
        let mut rdr = RawInput::from_vec(&data);
        assert_eq!(Ok(control), read_string(&mut rdr));

        // {:value "😎", :bytes [-32 -19 -96 -67 -19 -72 -114], :ubytes [224 237 160 189 237 184 142], :byte-count 7, :footer false}
        let data: Vec<u8> = vec![224, 237, 160, 189, 237, 184, 142];
        let control = "😎".to_string();
        let mut rdr = RawInput::from_vec(&data);
        assert_eq!(Ok(control), read_string(&mut rdr));

        let data: Vec<u8> = vec![227,60,101,204,129,226,157,164,239,184,143,195,159,226,132,157,230,157,177,228,186,172,230,157,177,228,186,172,237,160,189,237,184,137,32,237,160,189,237,184,142,32,237,160,190,237,180,148,32,237,160,189,237,184,144,32,237,160,189,237,185,132];
        let control = "é❤️ßℝ東京東京😉 😎 🤔 😐 🙄".to_string();
        let mut rdr = RawInput::from_vec(&data);
        assert_eq!(Ok(control), read_string(&mut rdr));
    }
}

