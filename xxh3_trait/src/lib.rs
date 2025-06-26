use xxhash_rust::xxh3::{xxh3_128};

pub type Xxh3Word = u128;

fn hasher(bytes: &[u8]) -> Xxh3Word {
    xxh3_128(bytes)
}

pub trait XXH3 {
    fn xxh3(&self) -> Xxh3Word;
}

impl XXH3 for u16 {
    fn xxh3(&self) -> Xxh3Word {
        hasher(self.to_le_bytes().as_slice())
    }
}

impl XXH3 for u32 {
    fn xxh3(&self) -> Xxh3Word {
        hasher(self.to_le_bytes().as_slice())
    }
}

impl XXH3 for u64 {
    fn xxh3(&self) -> Xxh3Word {
        hasher(self.to_le_bytes().as_slice())
    }
}

impl XXH3 for u128 {
    fn xxh3(&self) -> Xxh3Word {
        hasher(self.to_le_bytes().as_slice())
    }
}

impl XXH3 for i16 {
    fn xxh3(&self) -> Xxh3Word {
        hasher(self.to_le_bytes().as_slice())
    }
}

impl XXH3 for i32 {
    fn xxh3(&self) -> Xxh3Word {
        hasher(self.to_le_bytes().as_slice())
    }
}

impl XXH3 for i64 {
    fn xxh3(&self) -> Xxh3Word {
        hasher(self.to_le_bytes().as_slice())
    }
}

impl XXH3 for i128 {
    fn xxh3(&self) -> Xxh3Word {
        hasher(self.to_le_bytes().as_slice())
    }
}

impl XXH3 for usize {
    fn xxh3(&self) -> Xxh3Word {
        hasher(self.to_le_bytes().as_slice())
    }
}

impl XXH3 for isize {
    fn xxh3(&self) -> Xxh3Word {
        hasher(self.to_le_bytes().as_slice())
    }
}

impl XXH3 for f32 {
    fn xxh3(&self) -> Xxh3Word {
        hasher(self.to_le_bytes().as_slice())
    }
}

impl XXH3 for f64 {
    fn xxh3(&self) -> Xxh3Word {
        hasher(self.to_le_bytes().as_slice())
    }
}

impl XXH3 for char {
    fn xxh3(&self) -> Xxh3Word {
        let mut buffer = [0u8; 4];           // UTF-8 for any char fits in 4 bytes
        let s: &str = self.encode_utf8(&mut buffer);
        s.xxh3()
    }
}

impl XXH3 for &[u8] {
    fn xxh3(&self) -> Xxh3Word {
        hasher(self)
    }
}

impl<H: XXH3> XXH3 for &H {
    fn xxh3(&self) -> Xxh3Word {
        (*self).xxh3()
    }
}

impl<H: XXH3> XXH3 for & mut H {
    fn xxh3(&self) -> Xxh3Word {
        let shared: &H = &**self; // coerce &mut [H] to &[H]
        shared.xxh3()
    }
}

impl<H: XXH3> XXH3 for &[H] {
    fn xxh3(&self) -> Xxh3Word {
        let mut h = 0;
        for i in self.iter() {
            h ^= i.xxh3()
        }
        h
    }
}

impl<H: XXH3> XXH3 for Vec<H> {
    fn xxh3(&self) -> Xxh3Word {
        self.as_slice().xxh3()
    }
}

impl<H: XXH3> XXH3 for Box<H> {
    fn xxh3(&self) -> Xxh3Word {
        self.as_ref().xxh3()
    }
}

impl<'a, H: XXH3 + Clone> XXH3 for std::borrow::Cow<'a, H> {
    fn xxh3(&self) -> Xxh3Word {
        self.as_ref().xxh3()
    }
}

impl<H: XXH3> XXH3 for & mut [H] {
    fn xxh3(&self) -> Xxh3Word {
        let shared: &[H] = &**self; // coerce &mut [H] to &[H]
        shared.xxh3()
    }
}

impl<const N: usize, H: XXH3> XXH3 for [H; N] {
    fn xxh3(&self) -> Xxh3Word {
        let slice: &[H] = &*self;
        slice.xxh3()
    }
}

impl XXH3 for &str {
    fn xxh3(&self) -> Xxh3Word {
        hasher(self.as_bytes())
    }
}

pub mod xxh3_256;

#[cfg(test)]
mod xxh3_tests;

