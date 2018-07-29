
use imp::error::{Error, Result};

pub struct RawInput<'a> {
    input: &'a Vec<u8>,
    bytes_read: usize
}

impl<'a> RawInput<'a> { //< &'a Vec<u8>>
    pub fn new(input: &'a Vec<u8>) -> RawInput {
        RawInput{
            input: input,
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

    pub fn read_byte(&mut self) -> Result<&u8> {
        match self.input.get(self.bytes_read) {
            Some(byte) => {
                self.notify_bytes_read(1);
                // Ok(byte.clone())
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
    ///////////////////////////////////////////////////////////////////////////////////////////////
    pub fn read_raw_i16(&mut self) -> Result<i64> {
        let high = *self.read_byte()?;
        let low  = *self.read_byte()?;
        Ok((high as i64) << 8 + low)
    }

    pub fn read_raw_i24(&mut self) -> Result<i64> {
        let a = *self.read_byte()? as i64;
        let b = *self.read_byte()? as i64;
        let c = *self.read_byte()? as i64;
        Ok((a << 16) + (b << 8) + c)
    }

    pub fn read_raw_i32(&mut self) -> Result<i64> {
        let a = *self.read_byte()? as i64 & std::u8::MAX as i64;
        let b = *self.read_byte()? as i64 & std::u8::MAX as i64;
        let c = *self.read_byte()? as i64 & std::u8::MAX as i64;
        let d = *self.read_byte()? as i64 & std::u8::MAX as i64;
        Ok( ((a << 24) | (b << 16) | (c << 8) | d) & std::u32::MAX as i64)
    }

    pub fn read_raw_i40(&mut self) -> Result<i64> {
        let high = *self.read_byte()? as i64;
        let low = self.read_raw_i32()?;
        Ok( (high << 32) + low )
    }

    pub fn read_raw_i48(&mut self) -> Result<i64> {
        let high = *self.read_byte()? as i64;
        let low = self.read_raw_i40()?;
        Ok( (high << 40) + low )
    }

    pub fn read_raw_i64(&mut self) -> Result<i64> {
        let a = *self.read_byte()? as i64 & std::u8::MAX as i64;
        let b = *self.read_byte()? as i64 & std::u8::MAX as i64;
        let c = *self.read_byte()? as i64 & std::u8::MAX as i64;
        let d = *self.read_byte()? as i64 & std::u8::MAX as i64;
        let e = *self.read_byte()? as i64 & std::u8::MAX as i64;
        let f = *self.read_byte()? as i64 & std::u8::MAX as i64;
        let g = *self.read_byte()? as i64 & std::u8::MAX as i64;
        let h = *self.read_byte()? as i64 & std::u8::MAX as i64;
        Ok( (a << 56) | (b << 48) | (c << 40) | (d << 32) | (e << 24) | (f << 16) | (g << 8) | h )
    }

    // pub fn read_raw_float(&mut self) -> Result<f32> {}

    // pub fn read_raw_double(&mut self) -> Result<f64> {}

    // pub fn validateChecksum(&mut self) -> Result<()> {}

    // pub fn close(&mut self) {}

}


#[test]
fn read_byte_test (){
    let data: Vec<u8> = vec![0, 1, 2];
    let mut rdr = RawInput::new(&data);

    assert_eq!(0, rdr.get_bytes_read());
    assert_eq!(Ok(&0), rdr.read_byte());
    assert_eq!(Ok(&1), rdr.read_byte());
    assert_eq!(Ok(&2), rdr.read_byte());
    assert_eq!(3, rdr.get_bytes_read());
    assert_eq!(Err(Error::Eof), rdr.read_byte());
    assert_eq!(3, rdr.get_bytes_read());
    rdr.reset();
    assert_eq!(0, rdr.get_bytes_read());
    assert_eq!(Ok(&0), rdr.read_byte());
}

#[test]
fn read_bytes_test (){
    let data: Vec<u8> = vec![0, 1, 2, 3, 4];
    let mut rdr = RawInput::new(&data);

    assert_eq!(Err(Error::Eof), rdr.read_bytes(6));
    let control: &[u8] = &[0,1];
    assert_eq!(0, rdr.get_bytes_read());
    assert_eq!(Ok(control), rdr.read_bytes(2));
    assert_eq!(2, rdr.get_bytes_read());
    assert_eq!(Err(Error::Eof), rdr.read_bytes(4));
    let control: &[u8] = &[2,3,4];
    assert_eq!(Ok(control), rdr.read_bytes(3));
    assert_eq!(5, rdr.get_bytes_read());
    assert_eq!(Err(Error::Eof), rdr.read_byte());
    rdr.reset();
    let control: &[u8] = &[0,1,2,3,4];
    assert_eq!(Ok(control), rdr.read_bytes(5));
}