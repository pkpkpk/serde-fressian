
use std::mem;
use std::os::raw::{c_void};
use std::error::Error;

use de::{self};
use ser::{self, Serializer};
use imp::io::{ByteWriter, IWriteBytes};
use error::{self};

use serde::de::{Deserialize};
use serde::ser::{Serialize, SerializeMap};

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

#[inline]
fn vec_to_ptr(mut vec: Vec<u8>) -> *mut c_void {
    let ptr = vec.as_mut_ptr();
    mem::forget(vec);
    return ptr as *mut c_void;
}


/// Serialize to a vec and give ownership to javascript via a pointer.
///   - care should be taken by javascript consumers to read and free the bytes
///     synchronously and before any other calls/writes to the wasm module
///      - fress.wasm/read does this for you
///   - You can use Result<T,E> where T,E: Serialize; errors will be written
///     with an error code and picked up by the fress client as such
pub fn to_js<S: Serialize>(value: S) -> *mut c_void {
    let vec: Vec<u8> = ser::to_vec(&value).unwrap_or_else(|err| ser::to_vec(&err).unwrap());
    vec_to_ptr(vec)
}

