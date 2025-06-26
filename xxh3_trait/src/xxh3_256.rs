use crate::XXH3;
use xxhash_rust::xxh3::xxh3_128_with_seed;

/// Number of u128s in the state (for 256 this is 2)
const SIZE: usize = 2;

#[derive(Debug, Clone, Copy)]
pub struct Xxh3_256 {
    state: [u128; SIZE],
}

impl Xxh3_256 {
    /// Create a new xxh3 hasher from a seed value
    pub fn from_seed<H: XXH3>(seed: H) -> Self {
        let hash = seed.xxh3();
        Self {
            state: [hash, hash.xxh3()],
        }
    }

    /// Internal function used to convert `state` into `&[u8]`
    fn state_as_u8(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(self.state.as_ptr() as *const u8, SIZE * 16)
        }
    }

    /// Used by `next` and `RngCore` for number generation
    pub fn index(& self, i: u64) -> u128 {
        xxh3_128_with_seed(self.state_as_u8(), i)
    }

    /// Used by `get` and `path` to generate new RNGs
    pub fn combine<H: XXH3>(& self, key: H) -> Self {

        let mut ret = self.clone();

        ret.state[0] = ret.state[0] ^ key.xxh3();

        ret
    }

}