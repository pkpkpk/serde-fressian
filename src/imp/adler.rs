/*!
Adler-32 checksum
This implementation is based off the example found at
http://en.wikipedia.org/wiki/Adler-32.
# Example
```rust
use compress::checksum::adler;
let mut state = adler::State32::new();
state.feed(b"abracadabra");
let checksum = state.result();
```
*/

const MOD_ADLER: u32 = 65521;

/// Adler state for 32 bits
pub struct State32 {
    a: u32,
    b: u32,
}

impl State32 {
    /// Create a new state
    pub fn new() -> State32 {
        State32 { a: 1, b: 0 }
    }

    /// Mutate the state for given data
    pub fn feed(&mut self, buf: &[u8]) {
        for byte in buf.iter() {
            self.a = (self.a + *byte as u32) % MOD_ADLER;
            self.b = (self.a + self.b) % MOD_ADLER;
        }
    }

    /// Get checksum
    pub fn result(&self) -> u32 {
        (self.b << 16) | self.a
    }

    /// Reset the state
    pub fn reset(&mut self) {
        self.a = 1;
        self.b = 0;
    }
}

///////////////////////////////////////////////////////////////////////
// mod adler;

// fn basic_checksum() -> u32 {
//     let mut state = adler::State32::new();
//     let a: [u8; 1] = [1];
//     let b: [u8; 1] = [2];
//     state.feed(&a);
//     state.feed(&b);
//     state.result()
// }
//
// fn checksum(item: &[u8]) -> u32 {
//     let mut state = adler::State32::new();
//     state.feed(item);
//     state.result()
// }
//
//
