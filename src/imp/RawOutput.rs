#![allow(exceeding_bitshifts)]

extern crate serde;

mod error {
    pub use serde::de::value::Error;
    pub type Result<T> = ::std::result::Result<T, Error>;
}

use self::error::{Error, Result};
use std::cmp;

///////////////////////////////////////////////////////////////////////////////////////////////////
fn vec_write_byte(vec: &mut Vec<u8>, bytes_written: u64, byte: u8) -> Result<()> {
    let bytes_written = bytes_written as usize;
    if bytes_written == 0 {
        if vec.len() == 0 {
            vec.push(byte);
        } else {
            vec[0] = byte;
        }
    } else {
        if bytes_written < vec.len() {
            vec[bytes_written] = byte;
        } else {
            vec.push(byte);
        }
    }
    Ok(())
}

fn vec_write_bytes(vec: &mut Vec<u8>, pos: u64, buf: &[u8]) -> Result<()> {
    let pos = pos as usize;
    // Make sure the internal buffer is as least as big as where we currently are
    let len = vec.len();
    if len < pos {
        // use `resize` so that the zero filling is as efficient as possible
        vec.resize(pos, 0);
    }
    // Figure out what bytes will be used to overwrite what's currently
    // there (left), and what will be appended on the end (right)
    {
        let space = vec.len() - pos;
        let (left, right) = buf.split_at(cmp::min(space, buf.len()));
        vec[pos..pos + left.len()].copy_from_slice(left);
        vec.extend_from_slice(right);
    }
    Ok(())
}

///////////////////////////////////////////////////////////////////////////////////////////////////

pub trait IWriteBytes {
    fn write_raw_byte(&mut self, bytes_written: u64, byte: u8) -> Result<()>;

    fn write_raw_bytes(&mut self, pos: u64, bytes: &Vec<u8>, off: usize, len: usize) -> Result<()>;
}

impl IWriteBytes for Vec<u8> {
    fn write_raw_byte(&mut self, bytes_written: u64, byte: u8) -> Result<()> { //abstract out as IWriteBytes?
        vec_write_byte(self, bytes_written, byte)
    }
    fn write_raw_bytes(&mut self, pos: u64, bytes: &Vec<u8>, off: usize, len: usize) -> Result<()> {
        let buf = &bytes[off as usize .. (off + len) as usize];
        vec_write_bytes(self, pos, buf)
    }
}
///////////////////////////////////////////////////////////////////////////////////////////////////

pub struct RawOutput {
    out: Vec<u8>,
    bytes_written: u64
    //cache: Option<Vec<u8>>
    // checksum: Adler32
}

impl RawOutput {
    pub fn new(out: Vec<u8>) -> RawOutput {
         RawOutput{
             bytes_written: 0,
             out: out
         }
    }

    // TODO: cache + invalidate on writes
    pub fn to_vec(&mut self) -> Vec<u8> {
        if self.bytes_written == 0 {
            Vec::new()
        } else {
            let mut v: Vec<u8> = Vec::with_capacity(self.bytes_written as usize);
            v.extend_from_slice(&self.out[0..self.bytes_written as usize]);
            return v;
        }
    }

    /// returning the underlying bytevec, including any bytes past bytes_written
    pub fn into_inner(self) -> Vec<u8> { self.out }

    /// Gets a reference to the underlying value
    pub fn get_ref(&self) -> &Vec<u8> { &self.out }

    /// Gets a mutable reference to the underlying value
    pub fn get_mut(&mut self) -> &mut Vec<u8> { &mut self.out }

    pub fn get_bytes_written(&self) -> u64 {
        self.bytes_written
    }

    fn notify_bytes_written(&mut self, count: u64) {
        self.bytes_written += count;
    }

    pub fn reset(&mut self){
        // self.checksum.reset();
        self.bytes_written = 0;
    }

    // pub fn getChecksum(&self){
    //     self.checksum.getChecksum()
    // }

    pub fn write_raw_byte(&mut self, byte: u8) -> Result<()>{
        self.out.write_raw_byte(self.bytes_written, byte)?;
        self.notify_bytes_written(1);
        Ok(())
    }

    pub fn write_raw_bytes(&mut self, bytes: &Vec<u8>, off: usize, len: usize) -> Result<()> {
        self.out.write_raw_bytes(self.bytes_written, bytes, off, len)?;
        self.notify_bytes_written(len as u64);
        Ok(())
    }

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

