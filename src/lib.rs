//! A deterministic random number generator with hierarchical seeding and random access capabilities.
//! 
//! This crate provides a fast, deterministic random number generator that supports:
//! 
//! - **Hierarchical seeding**: Create child RNGs with different seeds
//! - **Random access**: Jump to any position in the sequence without generating intermediate values
//! - **Path-based seeding**: Use file system-like paths to create RNG hierarchies
//! - **Standard compatibility**: Implements `RngCore` and `SeedableRng` traits
//! 
//! # Quick Start
//! 
//! ```rust
//! use random_access_rng::RandomAccessRNG;
//! use rand_core::RngCore;
//! 
//! // Create a new RNG with a seed
//! let mut rng = RandomAccessRNG::new("my_seed");
//! 
//! // Generate random numbers from parent
//! let value = rng.next_u64();
//! 
//! // Create child RNGs
//! let mut child = rng.get("child_key");
//!
//! // Generate random numbers from child
//! let value = child.next_u64();
//! 
//! // Use path-based seeding
//! let mut path_rng = rng.path("level1/level2/level3");
//!
//! // Generate random numbers from path
//! let value = path_rng.next_u64();
//! ```
//!
//! ## Deterministic Randomness
//! 
//! The same seed always produces the same sequence of random numbers, making it perfect for:
//! - Procedural generation
//! - Reproducible simulations
//! - Testing and debugging
//! 
//! ## Hierarchical Seeding
//! 
//! Create independent child RNGs that maintain the deterministic properties:
//! 
//! ```rust
//! use random_access_rng::RandomAccessRNG;
//! 
//! let parent = RandomAccessRNG::new("world_seed");
//! 
//! // Different aspects of your world can have their own RNGs
//! let terrain_rng = parent.get("terrain");
//! let enemy_rng = parent.get("enemies");
//! let item_rng = parent.get("items");
//! 
//! //
//! ```
//!
//! Each child RNG is independent and deterministic and modifying the internal state of one object does not affect the
//! internal state of any other parent, sibling or child objects.
//!
//! ## Random Access
//! 
//! Jump to any position in the sequence instantly:
//! 
//! ```rust
//! use random_access_rng::RandomAccessRNG;
//! use rand_core::RngCore;
//! 
//! let mut rng = RandomAccessRNG::new("seed");
//! 
//! // Jump directly to position 1000
//! let value_at_1000 = rng.seek_u64(1000);
//! 
//! // Jump to position 5000
//! let value_at_5000 = rng.seek_u64(5000);
//! ```
//! 
//! ## Path-Based Seeding
//! 
//! Use file system-like paths to create RNG hierarchies:
//! 
//! ```rust
//! use random_access_rng::RandomAccessRNG;
//! 
//! let world = RandomAccessRNG::new("world_seed");
//! 
//! // Create RNGs for specific locations
//! let forest_rng = world.path("biomes/forest");
//! let cave_rng = world.path("biomes/cave");
//! let village_rng = world.path("structures/village");
//! ```
//! 
//! # Performance
//! 
//! This RNG is designed for speed and uses the XXH3 hash function, which is:
//! - Extremely fast (often faster than memcpy)
//! - High quality for non-cryptographic purposes
//! - Well-distributed output
//! 
//! # Security Notice
//! 
//! **This RNG is NOT cryptographically secure.** It's designed for:
//! - Game development
//! - Procedural generation
//! - Simulation and testing
//! - Any application requiring deterministic randomness
//! 
//! For security-sensitive applications, use a cryptographically secure RNGs look specifically for RNGs that implement the `CryptoRng` trait.
//! 
//! # Examples
//! 
//! ## Procedural Terrain Generation
//! 
//! ```rust
//! use random_access_rng::RandomAccessRNG;
//! use rand_core::RngCore;
//! use rand::Rng;
//! 
//! fn generate_terrain(world_seed: &str, x: i32, y: i32) -> f64 {
//!     let world = RandomAccessRNG::new(world_seed);
//!     let mut terrain = world.path(&format!("terrain/{}/{}", x, y));
//!     
//!     // Generate height value
//!     let height = terrain.random::<f64>();
//!     height * 1000.0
//! }
//! ```
//! 
//! ## Parallel Generation
//! 
//! ```rust
//! use random_access_rng::RandomAccessRNG;
//! use rand_core::RngCore;
//! use std::thread;
//! 
//! fn generate_chunk_parallel(world_seed: &str, chunk_id: u64) -> Vec<u64> {
//!     let mut rng = RandomAccessRNG::new(world_seed);
//!     
//!     // Jump to the start of this chunk
//!     rng.seek_u64(chunk_id * 1000);
//!     
//!     // Generate 1000 random numbers for this chunk
//!     (0..1000).map(|_| rng.next_u64()).collect()
//! }
//! ```
//! 
//! ## Testing with Deterministic RNGs
//! 
//! ```rust
//! use random_access_rng::RandomAccessRNG;
//! use rand_core::RngCore;
//! 
//! #[test]
//! fn test_deterministic_behavior() {
//!     let mut rng1 = RandomAccessRNG::new("test_seed");
//!     let mut rng2 = RandomAccessRNG::new("test_seed");
//!     
//!     // Both RNGs should produce identical sequences
//!     for _ in 0..100 {
//!         assert_eq!(rng1.next_u64(), rng2.next_u64());
//!     }
//! }
//! ```

// Expose the random access RNG module
pub mod random_access_rng;
pub use random_access_rng::RandomAccessRNG;

#[cfg(test)]
mod tests;