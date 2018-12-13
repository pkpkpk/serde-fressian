use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use serde::ser::{Serialize};

pub trait ICache {
    fn intern<T: Serialize + Hash + PartialEq>(&mut self, object: &T) -> Option<usize>; //u32?
    fn reset(&mut self) -> ();
}

fn default_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new(); //this should be parameterized
    t.hash(&mut s);
    s.finish()
}


pub struct Cache {
    hashes: Vec<u64>
}

impl Cache {
    pub fn new() -> Self {
        Cache {
            hashes: Vec::new()
        }
    }

    #[inline]
    fn test_hash(&self, h: u64) -> Option<usize> {
        self.hashes.iter().position(|h_i| *h_i == h)
    }
}

impl ICache for Cache {
    fn reset(&mut self) {
        self.hashes.clear()
    }

    fn intern<T>(&mut self, object: &T) -> Option<usize>
        where T: Serialize + Hash + PartialEq,
    {
        let h = default_hash(object);

        let test: Option<usize> = self.test_hash(h);

        if test.is_some() {
            return test
        } else {
            self.hashes.push(h);
            return None
        }
    }
}

#[test]
fn cache_test(){

    let mut cache = Cache::new();

    let v0 = vec![1,2,3];
    let v1 = "foo";

    assert_eq!(None, cache.intern(&v0));
    assert_eq!(None, cache.intern(&v1));
    assert_eq!(Some(0), cache.intern(&v0));
    assert_eq!(Some(1), cache.intern(&v1));
    cache.reset();
    assert_eq!(None, cache.intern(&v0));
    assert_eq!(None, cache.intern(&v1));
}

