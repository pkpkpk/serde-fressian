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

    let env_args: Vec<String> = env::args().collect();
    let args: Vec<Value> = serde_json::from_str(&env_args[1]).unwrap();

    #[derive(Serialize, Deserialize)]
    struct TaggedObject {
        tag: String,
        value: serde_json::Value,
    }

    let buf = Vec::with_capacity(10);
    let mut fw = Serializer::new(buf);

    for obj in args {
        match obj {
            Value::Null => {
                fw.write_null().unwrap()
            },

            Value::Bool(b) => {
                 fw.write_boolean(b).unwrap()
            },

            Value::Number(n) => {
                let f = n.as_f64().unwrap();
                if f == f.round() {
                    fw.write_int(f as i64).unwrap();
                } else {
                    fw.write_double(f).unwrap();
                }
            },

            Value::Object(_) => {
                let o: TaggedObject = serde_json::from_value(obj).unwrap();
                match o.tag.as_ref() {
                    "bytes" => {
                        let bytes: Vec<u8> = serde_json::from_value(o.value).unwrap();
                        fw.write_bytes(&bytes, 0, bytes.len()).unwrap();
                    }
                    "utf8" => {
                        // let s: String = serde_json::from_value(o.value).unwrap();
                        // fw.write_raw_utf8(&s).unwrap();
                        ()
                    }
                    "vec" => {
                        let v: Vec<serde_json::Value> = serde_json::from_value(o.value).unwrap();
                        println!("vec: {:?}", &v);
                        v.serialize(&mut fw).unwrap()
                    }
                    _ => {
                        println!("unmatched TaggedObject {}", o.tag);
                        ()
                    }
                }
            },

            Value::String(s) => {
                // fw.write_string(&s).unwrap();
                s.serialize(&mut fw).unwrap()
            }

            _ => {
                ()
            },

        }
    };

    fw.write_footer().unwrap();

    let mut output_map = HashMap::new();
    output_map.insert("bytesWritten", json!(fw.get_bytes_written()));
    output_map.insert("bytes", json!(fw.get_ref()));

    let json = serde_json::to_string(&output_map).expect("Failed to convert HashMap into JSON");

    println!("{}", json);

}

