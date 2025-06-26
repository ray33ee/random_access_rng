
#[cfg(test)]
mod new_rng_tests {
    use crate::RandomAccessRNG;
    use rand_core::{RngCore};

    #[test]
    fn test_char() {
        let u = 'π';
        let mut rarng = RandomAccessRNG::new(u);
        let value = rarng.next_u64();
        assert_eq!(value, 5080887248986460467);
    }

    #[test]
    fn test_u16() {
        let u = 0xffffu16;
        let mut rarng = RandomAccessRNG::new(u);
        let value = rarng.next_u64();
        assert_eq!(value, 91599049468127248);
    }

    #[test]
    fn test_u32() {
        let u = 0xffffffffu32;
        let mut rarng = RandomAccessRNG::new(u);
        let value = rarng.next_u64();
        assert_eq!(value, 10214697059907569025);
    }

    #[test]
    fn test_u64() {
        let u = 0xffffffffffffffffu64;
        let mut rarng = RandomAccessRNG::new(u);
        let value = rarng.next_u64();
        assert_eq!(value, 821603837758500535);
    }

    #[test]
    fn test_u128() {
        let u = 0xffffffffffffffffffffffffffffffffu128;
        let mut rarng = RandomAccessRNG::new(u);
        let value = rarng.next_u64();
        assert_eq!(value, 11948003739489801998);
    }

    #[test]
    fn test_string() {
        let u = "hello world!";
        let mut rarng = RandomAccessRNG::new(u);
        let value = rarng.next_u64();
        assert_eq!(value, 3781495884637729362);
    }

    #[test]
    fn test_empty_string() {
        let u = "";
        let mut rarng = RandomAccessRNG::new(u);
        let value = rarng.next_u64();
        assert_eq!(value, 8018728376013651415);
    }

    #[test]
    fn test_bytes() {
        let u: [u8; 4] = [0xff, 0x01, 0xba, 0xe4];
        let mut rarng = RandomAccessRNG::new(u.as_slice());
        let value = rarng.next_u64();
        assert_eq!(value, 14149283188203591743);
    }

    #[test]
    fn test_arrays() {
        let u = ["0xff", "0x01", "0xba", "0xe4", "hello", "world"];
        let mut rarng = RandomAccessRNG::new(u.as_slice());
        let value = rarng.next_u64();
        assert_eq!(value, 15426435159302312466);
    }

    #[test]
    fn test_nested_arrays() {
        let u = [["hello", "world"], ["stuff", "things"]];
        let mut rarng = RandomAccessRNG::new(u.as_slice());
        let value = rarng.next_u64();
        assert_eq!(value, 9977935657722848223);
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


/// These tests relate more to the `Hash` macro than anything, but we include for completeness
#[cfg(test)]
mod difference_tests {
    use crate::RandomAccessRNG;
    use rand_core::RngCore;

    #[test]
    fn test_vector() {
        let master_seed = 123456u64;

        let v1 = vec![1, 2];
        let v2 = vec![2, 1];

        let parent = RandomAccessRNG::new(master_seed);

        let mut child1 = parent.get(v1);
        let mut child2 = parent.get(v2);

        for _ in 0..1000 {
            assert_ne!(child1.next_u64(), child2.next_u64());
        }

    }

    #[test]
    fn test_struct_ne() {

        #[derive(Hash)]
        struct Test(u64, u64);

        let master_seed = 123456u64;

        let v1 = Test(1, 2);
        let v2 = Test(2, 1);

        let parent = RandomAccessRNG::new(master_seed);

        let mut child1 = parent.get(v1);
        let mut child2 = parent.get(v2);

        for _ in 0..1000 {
            assert_ne!(child1.next_u64(), child2.next_u64());
        }

    }

    #[test]
    fn test_struct_eq() {

        #[derive(Hash)]
        struct Test1(u64, u64);
        #[derive(Hash)]
        struct Test2(u64, u64);

        let master_seed = 123456u64;

        let v1 = Test1(1, 2);
        let v2 = Test2(1, 2);

        let parent = RandomAccessRNG::new(master_seed);

        let mut child1 = parent.get(v1);
        let mut child2 = parent.get(v2);

        for _ in 0..1000 {
            assert_eq!(child1.next_u64(), child2.next_u64());
        }

    }


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

    #[test]
    fn test_triple_path() {
        let parent = RandomAccessRNG::new("root");

        // Unix-style paths
        let mut child1 = parent.path("world/enemy/color");

        // Windows-style paths (forward slashes)
        let mut child2 = parent.path("world\\enemy\\color");

        // Paths with root directory are handled
        let mut child3 = parent.path("/world/enemy/color"); // Same as "world/enemy"

        let c2_64 = child2.next_u64();

        assert_eq!(child1.next_u64(), c2_64);
        assert_eq!(c2_64, child3.next_u64());
    }
}