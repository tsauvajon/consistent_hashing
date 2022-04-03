use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn main() {
    println!("Hello, world!");
}

/// In: key
/// Out: 0..255
fn hash(key: &str) -> u8 {
    let mut hasher = DefaultHasher::new();
    key.hash(&mut hasher);
    u8::try_from(hasher.finish() % 255).unwrap() // since we %255, it can't fail to convert into a u8.
}

#[test]
fn test_hash() {
    for (key, want) in vec![
        ("", 154),
        ("0", 234),
        ("1", 85),
        ("00", 80),
        ("01", 112),
        ("0123-4567-89ab-cdef", 128),
        ("0123-4567-89ab-cdee", 163),
        ("1234-5678-90ab-cdef", 54),
        ("abcd-ef12-3456-7890", 236),
    ] {
        let got = hash(key);
        assert_eq!(want, got, "key: {}", key);
    }
}
