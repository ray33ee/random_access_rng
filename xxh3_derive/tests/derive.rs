use xxh3_derive::XXH3;
use xxh3_trait::XXH3;

#[derive(XXH3)]
struct SimpleNamedStruct {
    a: u64,
}

#[derive(XXH3)]
struct SimpleUnnamedStruct(u64);

#[derive(XXH3)]
enum SimpleUnitEnum {
    First,
    Second,
}

#[derive(XXH3)]
enum SimpleUnnamedEnum {
    A(u64),
    B(u64),
}

#[test]
fn test_simple_named_struct() {
    assert_eq!(SimpleNamedStruct{ a: 99}.xxh3(), 313460803888701961170056283962404596353u128);
}

#[test]
fn test_simple_unnamed_struct() {
    assert_eq!(SimpleUnnamedStruct(99).xxh3(), 313460803888701961170056283962404596353u128);
}

#[test]
fn test_struct_equality() {
    assert_eq!(SimpleNamedStruct{ a: 99}.xxh3(), SimpleUnnamedStruct(99).xxh3());
}

#[test]
fn test_unit_enum_inequality() {
    assert_ne!(SimpleUnitEnum::First.xxh3(), SimpleUnitEnum::Second.xxh3());
}

#[test]
fn test_unit_unnamed_inequality() {
    assert_ne!(SimpleUnnamedEnum::A(100).xxh3(), SimpleUnnamedEnum::B(100).xxh3());
    assert_ne!(SimpleUnnamedEnum::A(100).xxh3(), SimpleUnnamedEnum::A(50).xxh3());
    assert_ne!(SimpleUnnamedEnum::B(100).xxh3(), SimpleUnnamedEnum::B(50).xxh3());
}

#[test]
fn test_unit_unnamed_equality() {
    assert_eq!(SimpleUnnamedEnum::A(100).xxh3(), SimpleUnnamedEnum::A(100).xxh3());
    assert_eq!(SimpleUnnamedEnum::B(1).xxh3(), SimpleUnnamedEnum::B(1).xxh3());
} 