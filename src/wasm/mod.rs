use std::mem;

use de::{self};
use ser::{self};
use error::{self};

// use serde::de;
use serde::ser::{Serialize};

/// Transfer ownership of the bytes to javascript, and return a pointer to them.
/// These bytes are unreachable to rust until they have been returned manually.
/// see https://doc.rust-lang.org/std/mem/fn.forget.html
#[inline]
fn vec_to_js(mut vec: Vec<u8>) -> *mut u8
{
    let ptr = vec.as_mut_ptr();
    // let len = vec.len();
    mem::forget(vec);
    ptr
}

/// for js consumers: request memory of the given byte length and return a pointer to it
#[no_mangle]
pub extern "C" fn fress_alloc(size: usize) -> *mut u8
{
    let buf = Vec::with_capacity(size);
    vec_to_js(buf)
}

/// for js consumers: return ownership of the bytes back to rust and drop them
#[no_mangle]
pub extern "C" fn fress_dealloc(ptr: *mut u8, len: usize)
{
    let _buf: Vec<u8> =  unsafe {
        Vec::from_raw_parts(ptr, len, len)
    };
}

/// Serialize to bytes and give ownership to javascript via a pointer.
///   - care should be taken by javascript consumers to read and free the bytes
///     synchronously and before any other calls/writes to the wasm module
///      - fress.wasm/read does this for you
///   - You can use Result<T,E> where T,E: Serialize; errors will be written
///     with an error code and picked up by the fress client as such
pub fn to_js<S: Serialize>(value: S) -> *mut u8
{
    let vec: Vec<u8> = ser::to_vec(&value).unwrap_or_else(|err| ser::to_vec(&err).unwrap());
    vec_to_js(vec)
}

/// Given a pointer and length from javascript, deserialize fressian bytes to rust data structures.
/// This does not take ownership of the bytes pointed to! The pointer & length should be kept so
/// that the bytes can be returned and dropped according to the needs of your deserialized value.
pub fn from_ptr<'a,T>(ptr: *mut u8, len: usize) -> error::Result<T>
where
    T: serde::de::Deserialize<'a>,
{
    let bytes: &[u8] =  unsafe {
        std::slice::from_raw_parts(ptr, len)
    };
    let mut deserializer = de::Deserializer::from_bytes(bytes);
    T::deserialize(&mut deserializer)
}
