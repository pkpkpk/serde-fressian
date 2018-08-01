#![allow(exceeding_bitshifts)]

extern crate serde;

use imp::error::{Error, Result};
use imp::io::{ByteWriter};
use std::cmp;

type RawOutput = ByteWriter;

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

