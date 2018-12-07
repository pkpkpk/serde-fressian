
use std::collections::{HashMap};
use crate::value::{Value};

pub struct Cache {
    index: u8,
    store: HashMap<Value,u8>
}

impl Cache {
    pub fn new() -> Self {
        Cache {
            index: 0,
            store: HashMap::<Value,u8>::new()
        }
    }

    pub fn get(&self, val: &Value) -> Option<u8> {
        self.store.get(val).map(|code: &u8| *code )
    }

    // returns Some(index) if already interned, None if inserted
    pub fn put(&mut self, val: Value) -> Option<u8> {
        if self.store.contains_key(&val) {
            self.get(&val)
        } else {
            let index = self.index;
            match self.store.insert(val, index) {
                Some(_) => {
                    None //should never get here
                }
                None => {
                    self.index += 1;
                    None
                }
            }
        }
    }

    pub fn reset(&mut self) {
        self.index = 0;
        self.store.clear()
    }
}