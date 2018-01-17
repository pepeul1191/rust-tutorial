use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

#[derive(Hash)]
struct Person {
    id: u32,
    name: String,
    phone: u64,
}