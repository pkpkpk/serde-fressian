// use byteorder::*;
use imp::error::{Error, Result};
use std::cmp;

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

    pub fn read_i8(&mut self) -> Result<i8> {
        match self.input.get(self.bytes_read) {
            Some(byte) => {
                self.notify_bytes_read(1);
                Ok(*byte as i8)
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

    // pub fn validateChecksum(&mut self) -> Result<()> {}

    // pub fn close(&mut self) {}
}


///////////////////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////////////


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
    // Ok(())
}

fn vec_write_bytes(vec: &mut Vec<u8>, pos: u64, buf: &[u8]) { // -> Result<()>
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
    // Ok(())
}

pub struct ByteWriter {
    out: Vec<u8>,
    bytes_written: u64 //make usize
    //cache: Option<Vec<u8>>
    //checksum: Adler32
}

impl ByteWriter {

    pub fn from_vec(out: Vec<u8>) -> ByteWriter {
         ByteWriter{
             bytes_written: 0,
             out: out
         }
    }

    /// returning the underlying bytevec, including any bytes past bytes_written
    pub fn into_inner(self) -> Vec<u8> { self.out }

    /// Gets a reference to the underlying value
    pub fn get_ref(&self) -> &Vec<u8> { &self.out }

    /// Gets a mutable reference to the underlying value
    pub fn get_mut(&mut self) -> &mut Vec<u8> { &mut self.out }

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

    pub fn reset(&mut self){
        // self.checksum.reset();
        self.bytes_written = 0;
    }

    pub fn notify_bytes_written(&mut self, count: u64) {
        self.bytes_written += count;
    }

    pub fn get_bytes_written(&self) -> u64 {
        self.bytes_written
    }

    pub fn write_raw_byte(&mut self, byte: u8) -> Result<()> { //abstract out as IWriteBytes?
        vec_write_byte(&mut self.out, self.bytes_written, byte);
        self.notify_bytes_written(1);
        Ok(())
    }

    pub fn write_raw_bytes(&mut self, bytes: &[u8], off: usize, len: usize) -> Result<()> {
        let buf = &bytes[off as usize .. (off + len) as usize];
        vec_write_bytes(&mut self.out, self.bytes_written, buf);
        self.notify_bytes_written(len as u64);
        Ok(())
    }

    // pub fn getChecksum(&self){
    //     self.checksum.getChecksum()
    // }
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
    fn write_raw_byte_test (){
        let mut wrt = ByteWriter::from_vec(Vec::new());
        wrt.write_raw_byte(99 as u8).unwrap();
        wrt.write_raw_byte(100 as u8).unwrap();
        wrt.write_raw_byte(101 as u8).unwrap();
        let control: Vec<u8> = vec![99, 100, 101];
        assert_eq!(&wrt.to_vec(), &control);
        assert_eq!(wrt.get_bytes_written(), 3);
        wrt.reset();
        assert_eq!(wrt.get_bytes_written(), 0);
        wrt.write_raw_byte(54 as u8).unwrap();
        let control: Vec<u8> = vec![54];
        assert_eq!(&wrt.to_vec(), &control);
    }

    #[test]
    fn write_raw_bytes_test(){
        let mut wrt = ByteWriter::from_vec(Vec::new());
        let v: Vec<u8> = vec![255,254,253,0,1,2,3];
        wrt.write_raw_bytes(v.as_slice(), 0, v.len()).unwrap();
        assert_eq!(&wrt.to_vec(), &v);
        wrt.reset();
        assert_eq!(&wrt.to_vec(), &vec![]);
        wrt.write_raw_bytes(v.as_slice(), 2, 3).unwrap();
        assert_eq!(&wrt.to_vec(), &vec![253, 0, 1]);
    }
}