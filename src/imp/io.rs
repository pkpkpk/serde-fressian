use byteorder::{BigEndian, WriteBytesExt};
use error::{Error, Result};
use std::cmp;

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

    pub fn get_bytes_read(&self) -> usize {
        self.bytes_read
    }

    pub fn reset(&mut self) {
        self.bytes_read = 0
    }

    // pub fn validateChecksum(&mut self) -> Result<()> {}

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
    pub fn read_i8(&mut self) -> Result<i8> {
        Ok(*self.read_u8()? as i8)
    }

    pub fn peek_u8(&mut self) -> Result<&u8> {
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

///////////////////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////////////



pub struct ByteWriter<T> {
    out: T,
    bytes_written: u64 //make usize
    //cache: Option<Vec<u8>>
    //checksum: Adler32
}

pub trait IWriteBytes {
    fn write_u8(&mut self, byte: u8) -> Result<()>;

    fn write_bytes(&mut self, bytes: &[u8], off: usize, len: usize) -> Result<()>;

    fn get_bytes_written(&self) -> u64;
}

impl IWriteBytes for ByteWriter<Vec<u8>> {
    fn write_u8(&mut self, byte: u8) -> Result<()> { //abstract out as IWriteBytes?
        vec_write_byte(&mut self.out, self.bytes_written, byte);
        self.notify_bytes_written(1);
        Ok(())
    }

    fn write_bytes(&mut self, bytes: &[u8], off: usize, len: usize) -> Result<()> {
        let buf = &bytes[off as usize .. (off + len) as usize];
        vec_write_bytes(&mut self.out, self.bytes_written as usize, buf);
        self.notify_bytes_written(len as u64);
        Ok(())
    }

    fn get_bytes_written(&self) -> u64 {
        self.bytes_written
    }

    // pub fn getChecksum(&self){
    //     self.checksum.getChecksum()
    // }
}

impl ByteWriter<Vec<u8>> {

    pub fn from_vec(out: Vec<u8>) -> Self {
         ByteWriter{
             bytes_written: 0,
             out: out
         }
    }

    /// returning the underlying bytevec, including any bytes past bytes_written
    /// shrink and return?
    pub fn into_inner(self) -> Vec<u8> { self.out }

    /// Gets a reference to the underlying value
    pub fn get_ref(&self) -> &Vec<u8> { &self.out }

    /// Gets a mutable reference to the underlying value
    pub fn get_mut(&mut self) -> &mut Vec<u8> { &mut self.out }

    pub fn to_vec(&mut self) -> Vec<u8> {
        if self.bytes_written == 0 {
            Vec::new()
        } else {
            // TODO: cache + invalidate on writes
            //should check if byteswritten is same length as vec, if so just clone?
            let mut v: Vec<u8> = Vec::with_capacity(self.bytes_written as usize);
            v.extend_from_slice(&self.out[0..self.bytes_written as usize]);
            return v;
        }
    }

    pub fn reset(&mut self){
        // self.checksum.reset();
        self.bytes_written = 0;
    }

    pub fn notify_bytes_written(&mut self, count: u64) {
        self.bytes_written += count;
    }
}

fn vec_write_byte(vec: &mut Vec<u8>, bytes_written: u64, byte: u8) {
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
}

//this is adapted from std::io::cursor<Vec<u8>>
fn vec_write_bytes(vec: &mut Vec<u8>, pos: usize, buf: &[u8]) {
    // Make sure the internal buffer is as least as big as where we currently are
    let len = vec.len();
    if len < pos {
        // use `resize` so that the zero filling is as efficient as possible
        vec.resize(pos, 0);
    }
    // Figure out what bytes will be used to overwrite what's currently
    // there (left), and what will be appended on the end (right)
    let space = vec.len() - pos;
    let (left, right) = buf.split_at(cmp::min(space, buf.len()));
    vec[pos..pos + left.len()].copy_from_slice(left);
    vec.extend_from_slice(right);
}



mod test {
    use super::*;

    #[test]
    fn read_byte_test (){
        let data: Vec<u8> = vec![0, 1, 2];
        let mut rdr = ByteReader::from_vec(&data);

        assert_eq!(0, rdr.get_bytes_read());
        assert_eq!(Ok(&0), rdr.read_u8());
        assert_eq!(Ok(&1), rdr.read_u8());
        assert_eq!(Ok(&2), rdr.read_u8());
        assert_eq!(3, rdr.get_bytes_read());
        assert_eq!(Err(Error::Eof), rdr.read_u8());
        assert_eq!(3, rdr.get_bytes_read());
        rdr.reset();
        assert_eq!(0, rdr.get_bytes_read());
        assert_eq!(Ok(&0), rdr.read_u8());
    }

    #[test]
    fn read_bytes_test (){
        let data: Vec<u8> = vec![0, 1, 2, 3, 4];
        let mut rdr = ByteReader::from_vec(&data);

        assert_eq!(Err(Error::Eof), rdr.read_bytes(6));
        let control: &[u8] = &[0,1];
        assert_eq!(0, rdr.get_bytes_read());
        assert_eq!(Ok(control), rdr.read_bytes(2));
        assert_eq!(2, rdr.get_bytes_read());
        assert_eq!(Err(Error::Eof), rdr.read_bytes(4));
        let control: &[u8] = &[2,3,4];
        assert_eq!(Ok(control), rdr.read_bytes(3));
        assert_eq!(5, rdr.get_bytes_read());
        assert_eq!(Err(Error::Eof), rdr.read_u8());
        rdr.reset();
        let control: &[u8] = &[0,1,2,3,4];
        assert_eq!(Ok(control), rdr.read_bytes(5));
    }

    #[test]
    fn write_u8_test (){
        let mut wrt = ByteWriter::from_vec(Vec::new());
        wrt.write_u8(99 as u8).unwrap();
        wrt.write_u8(100 as u8).unwrap();
        wrt.write_u8(101 as u8).unwrap();
        let control: Vec<u8> = vec![99, 100, 101];
        assert_eq!(&wrt.to_vec(), &control);
        assert_eq!(wrt.get_bytes_written(), 3);
        wrt.reset();
        assert_eq!(wrt.get_bytes_written(), 0);
        wrt.write_u8(54 as u8).unwrap();
        let control: Vec<u8> = vec![54];
        assert_eq!(&wrt.to_vec(), &control);
    }

    #[test]
    fn write_bytes_test(){
        let mut wrt = ByteWriter::from_vec(Vec::new());
        let v: Vec<u8> = vec![255,254,253,0,1,2,3];
        wrt.write_bytes(v.as_slice(), 0, v.len()).unwrap();
        assert_eq!(&wrt.to_vec(), &v);
        wrt.reset();
        assert_eq!(&wrt.to_vec(), &vec![]);
        wrt.write_bytes(v.as_slice(), 2, 3).unwrap();
        assert_eq!(&wrt.to_vec(), &vec![253, 0, 1]);
    }
}