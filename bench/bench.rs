use std::io::Cursor;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

static SOURCE: &'static [u8] = b"The quick brown fox jumps over the lazy dog";


fn sha1(data: &[u8]) -> [u8; 20] {
    use sha1::{Sha1, Digest};
    // let mut hasher = Sha1::new();
    <[u8; 20]>::from(Sha1::digest(data))
}


fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("sha1", |b| b.iter(||
        sha1(SOURCE)
    ));
    c.bench_function("murmur3rs", |b| b.iter(||
         benchhash::murmur3rs::hash(SOURCE)
    ));
    c.bench_function("murmur3c", |b| b.iter(||
        benchhash::murmur3c::hash(SOURCE)
    ));
    // c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}


criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);