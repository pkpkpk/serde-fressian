# Serde-Fressian
Fressian is a self-describing binary serialization format developed for clojure.

## wasm⥪fressian⥭cljs

## About

## WIP

## Fressian types

| Type    | rust | cljs    | clj  | ser | de  | fress
|---------|------|---------|------|-----|-----|-------
| NULL    |  ()  | nil     | nil  | [X] | [X] | [X]
| TRUE/FALSE | bool | bool | bool | [X] | [X] | [X]
| INT     | i64  | Number* | Long | [X] | [X] | [X]
| FLOAT | f32 | Number | Float | [X] | [X] | [X]
| DOUBLE | f64 | Number | Double | [X] | [X] | [X]
| STRING | string | string | string | [X] | [X]* | [X]
| UTF8*   | str | string | string | [X] | [X]* | [X]
| LIST | Vec&lt;T&gt; | vec | vec | [X] | [X] | [X]
| MAP | map* | map | map | [X] | [X] | [X]
| SET | set* | set | set | [X] | [X] | [X]
| SYM | tuple_struct | sym | sym | [X] | [X] | [X]
| KEY | tuple_struct | kw | kw | [X] | [X] | [X]
| INST | [Datetime&lt;Utc&gt;][chrono] | #inst | #inst | [X] | [X] | [X]
| UUID | [Uuid][uuid] |#UUID |#UUID | [X] | [X] | [X]
| REGEX | [Regex][reg] | regex | regex | [X] | [X] | [X]
| URI | [Url][url] | goog.Uri | URL | [X] | [X] | [X]
| INT_ARRAY | Vec&lt;i32&gt; | Int32Array | int[] | [X] | [X] | [X]
| LONG_ARRAY | Vec&lt;i64&gt; | Array&lt;Number&gt;*^ | long[] | [X] | [X] | [X]
| FLOAT_ARRAY | Vec&lt;f32&gt; | Float32Array | float[] | [X] | [X] | [X]
| DOUBLE_ARRAY | Vec&lt;f64&gt; | Float64Array | double[] | [X] | [X] | [X]
| BOOLEAN_ARRAY |Vec&lt;bool&gt; | Array&lt;bool&gt;*^ | bool[] | [X] | [X] | [X]


### TODO
+ restore footer
+ wasm API
  - exported write_byte
  - exported reset() method
  - reset, caching, footer ergonomics
  - usage patterns
+ fix nonsensical error types
+ value wrapper for heterogeneous colls
+ records
+ Char
+ BIGINT
  - crate use num::BigInt;
+ BIGDEC
+ OBJECT_ARRAY
+ uncompressed typed arrays, ints
+ TaggedObjects , structs
+ deserializing chunked strings & bytes not implemented yet
+ caching
  - ser
    - put cache
    - hopmap
  - de
    - get cache
+ checksum
+ type profiles (no inst etc)
+ document integers safety
+ preventing information loss
+ raw utf8 flag
+ serde limitations
  - using newtype attr flags



## Fress TODO
+ `api/reset`
+ need `*write-raw-utf8*` binding from api namespace (make default? goog-define?)
+ byte-stream alloc w/ with_capacity
+ uncompressed ints, typed arrays
+ empty string test for both fressian strings and utf8
+ array-seq for all native typed arrays

## Serde for Clojurists
  + strongly typed
  + weakly typed
  + derive
  + using with attr on wrappers a la serde_bytes


### rust serde types --> clojure runtime values
| Serde           |                    | Serde JSON          | Fressian| cljs | clj
|-----------------|--------------------|---------------------|---------|---------------|--------------
| unit            |                    |                     |  NULL   | nil           | nil   
| bool            |                    |                     |  T/F    | bool          | bool  
| i8              |                    |                     |  INT    | number        | long  
| i16             |                    |                     |  INT    | number        | long  
| i32             |                    |                     |  INT    | number        | long  
| i64             |                    |                     |  INT    | **glong?**    | long  
| i128            |                    |                     |  BIGINT | **TODO**      | bigint
| u8              |                    |                     |  INT    | number        | long
| u16             |                    |                     |  INT    | number        | long
| u32             |                    |                     |  INT    | number        | long
| u64             |                    |                     |  INT    | **glong? **   | ulong(java 8+)?
| u128            |                    |                     |  BIGINT | **TODO**      | bigint
| f32             |                    |                     |  FLOAT  | number        | float
| f64             |                    |                     |  DOUBLE | number        | double
| char            |                    |                     |  "char" | **TODO**      | char
| string          |                    |                     |  STRING | string        | string
|      \\-->      |                    |                     |  UTF8   | string        | tag -> string
| [u8]            |                    |                     |  BYTES  | byte-array    | byte-array
| Option<value>   |                    |                     |  ?value | ?value        | ?value
| unit_struct     |struct Z;           |`null`               |  NULL   | nil           |
| unit_variant    |E::Z                |`"Z"`                |  KEY    | keyword       |
| newtype_struct  |Y(i32)              |`0`                  |         |               |
| newtype_variant |E::Y(0)             |`{"Y":0}`            |         |               |
| tuple           |                    |                     |  list   | list          |
| tuple_struct    |`X(i32, i32)`       |`[0,0]`              |  rec    | record? list? |
| tuple_variant   |`E::X(0, 0)`        |`{"X":[0,0]}`        |  rec    | record? list? |
| struct          |`W { a: 0, b: 0}`   |`{"a":0,"b":0}`      |  STRUCT | struct?       |
| struct_variant  |`E::W { a: 0, b: 0}`|`{"W":{"a":0,"b":0}}`|         | struct?       |
| seq             |                    |                     |  LIST.. | list; open    |
| map             |                    |                     |         | map; open     |


+ tuple
  - A statically sized heterogeneous sequence of values for which the length will be known at deserialization time without looking at the serialized data, for example
  - `(String, u64, Vec<T>)`
  - `[u64; 10]`
+ map
  - A variably sized heterogeneous key-value pairing, for example `BTreeMap<K, V>`. When serializing, the length may or may not be known before iterating through all the entries. When deserializing, the length is determined by looking at the serialized data.
+ struct
  - A statically sized heterogeneous key-value pairing in which the keys are compile-time constant strings and will be known at deserialization time without looking at the serialized data
  - `struct S { r: u8, g: u8, b: u8 }`
+ struct_variant
  - `E::S` in `enum E { S { r: u8, g: u8, b: u8 } }`



+ serde impls
  - https://github.com/serde-rs/serde/blob/4e54aaf7963c3580cc50b56842949b0ce6b3a997/serde/src/de/impls.rs
  - https://github.com/serde-rs/serde/blob/bd366f675e6e12a2b96287098581bb63f219a2aa/serde/src/ser/mod.rs#L76
  - BTreeMap\<K, V\>
  - BTreeSet\<T\>
  - BinaryHeap\<T\>
  - HashMap\<K, V, H\>
  - HashSet\<T, H\>
  - LinkedList\<T\>
  - VecDeque\<T\>
  - Vec\<T\>



```rust
//from https://docs.serde.rs/serde_bytes/
//need this for maps, sets

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


[chrono]: https://github.com/chronotope/chrono
[uuid]: https://github.com/uuid-rs/uuid
[reg]: https://github.com/rust-lang/regex
[url]: https://github.com/servo/rust-url
