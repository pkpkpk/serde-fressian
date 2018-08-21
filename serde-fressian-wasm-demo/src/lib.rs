#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_fressian;

// use std::mem;
use serde::Serialize;
use serde_fressian::ser::{Serializer};

#[no_mangle]
pub extern "C" fn hello() -> *const u8 {
    let data = vec![["hello", "from", "wasm!"], ["isn't", "this", "exciting?!"]];

    let mut fressian_writer = Serializer::new();
    data.serialize(&mut fressian_writer).unwrap();
    fressian_writer.write_footer();
    fressian_writer.get_ref().as_ptr()
}