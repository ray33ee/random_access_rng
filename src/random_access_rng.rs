use rand_core::{RngCore, impls};
use xxh3_trait::{XXH3, Xxh3Word};
use std::path::{Component, Path};
use xxh3_trait::xxh3_256::Xxh3_256;

#[derive(Debug, Clone, Copy)]
pub struct RandomAccessRNG {
    seed: Xxh3_256,
    index: u64,
}

impl RandomAccessRNG {
    /// Generate a new `RandomAccessRNG` from a seed
    pub fn new<H: XXH3>(seed: H) -> Self {
        Self {
            seed: Xxh3_256::from_seed(seed),
            index: 0,
        }
    }

    /// Get a `RandomAccessRNG` child object with `self` as parent and `key` as the key
    pub fn get<H: XXH3>(&self, key: H) -> Self {
        Self {
            seed: self.seed.combine(key),
            index: 0,
        }
    }

    /// Internal helper used in `seek_u64` and `next_u64`
    fn next(& mut self) -> Xxh3Word {
        let result = self.seed.index(self.index);

        self.index += 1;

        result
    }

    /// Perform several `get` calls with a path, treating each path component as a `get` call
    pub fn path<P: AsRef<Path>>(&self, path: P) -> Self {

        let mut me = self.clone();

        for component in path.as_ref().components() {

            match component {
                Component::Prefix(p) => { panic!("Invalid windows path prefix - {:?}", p); }
                Component::RootDir => {}
                Component::CurDir => { panic!("Absolute paths not supported"); }
                Component::ParentDir => { panic!("Absolute paths not supported"); }
                Component::Normal(c) => {
                    me = me.get(c.to_str().unwrap());
                }
            }

        }
        me
    }

    /// Seek to the nth rng state instead of calling `next_u64` n times
    pub fn seek_u64(& mut self, index: u64) -> u64 {
        self.index = index;

        self.next() as u64
    }
}


impl RngCore for RandomAccessRNG {
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    fn next_u64(&mut self) -> u64 {
        self.next() as u64
    }

    fn fill_bytes(&mut self, b: &mut [u8]) {
        impls::fill_bytes_via_next(self, b)
    }

}
