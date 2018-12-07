use std::mem;

use crate::de::{self};
use crate::ser::{self};
use crate::error::{self};

// use serde::de;
use serde::ser::{Serialize};

/// Transfer ownership of slice of memory to javascript, and return a pointer.
///   + This is for giving javascript memory to write into. It assumes the given vec has len == 0
///     but has some non-zero allocated capacity that javascript requested
///   + These bytes are unreachable to rust until they have been returned manually.
///      - see https://doc.rust-lang.org/std/mem/fn.forget.html
#[inline]
fn buffer_to_js(mut vec: Vec<u8>) -> *mut u8
{
    assert_eq!(0, vec.len());
    let ptr = vec.as_mut_ptr();
    mem::forget(vec);
    ptr
}

/// Transfer ownership of slice of _bytes_ to javascript, and return a pointer.
///   + This is for giving javascript bytes to read. It differs from `buffer_to_js` only in that it
///     calls `vec.shrink_to_fit()` which trims off excess capacity to prevent a leak.
///   + These bytes are unreachable to rust until they have been returned manually.
///      - see https://doc.rust-lang.org/std/mem/fn.forget.html
#[inline]
pub fn bytes_to_js(mut vec: Vec<u8>) -> *mut u8
{
    vec.shrink_to_fit();
    let ptr = vec.as_mut_ptr();
    mem::forget(vec);
    ptr
}

/// for js consumers: request memory of the given byte length and return a pointer to it
#[no_mangle]
pub extern "C" fn fress_alloc(byte_len: usize) -> *mut u8
{
    buffer_to_js(Vec::with_capacity(byte_len))
}

/// for js consumers: return ownership of the bytes back to rust and drop them
#[no_mangle]
pub extern "C" fn fress_dealloc(ptr: *mut u8, cap: usize)
{
    let _buf: Vec<u8> =  unsafe {
        Vec::from_raw_parts(ptr, cap, cap)
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
    let vec: Vec<u8> = ser::to_vec(&value).unwrap_or_else(|err| {
        let res: Result<(), error::Error> = Err(err);
        ser::to_vec(&res).unwrap()
    });
    bytes_to_js(vec)
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


use std::panic;

extern {
    fn js_panic_hook(ptr: *mut u8);
}

pub fn hook(info: &panic::PanicInfo) {
    let msg: String = info.to_string();
    unsafe {
        js_panic_hook(to_js(msg))
    }
}

#[no_mangle]
pub extern "C" fn fress_init() {
    panic::set_hook(Box::new(hook));
}


