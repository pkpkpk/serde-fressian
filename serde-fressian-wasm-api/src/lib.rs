#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_fressian;

// use std::mem;
use serde::Serialize;
use serde_fressian::ser::{to_vec, Serializer};

#[no_mangle]
pub extern "C" fn hello() -> *const u8 {
    let data = vec!["hello", "from", "wasm!"];
    let buf = Vec::with_capacity(10);

    let mut FressianWriter = Serializer::new(buf);
    data.serialize(&mut FressianWriter).unwrap();
    FressianWriter.write_footer();
    FressianWriter.get_ref().as_ptr()
}