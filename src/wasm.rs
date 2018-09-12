
use std::mem;
use std::os::raw::{c_void};
use de::{self};
use ser::{self};
use imp::io::{ByteWriter, IWriteBytes};
use error::{self};

use serde::de::{Deserialize};
use serde::ser::{Serialize, Serializer, SerializeMap};

impl Serialize for error::Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_newtype_struct("ERROR", &self.err)
    }
}

/// return a pointer to available memory of given byte length
#[no_mangle]
pub extern "C" fn fress_alloc(size: usize) -> *mut c_void {
    let mut buf = Vec::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    mem::forget(buf);
    return ptr as *mut c_void;
}

/// given a pointer to bytes originally requested by fress_alloc
/// and written-to by javascript, reclaim them as a Vec<u8>
pub fn ptr_to_vec(ptr: *mut u8, cap: usize) -> Vec<u8> {
    unsafe {
        std::slice::from_raw_parts(ptr, cap).to_vec()
    }
}

/// return a Vec<u8> back to rust and drop it
#[no_mangle]
pub extern "C" fn fress_free(ptr: *mut u8, cap: usize) {
    let _ = ptr_to_vec(ptr, cap);
}

pub fn to_js<S: Serialize>(value: S) -> *mut c_void {
    let mut vec: Vec<u8> = ser::to_vec_footer(&value).unwrap_or_else(|err| ser::to_vec_footer(&err).unwrap());
    let ptr = vec.as_mut_ptr();
    mem::forget(vec);
    return ptr as *mut c_void;
}

