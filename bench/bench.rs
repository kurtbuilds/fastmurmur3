use std::hash::Hasher;
use std::io::Cursor;
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use fasthash::FastHash;
use fasthash::murmur3::Hash128_x64;
use rustc_hash;

static SOURCE: &'static [u8] = b"The quick brown fox jumps over the lazy dog";


fn sha1(data: &[u8]) -> [u8; 20] {
    use sha1::{Sha1, Digest};
    // let mut hasher = Sha1::new();
    <[u8; 20]>::from(Sha1::digest(data))
}


fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("hashes");

    group.bench_function("sha1", |b| b.iter(||
        sha1(SOURCE)
    ));
    group.bench_function("fastmurmur3", |b| b.iter(||
         fastmurmur3::hash(SOURCE)
    ));
    group.bench_function("murmur3c", |b| b.iter(||
        fastmurmur3::murmur3c::hash(SOURCE)
    ));
    group.bench_function("fasthash", |b| b.iter(||
        Hash128_x64::hash(SOURCE)
    ));
    group.bench_function("murmur3", |b| b.iter(||
        murmur3::murmur3_x64_128(&mut Cursor::new(SOURCE),0).unwrap()
    ));
    group.bench_function("twox_hash::Xxh3Hash128", |b| b.iter(|| {
        let mut h = twox_hash::xxh3::Hash128::default();
        h.write(SOURCE);
        h.finish()
    }));
    group.bench_function("twox_hash::Xxh3Hash64", |b| b.iter(|| {
        let mut h = twox_hash::xxh3::Hash64::default();
        h.write(SOURCE);
        h.finish()
    }));
    group.bench_function("xxhash_rust::xxh3_64", |b| b.iter(||
        xxhash_rust::xxh3::xxh3_64(SOURCE)
    ));
    group.bench_function("xxhash_rust::xxh3_128", |b| b.iter(||
        xxhash_rust::xxh3::xxh3_128(SOURCE)
    ));
    group.bench_function("rustc_hash", |b| b.iter(|| {
        let mut h = rustc_hash::FxHasher::default();
        h.write(SOURCE);
        h.finish()
    }));

    group.finish();
}


criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);