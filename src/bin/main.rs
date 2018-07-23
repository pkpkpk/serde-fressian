#![allow(dead_code)]
#![allow(unused_imports)]
#![feature(raw_identifiers)]

#[macro_use]
extern crate serde_json;
extern crate serde;

#[macro_use]
extern crate serde_derive;

extern crate serde_fressian;

use serde_json::{Value, Error};
use std::io;
use std::env;
use std::collections::HashMap;
use serde::Serialize;

use serde_fressian::ser::{to_vec, Serializer};

fn main() {
    // let js_MAX_SAFE_INTEGER: f64 = 9007199254740991.0;

    let env_args: Vec<String> = env::args().collect();
    let args: Vec<Value> = serde_json::from_str(&env_args[1]).unwrap();

    #[derive(Serialize, Deserialize)]
    struct TaggedObject {
        tag: String,
        value: serde_json::Value,
    }

    let buf = Vec::with_capacity(10);
    let mut serializer = Serializer::new(buf);
    // value.serialize(&mut serializer)?;

    for obj in args {
        match obj {
            Value::Null => {
                serializer.write_null().unwrap()
            },

            Value::Bool(b) => {
                 serializer.write_boolean(b).unwrap()
            },

            Value::Number(n) => {
                let f = n.as_f64().unwrap();
                if f == f.round() {
                    serializer.write_int(f as i64).unwrap();
                } else {
                    serializer.write_double(f).unwrap();
                }
            },

            Value::Object(_) => {
                let o: TaggedObject = serde_json::from_value(obj).unwrap();
                match o.tag.as_ref() {
                    "bytes" => {
                        let bytes: Vec<u8> = serde_json::from_value(o.value).unwrap();
                        serializer.write_bytes(&bytes, 0, bytes.len()).unwrap();
                    }
                    "utf8" => {
                        let s: String = serde_json::from_value(o.value).unwrap();
                        // FW.write_raw_utf8(&s).unwrap();
                    }
                    "vec" => {
                        let v: Vec<serde_json::Value> = serde_json::from_value(o.value).unwrap();
                        // FW.write_object(&v).unwrap();
                        println!("vec: {:?}", &v);
                        v.serialize(&mut serializer).unwrap()
                    }
                    _ => {
                        println!("unmatched TaggedObject {}", o.tag);
                        ()
                    }
                }
            },

            Value::String(s) => {
                // serializer.write_string(&s).unwrap();
                s.serialize(&mut serializer).unwrap()
            }

            _ => {
                ()
            },

        }
    };

    serializer.write_footer();

    let mut output_map = HashMap::new();
    // output_map.insert("bytesWritten", json!(FW.get_bytes_written()));
    output_map.insert("bytesWritten", json!(serializer.get_bytes_written()));
    // output_map.insert("bytes", json!(FW.get_ref()));
    output_map.insert("bytes", json!(serializer.get_ref()));

    let json = serde_json::to_string(&output_map).expect("Failed to convert HashMap into JSON");

    println!("{}", json);

}

