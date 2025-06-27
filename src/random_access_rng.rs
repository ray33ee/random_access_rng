use std::hash::Hash;
use rand_core::{RngCore, impls, SeedableRng};
use std::path::{Component, Path};
use xxhash_rust::xxh3::{Xxh3, xxh3_128};

fn xxh3_integer_hash(n: u128) -> u128 {
    xxh3_128(n.to_le_bytes().as_slice())
}

/// A deterministic random number generator that supports random access and hierarchical seeding.
/// 
/// This RNG uses fast XXH3 hashing to generate deterministic random numbers from any seed that
/// implements the [`Hash`] trait. It supports creating child RNGs derived from parent seeds.
/// 
/// # Key Features
/// 
/// - **Deterministic**: Same seed always produces the same sequence
/// - **Random Access**: Can seek to any position without generating intermediate values
/// - **Hierarchical**: Can create child RNGs with different seeds
/// - **Path-based**: Supports creating RNGs from file system-like paths
/// - **Standard Compatible**: Implements [`RngCore`] and [`SeedableRng`] traits
/// - **Portable**: Does not rely on implementation or OS
/// - **Reproducibility**: Seeding will never change between versions
/// - **Seek**: Efficiently seek to the nth random number in a sequence instead of having to calculate all n values.
///
/// # Examples
/// 
/// ## Standard RNG
/// ```rust
/// use random_access_rng::RandomAccessRNG;
/// use rand_core::RngCore;
/// 
/// let mut rng = RandomAccessRNG::new("my_seed");
/// let value = rng.next_u64();
/// ```
/// 
/// ## Creating Child RNGs
/// ```rust
/// use random_access_rng::RandomAccessRNG;
/// use rand_core::RngCore;
/// 
/// let parent = RandomAccessRNG::new("parent_seed");
/// let child = parent.get("child_key");
/// 
/// // Child RNGs are deterministic and independent
/// let mut child1 = child.clone();
/// let mut child2 = child.clone();
/// assert_eq!(child1.next_u64(), child2.next_u64());
/// ```
/// 
/// ## Random Access
/// ```rust
/// use random_access_rng::RandomAccessRNG;
/// use rand_core::RngCore;
/// 
/// let mut rng = RandomAccessRNG::new("seed");
/// 
/// // Generate 100 numbers
/// for _ in 0..100 {
///     rng.next_u64();
/// }
///
/// // Jump directly to position 1000
/// let value_at_1000 = rng.seek_u64(1000);
///
/// // And back to position 1
/// let value_at_1 = rng.seek_u64(1);
/// ```
/// 
/// ## Path-based RNGs
/// ```rust
/// use random_access_rng::RandomAccessRNG;
/// use rand_core::RngCore;
/// 
/// let parent = RandomAccessRNG::new("root");
/// let mut child1 = parent.path("level1/level2/level3");
/// 
/// // Equivalent to:
/// let mut child2 = parent.get("level1").get("level2").get("level3");
///
/// assert_eq!(child1.next_u64(), child2.next_u64());
/// ```
/// 
/// **Note**: This RNG is NOT cryptographically secure. Use a cryptographically secure
/// RNG (marked with the [`CryptoRng`](rand_core::CryptoRng) trait) for security-sensitive applications.
#[derive(Clone)]
pub struct RandomAccessRNG {
    hasher: Xxh3,
    index: u64,
}

impl RandomAccessRNG {

    /// Helper function to generate new RandomAccessRNGs (new or get)
    fn new_helper<H: Hash>(mut xxh3: Xxh3, seed: H) -> Self {
        seed.hash(& mut xxh3);

        Self {
            hasher: xxh3,
            index: 0,
        }
    }

    /// Generate a new [`RandomAccessRNG`] from a seed.
    /// 
    /// The seed can be any type that implements the [`Hash`] trait.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use random_access_rng::RandomAccessRNG;
    /// use rand_core::RngCore;
    ///
    /// #[derive(Hash)]
    /// struct Test(usize);
    /// 
    /// // Using different seed types
    /// let rng1 = RandomAccessRNG::new(42u64);
    /// let rng2 = RandomAccessRNG::new("hello world");
    /// let rng3 = RandomAccessRNG::new(vec![1, 2, 3, 4]);
    /// let rng4 = RandomAccessRNG::new(Test(400));
    /// ```
    /// 
    /// # Deterministic Behavior
    /// 
    /// The same seed will always produce the same sequence of random numbers:
    /// 
    /// ```rust
    /// use random_access_rng::RandomAccessRNG;
    /// use rand_core::RngCore;
    /// 
    /// let mut rng1 = RandomAccessRNG::new("seed");
    /// let mut rng2 = RandomAccessRNG::new("seed");
    /// 
    /// assert_eq!(rng1.next_u64(), rng2.next_u64());
    /// assert_eq!(rng1.next_u64(), rng2.next_u64());
    /// ```
    ///
    /// # Exactly reproducible
    ///
    /// We also guarantee the actual values produced by the RNG will always be the same:
    ///
    /// ```rust
    /// use random_access_rng::RandomAccessRNG;
    /// use rand_core::RngCore;
    ///
    /// let mut rng = RandomAccessRNG::new("seed");
    ///
    /// assert_eq!(rng.next_u64(), 10527926426583982810);
    /// ```
    ///
    pub fn new<H: Hash>(seed: H) -> Self {
        Self::new_helper(Xxh3::new(), seed)
    }

    /// Create a child RNG with a new seed derived from this RNG's state and the provided key.
    /// 
    /// The child RNG is deterministic and independent of the parent's current state.
    /// Multiple calls with the same key will produce the same child RNG.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use random_access_rng::RandomAccessRNG;
    /// use rand_core::RngCore;
    /// 
    /// let parent = RandomAccessRNG::new("parent_seed");
    /// 
    /// // Create child RNGs
    /// let child1 = parent.get("child_key");
    /// let child2 = parent.get("child_key");
    /// 
    /// // Child RNGs are identical
    /// let mut c1 = child1.clone();
    /// let mut c2 = child2.clone();
    /// assert_eq!(c1.next_u64(), c2.next_u64());
    /// 
    /// // Different keys produce different child RNGs
    /// let child3 = parent.get("different_key");
    /// let mut c3 = child3.clone();
    /// assert_ne!(c1.next_u64(), c3.next_u64());
    /// ```
    /// 
    /// # Use Cases
    /// 
    /// - **Procedural Generation**: Use different keys for different aspects (terrain, enemies, items)
    /// - **Testing**: Create independent RNGs for different test scenarios
    /// - **Simulation**: Separate RNGs for different simulation components
    pub fn get<H: Hash>(&self, key: H) -> Self {
        Self::new_helper(self.hasher.clone(), key)
    }

    /// Create a descendant RNG by applying multiple keys in sequence.
    /// 
    /// This is equivalent to calling [`get`](RandomAccessRNG::get) multiple times in sequence.
    /// The keys are applied in the order they appear in the iterator.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use random_access_rng::RandomAccessRNG;
    /// use rand_core::RngCore;
    /// 
    /// let parent = RandomAccessRNG::new("root");
    /// 
    /// // Using a vector of keys
    /// let keys = vec!["level1", "level2", "level3"];
    /// let mut descendant1 = parent.descendant(keys.iter());
    /// 
    /// // Equivalent to:
    /// let mut descendant2 = parent.get("level1").get("level2").get("level3");
    ///
    /// assert_eq!(descendant1.next_u64(), descendant2.next_u64());
    /// ```
    ///
    pub fn descendant<'a, H: Hash + 'a + ?Sized, I: IntoIterator<Item = & 'a H>>(&self, keys: I) -> Self {
        let mut h = self.hasher.clone();

        for key in keys {
            key.hash(&mut h);
        }

        Self {
            hasher: h,
            index: 0,
        }
    }

    /// Create a descendant RNG from a path.
    /// 
    /// Each path component is treated as a key for creating child RNGs.
    /// The path is parsed using standard path semantics, with some restrictions.
    /// 
    /// # Path Semantics
    /// 
    /// - **Normal components**: Used as keys (e.g., "folder", "file.txt")
    /// - **Root directory (`/`)**: Ignored
    /// - **Current directory (`.`)**: Not supported, will panic
    /// - **Parent directory (`..`)**: Not supported, will panic
    /// - **Windows prefixes**: Not supported, will panic
    /// 
    /// # Examples
    ///
    /// The following child objects are equivalent
    ///
    /// ```rust
    /// use random_access_rng::RandomAccessRNG;
    /// use rand_core::RngCore;
    /// 
    /// let parent = RandomAccessRNG::new("root");
    /// 
    /// // Unix-style paths
    /// let mut child1 = parent.path("world/enemy/color");
    /// 
    /// // Windows-style paths (forward slashes)
    /// let mut child2 = parent.path("world\\enemy\\color");
    /// 
    /// // Paths with root directory are handled
    /// let mut child3 = parent.path("/world/enemy/color");
    /// ```
    /// 
    /// # Panics
    /// 
    /// This method will panic if the path contains:
    /// - Current directory components (`.`)
    /// - Parent directory components (`..`)
    /// - Windows path prefixes
    /// 
    /// # Use Cases
    /// 
    /// - **File-based procedural generation**: Use paths as RNG seeds
    /// - **Organized randomness**: Group related random generation by path structure
    pub fn path<P: AsRef<Path>>(&self, path: P) -> Self {
        self.descendant(path
            .as_ref()
            .components()
            .filter_map(|component| match component {
                Component::Normal(c) => Some(c.to_str().expect("Invalid UTF-8 in component")),
                Component::RootDir => None,
                Component::Prefix(p) => panic!("Invalid windows path prefix - {:?}", p),
                Component::CurDir | Component::ParentDir => panic!("Absolute paths not supported"),
            }))
    }

    /// Internal helper used in `seek_u64` and `next_u64`
    fn next(& mut self) -> u128 {
        //Simple way to generate next random number by combining self.seed and self.index
        let result = xxh3_integer_hash(self.hasher.digest128() ^ self.index as u128);

        self.index += 1;

        result
    }

    /// Seek to a specific position in the random number sequence.
    /// 
    /// This method allows you to jump directly to any position in the sequence
    /// without having to repeatedly call [`next_u64`](RandomAccessRNG::next_u64).
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use random_access_rng::RandomAccessRNG;
    /// use rand_core::RngCore;
    /// 
    /// let mut rng = RandomAccessRNG::new("seed");
    /// 
    /// // Generate some numbers
    /// let first = rng.next_u64();
    /// let second = rng.next_u64();
    /// 
    /// // Jump to position 1000
    /// let at_1000 = rng.seek_u64(1000);
    /// 
    /// // Jump back to position 0 and verify that the second value matches
    /// rng.seek_u64(0);
    /// let at_2 = rng.next_u64();
    /// assert_eq!(at_2, second);
    /// ```
    /// 
    /// # Performance
    /// 
    /// Seeking is O(1) and doesn't require generating intermediate values,
    /// making it much faster than sequential generation for large jumps.
    /// 
    /// # Use Cases
    /// 
    /// - **Parallel generation**: Different threads can generate different parts of the sequence
    /// - **Caching**: Generate random numbers on-demand without storing the entire sequence
    /// - **Resumable generation**: Save the current position and resume later
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

impl SeedableRng for RandomAccessRNG {

    type Seed = [u8; 8]; //Low entropy for non-crypto RNGs

    fn from_seed(seed: Self::Seed) -> Self {
        Self::new(seed)
    }

}
