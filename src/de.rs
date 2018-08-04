use std::ops::{AddAssign, MulAssign, Neg};

use serde::de::{
    self, Deserialize, DeserializeSeed, EnumAccess, IntoDeserializer,
    MapAccess, SeqAccess, VariantAccess, Visitor,
};

use imp::error::{Error, Result};
use imp::RawInput::{RawInput};
// use imp::codes;

pub struct Deserializer<'a>{
    rawIn: RawInput<'a>
}

impl<'a> Deserializer<'a> {

    pub fn from_vec(v: &'a Vec<u8>) -> Deserializer {
        Deserializer {
            rawIn: RawInput::from_vec(v)
        }
    }

    pub fn read_next_code(&mut self) -> Result<i8> {
        self.rawIn.read_i8()
    }

    pub fn read_int(&mut self) -> Result<i64> {
        self.rawIn.read_int()
    }

    // pub fn read_int_code(&mut self, code: i8) -> Result<i64> {
    //     self.rawIn.read_int_code(code)
    // }

    pub fn read_float(&mut self) -> Result<f32> {
        self.rawIn.read_float()
    }

    pub fn read_double(&mut self) -> Result<f64> {
        self.rawIn.read_double()
    }

    // fn read_code(&mut self, code: i8) -> Result<> {}

}
