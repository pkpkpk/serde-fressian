## Fressian Types

| Type       | rust | cljs    | clj  |
|------------|------|---------|------|
| NULL       |  ()/None  | nil     | nil  
| TRUE/FALSE | bool | bool | bool
| INT        | i64  | Number* | Long
| FLOAT      | f32  | Number | Float
| DOUBLE     | f64  | Number | Double
| BYTES      | serde_bytes::ByteBuf | Int8Array | byte[]
| STRING     | string       | string | string
| UTF8*      | string       | string | string
| LIST       | Vec&lt;T&gt; | vec    | vec
| MAP        | map*         | map    | map
| SET        | types::SET*  | set    | set
| SYM        | types::SYM   | sym | sym
| KEY        | types::KEY   | kw | kw
| INST       | types::INST  | #inst | #inst
| UUID       | types::UUID  | #UUID |#UUID
| REGEX      | types::REGEX | regex | regex
| URI        | types::URI   | goog.Uri | URL
| INT_ARRAY     | types::IntArray(Vec&lt;i32&gt;)      | Int32Array | int[]
| LONG_ARRAY    | types::LongArray(Vec&lt;i64&gt;)     | Array&lt;Number&gt;*^ | long[]
| FLOAT_ARRAY   | types::FloatArray(Vec&lt;f32&gt;)    | Float32Array          | float[]
| DOUBLE_ARRAY  | types::DoubleArray(Vec&lt;f64&gt;)   | Float64Array          | double[]
| BOOLEAN_ARRAY | types::BooleanArray(Vec&lt;bool&gt;) | Array&lt;bool&gt;*^   | bool[]







### Complications with Serde

All standard rust types have built-in Serialize/Deserialize impls which prevent fine grain control over serialization. For example, HashSets, BTreeSets, Slices, and Vec<T> are all written as generic sequences and the nuance of each container is lost. This is a known source of friction in serde, and in the future there will be a way to override Serialization impls. Until then, the workaround solution is to create a 'newtype' struct enclosing your target type, which allows you to use custom serialize impls. You can use wrapper types directly, or use serde attributes (see below).

The most common wrapper you'll probably need is the ByteBuf provided by the [serde_bytes][serde_bytes] crate. This let's you write Vec&lt;u8&gt; and `&[u8]` as BYTE arrays rather than generic lists.

```rust
use serde_bytes::{ByteBuf};
use serde_fressian::ser;

let bytes: Vec<u8> = vec![0,1,2];
let output = ser::to_vec(&bytes).unwrap(); //--> serialized as LIST

let bb = serde_bytes::ByteBuf::from(bytes);
let output = ser::to_vec(&bb).unwrap(); //--> serialized as BYTES

```
#### `serde_fressian::set`

This module provides a wrapper for BTreeSets. `serde_fressian` prefers BTreeSets over HashSets because the former implements Hash on the Set container itself. The same is true for BtreeMaps vs HashMaps. Hashing the container itself is required for implementing `serde_fressian::value` (see below) because it enables using the containers themselves as keys (..or set values), which may show up in that crazy clojure data.

```rust
#[macro_use]
extern crate maplit; // provides map & set literals
use std::collections::{BTreeSet};
use serde_fressian::ser;
use serde_fressian::set::{SET};

let set: BTreeSet<i64> = btreeset!{0,1,2,3};
let output = ser::to_vec(&set).unwrap(); //--> serialized as LIST

let wrapped_set = SET<i64> = SET::from(set);
let output = ser::to_vec(&wrapped_set).unwrap(); //--> serialized as SET

```

#### `serde_fressian::typed_arrays`

This module provides additional wrappers for fressian's typed arrays:

```rust
use serde_fressian::ser;
use serde_fressian::typed_arrays::{DoubleArray};

let v: Vec<f64> = vec![-2.0, -1.0, 0.0, 1.0, 2.0];
let output = ser::to_vec(&v).unwrap(); //--> serialized as LIST

let da: DoubleArray = DoubleArray::from_vec(v);
let output = ser::to_vec(&da).unwrap(); //--> serialized as DOUBLE_ARRAY
```





#### Other Newtypes
`serde_fressian::sym::{SYM}`
`serde_fressian::key::{KEY}`


`use serde_fressian::regex::{REGEX};`
`use serde_fressian::uri::{URI};`
`use serde_fressian::uuid::{UUID};`
`use serde_fressian::inst::{INST};`






#### Using Serde Attributes

You can use [serde attributes](https://serde.rs/attributes.html) to use custom serialize/deserialize impls produced by serde-derive. For example, this is from [serde_bytes][serde_bytes]

```rust
#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_bytes;

#[derive(Serialize)]
struct Efficient<'a> {
    #[serde(with = "serde_bytes")]
    bytes: &'a [u8],

    #[serde(with = "serde_bytes")]
    byte_buf: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
struct Packet {
    #[serde(with = "serde_bytes")]
    payload: Vec<u8>,
}
```




<!-- [Url][url] -->
<!-- [Regex][reg] -->
<!-- [Uuid][uuid] -->
<!-- [Datetime&lt;Utc&gt;][chrono] -->
[chrono]: https://github.com/chronotope/chrono
[uuid]: https://github.com/uuid-rs/uuid
[reg]: https://github.com/rust-lang/regex
[url]: https://github.com/servo/rust-url
[serde_bytes]: https://docs.serde.rs/serde_bytes

<hr>

## Value

<hr>

## Serde Types
| Serde           |      rust          | Fressian| cljs | clj | Serde JSON          
|-----------------|--------------------|---------------------|---------|---------------|--------------
| unit            |                    |  NULL   | nil           | nil   
| bool            |                    |  T/F    | bool          | bool  
| i8              |                    |  INT    | number        | long  
| i16             |                    |  INT    | number        | long  
| i32             |                    |  INT    | number        | long  
| i64             |                    |  INT    | **glong?**    | long  
| i128            |                    |  BIGINT | **TODO**      | bigint
| u8              |                    |  INT    | number        | long
| u16             |                    |  INT    | number        | long
| u32             |                    |  INT    | number        | long
| u64             |                    |  INT    | **glong? **   | ulong(java 8+)?
| u128            |                    |  BIGINT | **TODO**      | bigint
| f32             |                    |  FLOAT  | number        | float
| f64             |                    |  DOUBLE | number        | double
| char            |                    |  "char" | **TODO**      | char
| string          |                    |  STRING | string        | string
|      \\-->      |                    |  UTF8   | string        | tag -> string
| [u8]            |                    |  BYTES  | byte-array    | byte-array
| Option<value>   |                    |  ?value | ?value        | ?value
| unit_struct     |struct Z;           |  NULL   | nil           | |`null`               
| unit_variant    |E::Z                |  KEY    | keyword       | |`"Z"`                
| newtype_struct  |Y(i32)              |         |               | |`0`                  
| newtype_variant |E::Y(0)             |         |               | |`{"Y":0}`            
| tuple           |                    |  list   | list          | |
| tuple_struct    |`X(i32, i32)`       |  rec    | record? list? | |`[0,0]`
| tuple_variant   |`E::X(0, 0)`        |  rec    | record? list? | |`{"X":[0,0]}`
| struct          |`W { a: 0, b: 0}`   |  STRUCT | struct?       | |`{"a":0,"b":0}`
| struct_variant  |`E::W { a: 0, b: 0}`|         | struct?       | |`{"W":{"a":0,"b":0}}`
| seq             |                    |  LIST.. | list; open    | |
| map             |                    |         | map; open     | |