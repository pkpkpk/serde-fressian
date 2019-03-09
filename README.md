# `serde-fressian = "0.1.1"` [![Latest Version]][crates.io]
Fressian is a self-describing binary serialization format developed for clojure.

[Latest Version]: https://img.shields.io/crates/v/serde-fressian.svg
[crates.io]: https://crates.io/crates/serde-fressian

<hr>

## UPDATE 2019-03-09

This project is hammocked.

In its current state, this is an incomplete implementation of fressian. In addition to a couple missing types, part of the fressian protocol is *caching*, where you can hand off some data to serializer with a flag, and every subsequent time the serializer sees an *"equal"* object, it is written as a few bytes at most. As you can imagine for wasm consumers, this is a highly attractive feature, especially if you can agree on a cache table ahead of time and never pay the full cost of marshalling expensive data. This is the most important new feature moving forward, and will require some work.

#### Wishlist

+ simple interface for caching
+ custom user caches via traits
  - choose your hash algorithm, hard code behavior, etc
+ zero-copy, no-move cache-testing
  - this may be impossible with heterogeneous types
+ enable `&'static mut` caches
  - should be able to ship a lazy-static global cache & recycle its use
  - big perf win for wasm
+ `&mut` byte buffers
  - in single thread ctx, no reason to reallocate a buffer for every serializer instance
  - multi-threaded can pool
+ automatic caching via attribute
  - serialize impls are pretty rigid. Possibly can wrap them with a Cache trait and ship default impls
+ fill out type support

#### Problems

1. equivalence between objects

This is easy in JVM & JS because everything is an object, we can use runtime reflection, and we can rely on clojure to do the work for us. For rust though, objects of different types will cannot be compared, and serde will not let us further constrain types with traits (Hash etc) for use in Serializers.

2. serde wants to be stateless

Serde's `Serialize` want to treat every object the same, and you cannot introduce new trait restrictions on serializable types. Short of thread-local global variables, you cannot write a `Serialize` impl that accesses state somewhere to conditionally change serialization strategies.

#### Solutions

+ ~~adhoc reflection, serializing trait objects~~
  - tried & failed :-(
+ [serde-state](https://docs.rs/serde_state/0.3.0/serde_state/)?
+ enable caching only for types that can be converted to a common `Value` enum, but with the constraint of using a dedicated serialization api for the types you intend to cache
  - This was the original impl for caching. I am going to revert it back
+ completely wrap serde with a set of serialization traits designed with caching in mind
  - this is probably the most robust way forward, but will break from serde idioms

#### Feedback

Am I overlooking something?

<hr>

## wasm⥪fressian⥭cljs

When compiled for WebAssembly, serde-fressian can be used to convey rich values to and from clojurescript.

#### serde-fressian has 2 companion projects:
  + [fress](https://github.com/pkpkpk/fress)
    - an implementation of Fressian for clojurescript designed to interface with serde-fressian's wasm api from javascript.
  + [cargo-cljs](https://github.com/pkpkpk/cargo-cljs)
    - a clojurescript library for scripting cargo via nodejs

#### A WIP
  + No records, BIGINT, BIGDEC, OBJECT_ARRAY, char
  + No caching except for the types that require it
  + No checksum/validation
  + serde::fressian::value needs own Deserializer/Serializer impls, indexing, identity predicates
  + plenty of wasm specific optimizations yet to implement

#### Usage

serde-fressian tries to follow the standard serde conventions. Deserializers can accept readers, vecs, and slices. Serializers at this time however only support writing to vecs.

```rust
use serde_fressian::ser;
use serde_fressian::de;

let write_data: Vec<String> = vec!["some".to_string(), "strings".to_string()];

let bytes: Vec<u8> = ser::to_vec(&data).unwrap();

// this is strongly typed deserialization
let read_data: Vec<String> = de::from_vec(&bytes).unwrap();

assert_eq!(write_data,read_data)
```

If you know ahead of time what the bytes are going to contain, you can use strongly typed deserialization to extract your values. This is less flexible but is *very* fast. If you are unsure of the content, `serde_fressian::value::Value` is an enum encompassing all fressian types and will deserialize values as they are described.

```rust
use serde_fressian::ser;
use serde_fressian::de;
use serde_fressian::value::{Value};

let write_data: Vec<String> = vec!["some".to_string(), "strings".to_string()];

let bytes: Vec<u8> = ser::to_vec(&data).unwrap();

// this is weakly typed deserialization
let read_data: Value = de::from_vec(&bytes).unwrap();

// Value::LIST(vec![Value::STRING("some".to_string()),Value::STRING("strings".to_string())])
assert_eq!(read_data, Value::from(write_data))

```

#### Wasm API

The `serde_fressian::wasm` module is designed to interop with [fress.wasm](https://github.com/pkpkpk/fress/blob/master/src/main/cljs/fress/wasm.cljs)

```rust
use serde_fressian::error::{Error as FressError};
use serde_fressian::value::{self, Value};
use serde_fressian::wasm::{self};


// called by javascript with ptr to written bytes
#[no_mangle]
pub extern "C" fn echo(ptr: *mut u8, len: usize) -> *mut u8
{
    // read a value from javascript
    let val: Result<Value, FressError> = wasm::from_ptr(ptr, len);

    // from_ptr borrows, Value copies. So must own and free bytes separately
    wasm::fress_dealloc(ptr, len);

    //serializes the result, hands ownership of resulting bytes over to js
    wasm::to_js(val)
}
```

#### Docs

For my own sanity, most documentation will be over at [fress](https://github.com/pkpkpk/fress)

#### About Fressian
[Fressian](https://github.com/Datomic/fressian) was designed by [Rich Hickey](https://twitter.com/richhickey). There is a clojure wrapper [here](https://github.com/clojure/data.fressian). There is a design talk [here](https://www.youtube.com/watch?v=JArZqMqsaB0).


