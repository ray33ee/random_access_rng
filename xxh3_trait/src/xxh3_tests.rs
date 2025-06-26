#[cfg(test)]
mod tests {
    use crate::XXH3;

    #[test]
    fn test_numbers_eq_xxh3() {
        assert_eq!(10u32.xxh3(), 10u32.xxh3());
        assert_eq!(10i32.xxh3(), 10u32.xxh3());
    }

    #[test]
    fn test_numbers_ne_xxh3() {
        assert_ne!(10u32.xxh3(), 10u64.xxh3());
        assert_ne!((-10i32).xxh3(), 10u32.xxh3());
        assert_ne!(10u32.xxh3(), 11u32.xxh3());
        assert_ne!(1u32.xxh3(), 3u32.xxh3());
    }

}