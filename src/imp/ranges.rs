#![allow(dead_code)]

pub const PACKED_1_START: i64 = -1;
pub const PACKED_1_END: i64 = 64;

pub const PACKED_2_START: u64 = 0xFFFFFFFFFFFFF000;
pub const PACKED_2_END: u64 = 0x0000000000001000;
pub const PACKED_3_START: u64 = 0xFFFFFFFFFFF80000;
pub const PACKED_3_END: u64 = 0x0000000000080000;
pub const PACKED_4_START: u64 = 0xFFFFFFFFFE000000;
pub const PACKED_4_END: u64 = 0x0000000002000000;
pub const PACKED_5_START: u64 = 0xFFFFFFFE00000000;
pub const PACKED_5_END: u64 = 0x0000000200000000;
pub const PACKED_6_START: u64 = 0xFFFFFE0000000000;
pub const PACKED_6_END: u64 = 0x0000020000000000;
pub const PACKED_7_START: u64 = 0xFFFE000000000000;
pub const PACKED_7_END: u64 = 0x0002000000000000;

pub const PRIORITY_CACHE_PACKED_END: u8 = 32;
pub const STRUCT_CACHE_PACKED_END: i32 = 16;
pub const BYTES_PACKED_LENGTH_END: usize = 8;
pub const STRING_PACKED_LENGTH_END: usize = 8;
pub const LIST_PACKED_LENGTH_END: u8 = 8;

pub const BYTE_CHUNK_SIZE: usize = 65535;