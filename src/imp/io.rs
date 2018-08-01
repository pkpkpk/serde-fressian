// use byteorder::*;
use imp::error::{Error, Result};

pub struct ByteReader<'a> {
    input: &'a Vec<u8>,
    bytes_read: usize
}

// pub trait IReadBytes {
//     pub fn read_u8(&mut self) -> Result<&u8>;
//     pub fn read_bytes(&mut self, length: usize) -> Result<&[u8]>;
//     pub fn reset(&mut self);
//     pub fn get_bytes_read(&self) -> usize;
//     pub fn notify_bytes_read(&mut self, count: usize);
// }


impl<'a> ByteReader<'a>{
    pub fn from_vec(v: &'a Vec<u8>) -> ByteReader {
        ByteReader{
            input: v,
            bytes_read: 0
        }
    }

    pub fn notify_bytes_read(&mut self, count: usize){
        self.bytes_read += count;
    }

    pub fn get_bytes_read(&self) -> usize {
        self.bytes_read
    }

    pub fn reset(&mut self) {
        self.bytes_read = 0
    }

    pub fn read_u8(&mut self) -> Result<&u8> {
        match self.input.get(self.bytes_read) {
            Some(byte) => {
                self.notify_bytes_read(1);
                Ok(byte)
            }
            None => {
                Err(Error::Eof)
            }
        }
    }

    pub fn read_bytes(&mut self, length: usize) -> Result<&[u8]>{
        if length == 0 {
            Err(Error::Syntax)
        } else {
            let end = self.bytes_read + length;
            if self.input.len() < end {
                Err(Error::Eof)
            } else {
                let start = self.bytes_read;
                self.notify_bytes_read(length);
                let bytes: &[u8] = &self.input[start..end];
                Ok(bytes)
            }
        }
    }
}