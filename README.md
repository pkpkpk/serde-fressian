# `serde-fressian = "0.1.0"`
Fressian is a self-describing binary serialization format developed for clojure.

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


#### About Fressian
[Fressian](https://github.com/Datomic/fressian) was designed by [Stuart Halloway](https://twitter.com/stuarthalloway) and the good people of Cognitect. There is a clojure wrapper [here](https://github.com/clojure/data.fressian). There is a design talk [here](https://www.youtube.com/watch?v=JArZqMqsaB0).


