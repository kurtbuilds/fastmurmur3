use std::io::Cursor;

pub mod murmur3rs;
pub mod murmur3c;

static SOURCE: &'static [u8] = b"The quick brown fox jumps over the lazy dog";


fn main() {
    let a = murmur3rs::murmur3_x64_128(SOURCE, 0);
    let b = murmur3c::hash(SOURCE);
    assert_eq!(a, b);
    println!("Hello, world!");
}
