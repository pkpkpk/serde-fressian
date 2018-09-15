
pub const PRIORITY_CACHE_PACKED_START: u8 = 0x80; //128
pub const PRIORITY_CACHE_PACKED_END: u8 = 0xA0; // 160
pub const STRUCT_CACHE_PACKED_START: u8 = 0xA0;
pub const STRUCT_CACHE_PACKED_END: u8 = 0xB0; //176
pub const LONG_ARRAY: u8 = 0xB0;
pub const DOUBLE_ARRAY: u8 = 0xB1;
pub const BOOLEAN_ARRAY: u8 = 0xB2;
pub const INT_ARRAY: u8 = 0xB3;
pub const FLOAT_ARRAY: u8 = 0xB4;
pub const OBJECT_ARRAY: u8 = 0xB5; //181
pub const UTF8: u8 = 0xBF; // 191  <= unique to fress client //////////////////
pub const MAP: u8 = 0xC0; //192
pub const SET: u8 = 0xC1; //193
pub const UUID: u8 = 0xC3; //195
pub const REGEX: u8 = 0xC4; // 196
pub const URI: u8 = 0xC5; // 197
pub const BIGINT: u8 = 0xC6; //198
pub const BIGDEC: u8 = 0xC7; //199
pub const INST: u8 = 0xC8; //200
pub const SYM: u8 = 0xC9; //201
pub const KEY: u8 = 0xCA; // 202
pub const ERROR: u8 = 0xCB; // 203 <= extended
pub const GET_PRIORITY_CACHE: u8 = 0xCC; //204
pub const PUT_PRIORITY_CACHE: u8 = 0xCD; //205
pub const PRECACHE: u8 = 0xCE; //206
pub const FOOTER: u8 = 0xCF; //207
pub const FOOTER_MAGIC: i64 = 0xCFCFCFCF; //3486502863//////////////////////////////////
pub const BYTES_PACKED_LENGTH_START: u8 = 0xD0; //208
pub const BYTES_PACKED_LENGTH_END: u8 = 0xD8; // 216
pub const BYTES_CHUNK: u8 = 0xD8; //216
pub const BYTES: u8 = 0xD9; //217
pub const STRING_PACKED_LENGTH_START: u8 = 0xDA; //218
pub const STRING_PACKED_LENGTH_END: u8 = 0xE2; //226
pub const STRING_CHUNK: u8 = 0xE2; //226
pub const STRING: u8 = 0xE3; //227
pub const LIST_PACKED_LENGTH_START: u8 = 0xE4; //228
pub const LIST_PACKED_LENGTH_END: u8 = 0xEC; //236
pub const LIST: u8 = 0xEC; //236
pub const BEGIN_CLOSED_LIST: u8 = 0xED; //237
pub const BEGIN_OPEN_LIST: u8 = 0xEE; //238
pub const STRUCTTYPE: u8 = 0xEF; //239
pub const STRUCT: u8 = 0xF0; //240
pub const META: u8 = 0xF1; //241
pub const STR: u8 = 0xF2; //  <= extended, WASM write only
// 243
pub const ANY: u8 = 0xF4; //244
pub const TRUE: u8 = 0xF5; //245
pub const FALSE: u8 = 0xF6; //246
pub const NULL: u8 = 0xF7; //247
pub const INT: u8 = 0xF8; //248
pub const FLOAT: u8 = 0xF9; //249
pub const DOUBLE: u8 = 0xFA; //250
pub const DOUBLE_0: u8 = 0xFB; //251
pub const DOUBLE_1: u8 = 0xFC; //252
pub const END_COLLECTION: u8 = 0xFD; //253
pub const RESET_CACHES: u8 = 0xFE; //254
pub const INT_PACKED_1_START: u8 = 0xFF; //255
pub const INT_PACKED_1_END: u8 = 0x40; //64
pub const INT_PACKED_2_START: u8 = 0x40;
pub const INT_PACKED_2_ZERO: u8 = 0x50; //80
pub const INT_PACKED_2_END: u8 = 0x60;
pub const INT_PACKED_3_START: u8 = 0x60;
pub const INT_PACKED_3_ZERO: u8 = 0x68; //104
pub const INT_PACKED_3_END: u8 = 0x70; //112
pub const INT_PACKED_4_START: u8 = 0x70;
pub const INT_PACKED_4_ZERO: u8 = 0x72;
pub const INT_PACKED_4_END: u8 = 0x74;
pub const INT_PACKED_5_START: u8 = 0x74; //116
pub const INT_PACKED_5_ZERO: u8 = 0x76; //118
pub const INT_PACKED_5_END: u8 = 0x78; //120
pub const INT_PACKED_6_START: u8 = 0x78; //120
pub const INT_PACKED_6_ZERO: u8 = 0x7A; //122
pub const INT_PACKED_6_END: u8 = 0x7C; //124
pub const INT_PACKED_7_START: u8 = 0x7C;
pub const INT_PACKED_7_ZERO: u8 = 0x7E; //126
pub const INT_PACKED_7_END: u8 = 0x80;
