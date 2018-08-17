// need optional checksum

use imp::error::{Error, Result};

pub struct ByteReader<'a> {
    input: &'a [u8],
    bytes_read: usize
}


impl<'a> ByteReader<'a> {
    pub fn new(bytes: &'a [u8] ) -> Self {
        ByteReader {
            input: bytes,
            bytes_read: 0
        }
    }

    pub fn from_vec(v: &'a Vec<u8>) -> Self {
        ByteReader::new(v.as_slice())
    }

    pub fn notify_bytes_read(&mut self, count: usize){
        self.bytes_read += count;
    }

    pub fn read_u8(&mut self) -> Result<&u8> { /////// change to just return byte
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
    pub fn read_i8(&mut self) -> Result<i8> { /////// change to just return byte
        Ok(*self.read_u8()? as i8)
    }

    pub fn peek_u8(&mut self) -> Result<&u8> { /////// change to just return byte
        match self.input.get(self.bytes_read) {
            Some(byte) => {
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
