# `serde-fressian = "0.1.0"`
Fressian is a self-describing binary serialization format developed for clojure.

## wasm⥪fressian⥭cljs

When compiled for WebAssembly, serde-fressian can be used to convey rich values to and from clojurescript. For more info see [serde-fressian-wasm-demo/README.md](serde-fressian-wasm-demo/README.md)

## About

## WIP

### TODO
+ ~~restore footer~~
+ value
  - rt all types
  - indexing
+ STR, write only for serde, read-only for fress
+ wasm API
  - lock/release
  - exported write_byte
  - exported reset() method
  - caching, footer ergonomics
  - usage patterns
+ fix nonsensical error types
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
+ identify lossy types
+ raw utf8 flag
+ serde limitations
  - using newtype attr flags
+ SET, MAP

+ basic caching copies. What about caching references? new fressian type? see rust's bincode



## Fress TODO
+ `api/reset`
+ need `*write-raw-utf8*` binding from api namespace (make default? goog-define?)
+ byte-stream alloc w/ with_capacity
+ uncompressed ints, typed arrays
+ empty string test for both fressian strings and utf8
+ array-seq for all native typed arrays
+ fress-js
+ document checksum changes

## Serde for Clojurists
  + strongly typed
  + weakly typed
  + serde types
  + derive
  + using with attr on wrappers a la serde_bytes


