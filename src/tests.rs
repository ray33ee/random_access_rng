#[cfg(test)]
mod xxh3_hashable_tests {
    use crate::XXH3;

    #[test]
    fn test_0_ne() {
        let u = 0u128;
        let hash = u.xxh3();
        assert_ne!(u, hash);
    }

    #[test]
    fn test_1_ne() {
        let u = 1u128;
        let hash = u.xxh3();
        assert_ne!(u, hash);
    }
}

#[cfg(test)]
mod new_rng_tests {
    use crate::RandomAccessRNG;
    use rand_core::{RngCore};

    #[test]
    fn test_char() {
        let u = 'π';
        let mut rarng = RandomAccessRNG::new(u);
        let value = rarng.next_u64();
        assert_eq!(value, 8284857334634725328);
    }

    #[test]
    fn test_u16() {
        let u = 0xffffu16;
        let mut rarng = RandomAccessRNG::new(u);
        let value = rarng.next_u64();
        assert_eq!(value, 9225551887033442823);
    }

    #[test]
    fn test_u32() {
        let u = 0xffffffffu32;
        let mut rarng = RandomAccessRNG::new(u);
        let value = rarng.next_u64();
        assert_eq!(value, 2450845663194698032);
    }

    #[test]
    fn test_u64() {
        let u = 0xffffffffffffffffu64;
        let mut rarng = RandomAccessRNG::new(u);
        let value = rarng.next_u64();
        assert_eq!(value, 252690945009319174);
    }

    #[test]
    fn test_u128() {
        let u = 0xffffffffffffffffffffffffffffffffu128;
        let mut rarng = RandomAccessRNG::new(u);
        let value = rarng.next_u64();
        assert_eq!(value, 10781180276554971882);
    }

    #[test]
    fn test_f32() {
        let u = 1.29387420376023857f32;
        let mut rarng = RandomAccessRNG::new(u);
        let value = rarng.next_u64();
        assert_eq!(value, 16941462650142886078);
    }

    #[test]
    fn test_f64() {
        let u = 1.29387420376023857f64;
        let mut rarng = RandomAccessRNG::new(u);
        let value = rarng.next_u64();
        assert_eq!(value, 10321688837969058469);
    }

    #[test]
    fn test_string() {
        let u = "hello world!";
        let mut rarng = RandomAccessRNG::new(u);
        let value = rarng.next_u64();
        assert_eq!(value, 2180803427816266899);
    }

    #[test]
    fn test_empty_string() {
        let u = "";
        let mut rarng = RandomAccessRNG::new(u);
        let value = rarng.next_u64();
        assert_eq!(value, 9094331554905357734);
    }

    #[test]
    fn test_bytes() {
        let u: [u8; 4] = [0xff, 0x01, 0xba, 0xe4];
        let mut rarng = RandomAccessRNG::new(u.as_slice());
        let value = rarng.next_u64();
        assert_eq!(value, 5612944908904660542);
    }

    #[test]
    fn test_arrays() {
        let u = ["0xff", "0x01", "0xba", "0xe4", "hello", "world"];
        let mut rarng = RandomAccessRNG::new(u.as_slice());
        let value = rarng.next_u64();
        assert_eq!(value, 18207671733807896077);
    }

    #[test]
    fn test_nested_arrays() {
        let u = [["hello", "world"], ["stuff", "things"]];
        let mut rarng = RandomAccessRNG::new(u.as_slice());
        let value = rarng.next_u64();
        assert_eq!(value, 13214984778475501371);
    }
}

#[cfg(test)]
mod sequential_tests {
    use crate::RandomAccessRNG;
    use rand_core::{RngCore};

    #[test]
    fn test_reproducibility() {
        let u = 10u64;

        let mut rarng = RandomAccessRNG::new(u);
        let mut rarng2 = RandomAccessRNG::new(u);

        for _ in 0..100 {
            assert_eq!(rarng.next_u64(), rarng2.next_u64());
        }

    }

    #[test]
    fn test_seek() {
        let u = 10u64;

        let mut rarng = RandomAccessRNG::new(u);
        let mut rarng2 = RandomAccessRNG::new(u);

        for i in 0..100u64 {
            assert_eq!(rarng.next_u64(), rarng2.seek_u64(i));
        }
    }

    #[test]
    fn test_seek_random_access() {
        let u = 10u64;

        let mut rarng = RandomAccessRNG::new(u);
        let mut rarng2 = RandomAccessRNG::new(u);

        for _ in 0..100u64 {
            rarng.next_u64();
        }

        assert_eq!(rarng.next_u64(), rarng2.seek_u64(100));
    }
}


#[cfg(test)]
mod orthogonality_tests {
    use rand_core::RngCore;
    use crate::RandomAccessRNG;

    #[test]
    /// Whatever order we access child/parent object in, the result is the same
    fn test_parent_child_orthogonality() {
        let master_seed = 123456u64;

        let mut parent1 = RandomAccessRNG::new(master_seed);

        let mut parent2 = RandomAccessRNG::new(master_seed);

        let mut child1 = parent1.get("child");
        let mut child2 = parent2.get("child");

        let p1_u64 = parent1.next_u64();
        let c1_u64 = child1.next_u64();

        let c2_u64 = child2.next_u64();
        let p2_u64 = parent2.next_u64();

        assert_eq!(p1_u64, p2_u64);
        assert_eq!(c1_u64, c2_u64);

    }

    #[test]
    /// Whatever order we access sibling object in, the result is the same
    fn test_sibling_orthogonality() {
        let master_seed = 123456u64;

        let parent = RandomAccessRNG::new(master_seed);

        let mut child1 = parent.get("child1");
        let mut child2 = parent.get("child2");

        let c1_u64_a = child1.next_u64();
        let c2_u64_a = child2.next_u64();

        let parent = RandomAccessRNG::new(master_seed);

        let mut child1 = parent.get("child1");
        let mut child2 = parent.get("child2");

        let c2_u64_b = child2.next_u64();
        let c1_u64_b = child1.next_u64();

        assert_eq!(c1_u64_a, c1_u64_b);
        assert_eq!(c2_u64_a, c2_u64_b);

    }

    #[test]
    /// Whatever order we seek, the result is the same
    fn test_seek_orthogonality() {
        let master_seed = 123456u64;

        let mut parent = RandomAccessRNG::new(master_seed);

        let s1_a = parent.seek_u64(100);
        let s2_a = parent.seek_u64(200);

        let mut parent = RandomAccessRNG::new(master_seed);

        let s2_b = parent.seek_u64(200);
        let s1_b = parent.seek_u64(100);

        assert_eq!(s1_a, s1_b);
        assert_eq!(s2_a, s2_b);

    }

    #[test]
    /// Seeking should make no difference to a child
    fn test_seek_child_orthogonality() {
        let master_seed = 123456u64;

        let mut parent = RandomAccessRNG::new(master_seed);

        let mut child1 = parent.get("child");

        parent.seek_u64(1000);

        let mut child2 = parent.get("child");

        assert_eq!(child1.next_u64(), child2.next_u64());

    }

    #[test]
    /// calling next_u64 should not affect child
    fn test_next64_child_orthogonality() {
        let master_seed = 123456u64;

        let mut parent = RandomAccessRNG::new(master_seed);

        let mut child1 = parent.get("child");

        parent.next_u64();

        let mut child2 = parent.get("child");

        assert_eq!(child1.next_u64(), child2.next_u64());

    }

    #[test]
    /// calling next_u32 should not affect child
    fn test_next32_child_orthogonality() {
        let master_seed = 123456u64;

        let mut parent = RandomAccessRNG::new(master_seed);

        let mut child1 = parent.get("child");

        parent.next_u32();

        let mut child2 = parent.get("child");

        assert_eq!(child1.next_u64(), child2.next_u64());

    }
}



#[cfg(test)]
mod reproducibility_tests {
    use crate::RandomAccessRNG;
    use rand_core::RngCore;

    #[test]
    fn test_sibling_reproducibility() {
        let master_seed = 123456u64;

        let parent = RandomAccessRNG::new(master_seed);

        let mut child1 = parent.get("child");
        let mut child2 = parent.get("child");

        for _ in 0..1000 {
            assert_eq!(child1.next_u64(), child2.next_u64());
        }
    }

    #[test]
    fn test_master_reproducibility() {
        let master_seed = 123456u64;

        let mut parent1 = RandomAccessRNG::new(master_seed);
        let mut parent2 = RandomAccessRNG::new(master_seed);

        for _ in 0..1000 {
            assert_eq!(parent1.next_u64(), parent2.next_u64());
        }
    }
}


#[cfg(test)]
mod distribution_tests {

}


#[cfg(test)]
mod path_tests {
    use crate::RandomAccessRNG;
    use rand_core::RngCore;

    #[test]
    fn test_path() {
        let master_seed = 123456u64;

        let parent = RandomAccessRNG::new(master_seed);

        let hello = parent.get("hello");

        let mut world = hello.get("world!");

        let mut hello_world = parent.path("hello/world!");

        for _ in 0..1000 {
            assert_eq!(world.next_u64(), hello_world.next_u64());
        }
    }

    #[test]
    fn test_root_path() {
        let master_seed = 123456u64;

        let parent = RandomAccessRNG::new(master_seed);

        let hello = parent.get("hello");

        let mut world = hello.get("world!");

        let mut hello_world = parent.path("/hello/world!");

        for _ in 0..1000 {
            assert_eq!(world.next_u64(), hello_world.next_u64());
        }
    }

    #[test]
    fn test_terminating_path() {
        let master_seed = 123456u64;

        let parent = RandomAccessRNG::new(master_seed);

        let hello = parent.get("hello");

        let mut world = hello.get("world!");

        let mut hello_world = parent.path("hello/world!/");

        for _ in 0..1000 {
            assert_eq!(world.next_u64(), hello_world.next_u64());
        }
    }

    #[test]
    fn test_extra_slashes_path() {
        let master_seed = 123456u64;

        let parent = RandomAccessRNG::new(master_seed);

        let hello = parent.get("hello");

        let mut world = hello.get("world!");

        let mut hello_world = parent.path("//hello//////world!///");

        for _ in 0..1000 {
            assert_eq!(world.next_u64(), hello_world.next_u64());
        }
    }

    #[test]
    fn test_root_and_terminating_path() {
        let master_seed = 123456u64;

        let parent = RandomAccessRNG::new(master_seed);

        let hello = parent.get("hello");

        let mut world = hello.get("world!");

        let mut hello_world = parent.path("/hello/world!/");

        for _ in 0..1000 {
            assert_eq!(world.next_u64(), hello_world.next_u64());
        }
    }

    #[test]
    fn test_piecewise_path() {
        let master_seed = 123456u64;

        let parent = RandomAccessRNG::new(master_seed);

        let first = parent.path("a/b");
        let mut second = first.path("c/d");

        let mut all = parent.path("a/b/c/d");

        for _ in 0..1000 {
            assert_eq!(second.next_u64(), all.next_u64());
        }
    }
}