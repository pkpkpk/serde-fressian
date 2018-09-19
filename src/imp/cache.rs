
use std::collections::{HashMap};
use value::{Value};

pub struct Cache {
    index: u8,
    store: HashMap<Value,u8>
}

// pub enum Put {
//     New(u8),
//     Exists(u8)
// }

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

    pub fn put(&mut self, val: Value) -> Option<u8> { //bool?
        let index = self.index;
        if self.store.contains_key(&val) {
            None
        } else {
            match self.store.insert(val, index) {
                Some(_) => {
                    None //should never get here
                }
                None => {
                    self.index += 1;
                    Some(index)
                }
            }
        }
    }

    pub fn reset(&mut self) {
        self.index = 0;
        self.store.clear()
    }
}