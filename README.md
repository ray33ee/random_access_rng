# Random Access RNG

A fast, deterministic random number generator with hierarchical seeding and random access capabilities.

[![Crates.io](https://img.shields.io/crates/v/random_access_rng)](https://crates.io/crates/random_access_rng)
[![Documentation](https://docs.rs/random_access_rng/badge.svg)](https://docs.rs/random_access_rng)
[![Version](https://img.shields.io/badge/version-0.1.0-blue.svg)](https://github.com/ray33ee/random_access_rng)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Features

- **🚀 Fast**: Uses XXH3 hash function for high performance
- **🎯 Deterministic**: Same seed always produces the same sequence
- **🌳 Hierarchical**: Create child RNGs with different seeds
- **📍 Random Access**: Jump to any position without generating intermediate values
- **🛤️ Path-based**: Use paths to create RNG hierarchies
- **🔧 Standard Compatible**: Implements [RngCore](https://docs.rs/rand/latest/rand/trait.RngCore.html) and [SeedableRng](https://docs.rs/rand/latest/rand/trait.SeedableRng.html) traits

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
random_access_rng = "0.1.0"
```

Basic usage:

```rust
use random_access_rng::RandomAccessRNG;

// Create a new RNG with a seed
let mut rng = RandomAccessRNG::new("my_seed");

// Generate random numbers
let value = rng.next_u64();

// Create child RNGs
let child = rng.get("child_key");

// Use path-based seeding
let path_rng = rng.path("level1/level2/level3");
```

## Examples

### Procedural Terrain Generation

```rust
use random_access_rng::RandomAccessRNG;
use rand::Rng;

fn generate_terrain(world_seed: &str, x: i32, y: i32) -> f64 {
    let world = RandomAccessRNG::new(world_seed);
    let terrain = world.path(&format!("terrain/{}/{}", x, y));
    
    // Generate height value
    let height = terrain.random::<f64>();
    height * 1000.0 // Scale to 0-1000 range
}

// Generate terrain for different coordinates
let height1 = generate_terrain("world_seed", 10, 20);
let height2 = generate_terrain("world_seed", 10, 20); // Same result
let height3 = generate_terrain("world_seed", 15, 25); // Different result
```
### Game Development

```rust
use random_access_rng::RandomAccessRNG;

struct GameWorld {
    rng: RandomAccessRNG,
}

impl GameWorld {
    fn new(seed: &str) -> Self {
        Self {
            rng: RandomAccessRNG::new(seed),
        }
    }
    
    fn generate_enemy(&self, enemy_type: &str) -> Enemy {
        let enemy_rng = self.rng.get(enemy_type);
        Enemy::new(enemy_rng)
    }
    
    fn generate_item(&self, location: &str) -> Item {
        let item_rng = self.rng.path(&format!("items/{}", location));
        Item::new(item_rng)
    }
}

let world = GameWorld::new("game_seed");
let enemy = world.generate_enemy("goblin");
let item = world.generate_item("forest/chest");
```

## Key Concepts

### Deterministic Randomness

The same seed always produces the same sequence, making it perfect for:
- Procedural generation
- Reproducible simulations
- Testing and debugging

```rust
let mut rng1 = RandomAccessRNG::new("test_seed");
let mut rng2 = RandomAccessRNG::new("test_seed");

// Both RNGs produce identical sequences
assert_eq!(rng1.next_u64(), rng2.next_u64());
assert_eq!(rng1.next_u64(), rng2.next_u64());
```

### Hierarchical Seeding

Create independent child RNGs that maintain deterministic properties:

```rust
let parent = RandomAccessRNG::new("world_seed");

// Different aspects can have their own RNGs
let terrain_rng = parent.get("terrain");
let enemy_rng = parent.get("enemies");
let item_rng = parent.get("items");

// Each child RNG is independent and deterministic
```

### Random Access

Jump to any position in the sequence instantly:

```rust
let mut rng = RandomAccessRNG::new("seed");

// Jump directly to position 1000
let value_at_1000 = rng.seek_u64(1000);

// Jump to position 5000
let value_at_5000 = rng.seek_u64(5000);

// Jump to back position 0
let value_at_5000 = rng.seek_u64(0);
```

### Path-Based Seeding

Use file system-like paths to create RNG hierarchies:

```rust
let world = RandomAccessRNG::new("world_seed");

// Create RNGs for specific locations
let forest_rng = world.path("biomes/forest");
let cave_rng = world.path("biomes/cave");
let village_rng = world.path("structures/village");
```

## Performance

This RNG is designed for speed and uses the XXH3 hash function, which is:
- Extremely fast
- High quality for non-cryptographic purposes
- Well-distributed output

## Security Notice

**This RNG is NOT cryptographically secure.** It's designed for:
- Procedural generation
- Simulation and testing
- Game development
- Any application requiring deterministic randomness

For security-sensitive applications, use a cryptographically secure RNG marked by the [CryptoRng](https://docs.rs/rand_core/latest/rand_core/trait.CryptoRng.html) trait.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
random_access_rng = "0.1.0"
```

## Documentation

- [API Documentation](https://docs.rs/random_access_rng)

## License

This project is licensed under the MIT License - see the [LICENSE](license) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Changelog

### 0.1.0
- Initial release
- Basic RNG functionality
- Hierarchical seeding
- Random access capabilities
- Path-based seeding 