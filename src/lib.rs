// Re-export the trait and related types
pub use xxh3_trait::{XXH3, Xxh3Word};

// Expose the random access RNG module
pub mod random_access_rng;
pub use random_access_rng::RandomAccessRNG;

// Re-export xxh3_trait module for convenience
pub mod xxh3_trait {
    pub use xxh3_trait::*;
}

mod tests;