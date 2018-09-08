
use std::mem;
use std::os::raw::{c_void};
use serde::Serialize;
use ser::{self,Serializer};
use de::{self};
use imp::io::{ByteWriter, IWriteBytes};

#[no_mangle]
pub extern "C" fn fress_alloc(size: usize) -> *mut c_void {
    let mut buf = Vec::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    mem::forget(buf);
    return ptr as *mut c_void;
}

pub fn ptr_to_vec(ptr: *mut u8, cap: usize) -> Vec<u8>
{
    unsafe  {
        std::slice::from_raw_parts(ptr, cap).to_vec()
    }
}

pub fn to_js<T: Serialize>(value: T) -> *mut c_void {
    let mut vec: Vec<u8> = ser::to_vec_footer(&value).unwrap();
    let ptr = vec.as_mut_ptr();
    mem::forget(vec);
    return ptr as *mut c_void;
}

