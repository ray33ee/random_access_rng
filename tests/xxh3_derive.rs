use xxh3_derive::XXH3;
use xxh3_trait::XXH3;
use random_access_rng::RandomAccessRNG;
use rand_core::RngCore;

#[derive(XXH3)]
struct SimpleNamedStruct {
    a: u64,
}

#[derive(XXH3)]
struct SimpleUnnamedStruct(u64);

#[derive(XXH3)]
enum SimpleUnitEnum {
    First,
}

#[test]
fn test_simple_named_struct() {
    let master_seed = 123456u64;
    let parent = RandomAccessRNG::new(master_seed);
    let mut child = parent.get(SimpleNamedStruct{ a: 99});
    assert_eq!(child.next_u64(), 6992678887897862208);
}

#[test]
fn test_simple_unnamed_struct() {
    let master_seed = 123456u64;
    let parent = RandomAccessRNG::new(master_seed);
    let mut child = parent.get(SimpleUnnamedStruct(99));
    assert_eq!(child.next_u64(), 6992678887897862208);
}

#[test]
fn test_simple_unit_enum() {
    let master_seed = 123456u64;
    let parent = RandomAccessRNG::new(master_seed);
    let mut child = parent.get(SimpleUnitEnum::First);
    assert_eq!(child.next_u64(), 8307879805890066694);
} 