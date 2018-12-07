#![allow(exceeding_bitshifts)]

/// RawOutput is responsible for converting types into their fressian
/// byte representations and handing off those bytes to the destination writer.
/// RawOutput has no state of its own, its just a collection of functions that
/// interface with the writer.

extern crate serde;

use crate::error::{Result};
use crate::imp::io::{IWriteBytes};
use crate::imp::codes;
use crate::imp::ranges;
use std::cmp;

#[derive(Clone, Debug)]
pub struct RawOutput;

impl RawOutput {

    pub fn write_raw_i16<B: ?Sized>(&mut self, writer: &mut B, i: i32) -> Result<()>
        where
            B: IWriteBytes,
    {
        writer.write_u8(((i >>  8) & 0xFF) as u8)?;
        writer.write_u8(        (i & 0xFF) as u8)
    }

    pub fn write_raw_i24<B: ?Sized>(&mut self, writer: &mut B, i: i32) -> Result<()>
        where
            B: IWriteBytes,
    {
        writer.write_u8(((i >> 16) & 0xFF) as u8)?;
        writer.write_u8(((i >>  8) & 0xFF) as u8)?;
        writer.write_u8(        (i & 0xFF) as u8)
    }

    pub fn write_raw_i32<B: ?Sized>(&mut self, writer: &mut B, i: i32) -> Result<()>
        where
            B: IWriteBytes,
    {
        writer.write_u8(((i >> 24) & 0xFF) as u8)?;
        writer.write_u8(((i >> 16) & 0xFF) as u8)?;
        writer.write_u8(((i >>  8) & 0xFF) as u8)?;
        writer.write_u8(        (i & 0xFF) as u8)
    }

    pub fn write_raw_i40<B: ?Sized>(&mut self, writer: &mut B, i: i64) -> Result<()>
        where
            B: IWriteBytes,
    {
        writer.write_u8(((i >> 32) & 0xFF) as u8)?;
        writer.write_u8(((i >> 24) & 0xFF) as u8)?;
        writer.write_u8(((i >> 16) & 0xFF) as u8)?;
        writer.write_u8(((i >>  8) & 0xFF) as u8)?;
        writer.write_u8(        (i & 0xFF) as u8)
    }

    pub fn write_raw_i48<B: ?Sized>(&mut self, writer: &mut B, i: i64) -> Result<()>
        where
            B: IWriteBytes,
    {
        writer.write_u8(((i >> 40) & 0xFF) as u8)?;
        writer.write_u8(((i >> 32) & 0xFF) as u8)?;
        writer.write_u8(((i >> 24) & 0xFF) as u8)?;
        writer.write_u8(((i >> 16) & 0xFF) as u8)?;
        writer.write_u8(((i >>  8) & 0xFF) as u8)?;
        writer.write_u8(        (i & 0xFF) as u8)
    }

    pub fn write_raw_i64<B: ?Sized>(&mut self, writer: &mut B, i: i64) -> Result<()>
        where
            B: IWriteBytes,
    {
        writer.write_u8((i >> 56) as u8)?;
        writer.write_u8((i >> 48) as u8)?;
        writer.write_u8((i >> 40) as u8)?;
        writer.write_u8((i >> 32) as u8)?;
        writer.write_u8((i >> 24) as u8)?;
        writer.write_u8((i >> 16) as u8)?;
        writer.write_u8((i >>  8) as u8)?;
        writer.write_u8((i >>  0) as u8)
    }

    pub fn write_raw_float<B: ?Sized>(&mut self, writer: &mut B, f: f32) -> Result<()>
        where
            B: IWriteBytes,
    {
        self.write_raw_i32(writer, f.to_bits() as i32)
    }

    pub fn write_raw_double<B: ?Sized>(&mut self, writer: &mut B, f: f64) -> Result<()>
        where
            B: IWriteBytes,
    {
        self.write_raw_i64(writer, f.to_bits() as i64)
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////

    pub fn write_code<B: ?Sized>(&mut self, writer: &mut B, code: u8) -> Result<()>
        where
            B: IWriteBytes,
    {
        writer.write_u8(code)
    }

    pub fn write_int<B: ?Sized>(&mut self, writer: &mut B, i: i64) -> Result<()>
        where
            B: IWriteBytes,
    {
        match bit_switch(i) {
            1..=14 => {
                writer.write_u8(codes::INT)?;
                self.write_raw_i64(writer, i)
            }

            15..=22 => {
                writer.write_u8(codes::INT_PACKED_7_ZERO.wrapping_add( (i >> 48) as u8 ))?;
                self.write_raw_i48(writer,i)
            }

            23..=30 => {
                writer.write_u8(codes::INT_PACKED_6_ZERO.wrapping_add( (i >> 40) as u8 ))?;
                self.write_raw_i40(writer,i)
            }

            31..=38 => {
                writer.write_u8(codes::INT_PACKED_5_ZERO.wrapping_add( (i >> 32) as u8 ))?;
                self.write_raw_i32(writer,i as i32)
            }

            39..=44 => {
                writer.write_u8(codes::INT_PACKED_4_ZERO.wrapping_add( (i >> 24) as u8))?;
                self.write_raw_i24(writer,i as i32)
            }

            45..=51 => {
                writer.write_u8(codes::INT_PACKED_3_ZERO.wrapping_add( (i >> 16) as u8))?;
                self.write_raw_i16(writer,i as i32)
            }

            52..=57 => {
                writer.write_u8(codes::INT_PACKED_2_ZERO.wrapping_add( (i >> 8) as u8))?;
                writer.write_u8(i as u8)
            }

            58..=64 => {
                if i < -1 {
                    writer.write_u8(codes::INT_PACKED_2_ZERO.wrapping_add( (i >> 8) as u8))?;
                    writer.write_u8(i as u8)
                } else {
                    writer.write_u8(i as u8)
                }
            }

            _ => Err(serde::de::Error::custom("more than 64 bits in a long!"))///////////////////////////////
        }
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////
    ///////////////////////////////////////////////////////////////////////////////////////////////

    pub fn write_float<B: ?Sized>(&mut self, writer: &mut B, f: f32) -> Result<()>
        where
            B: IWriteBytes,
    {
        writer.write_u8(codes::FLOAT)?;
        self.write_raw_float(writer,f)
    }


    pub fn write_double<B: ?Sized>(&mut self, writer: &mut B, f: f64) -> Result<()>
        where
            B: IWriteBytes,
    {
        if f == 0.0 {
            writer.write_u8(codes::DOUBLE_0)
        } else if f == 1.0 {
            writer.write_u8(codes::DOUBLE_1)
        } else {
            writer.write_u8(codes::DOUBLE)?;
            self.write_raw_double(writer,f)
        }
    }

    pub fn write_count<B: ?Sized>(&mut self, writer: &mut B, count: usize) -> Result<()>
        where
            B: IWriteBytes,
    {
        self.write_int(writer, count as i64)
    }

    pub fn write_null<B: ?Sized>(&mut self, writer: &mut B) -> Result<()>
        where
            B: IWriteBytes,
    {
        writer.write_u8(codes::NULL)
    }

    pub fn write_boolean<B: ?Sized>(&mut self, writer: &mut B, b: bool) -> Result<()>
        where
            B: IWriteBytes,
    {
        if b {
            writer.write_u8(codes::TRUE)
        } else {
            writer.write_u8(codes::FALSE)
        }
    }

    pub fn write_bytes<B: ?Sized>(&mut self, writer: &mut B, bytes: &[u8], offset: usize, length: usize) -> Result<()>
        where
            B: IWriteBytes,
    {
        if length < ranges::BYTES_PACKED_LENGTH_END {
            writer.write_u8(codes::BYTES_PACKED_LENGTH_START + length as u8)?;
            writer.write_bytes(bytes, offset,length)
        } else {
            let mut length = length;
            let mut offset = offset;
            while ranges::BYTE_CHUNK_SIZE < length {
                writer.write_u8(codes::BYTES_CHUNK)?;
                self.write_count(writer, ranges::BYTE_CHUNK_SIZE)?;
                writer.write_bytes(bytes, offset, ranges::BYTE_CHUNK_SIZE)?;
                offset += ranges::BYTE_CHUNK_SIZE;
                length -= ranges::BYTE_CHUNK_SIZE;
            };
            writer.write_u8(codes::BYTES)?;
            self.write_count(writer,length)?;
            writer.write_bytes(bytes, offset, length)
        }
    }

    #[cfg(not(raw_UTF8))]
    pub fn write_string<B: ?Sized>(&mut self, writer: &mut B, s: &str) -> Result<()>
        where
            B: IWriteBytes,
    {
        let char_length: usize = s.chars().count();

        if char_length == 0 {
            writer.write_u8(codes::STRING_PACKED_LENGTH_START)?;
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
                    writer.write_u8(codes::STRING_PACKED_LENGTH_START.wrapping_add( buf_pos as u8))?;
                } else if j_string_pos == j_char_length {
                    writer.write_u8(codes::STRING)?;
                    self.write_count(writer,buf_pos)?;
                } else {
                    writer.write_u8(codes::STRING_CHUNK)?;
                    self.write_count(writer,buf_pos)?;
                }
                writer.write_bytes(&buffer,0,buf_pos)?;
            }
        }

        Ok(())
    }

    #[cfg(raw_UTF8)]
    pub fn write_string<B: ?Sized>(&mut self, writer: &mut B, s: &str) -> Result<()>
        where
            B: IWriteBytes,
    {
        let bytes = s.as_bytes();
        let length = bytes.len();
        writer.write_u8(codes::UTF8)?;
        self.write_count(writer,length)?;
        writer.write_bytes(&bytes.to_vec(), 0, length)
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
