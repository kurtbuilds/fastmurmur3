use criterion::{BenchmarkId, criterion_group, criterion_main, Criterion};
#[cfg(feature = "fasthash")]
use fasthash::murmur3::Hash128_x64;
#[cfg(feature = "fasthash")]
use fasthash::FastHash;
use highway::HighwayHash;
use std::hash::Hasher;
use std::io::Cursor;
use rand::RngCore;

fn sha1(data: &[u8]) -> [u8; 20] {
    use sha1::{Digest, Sha1};
    // let mut hasher = Sha1::new();
    <[u8; 20]>::from(Sha1::digest(data))
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("hashes");

    for size in [16, 20, 32, 40, 64, 70, 128, 130, 256, 260, 512, 520, 1024, 1030, 2048, 2050, 4096, 5500] {
        let mut rng = rand::thread_rng();
        let mut buf = vec![0; size];
        rng.fill_bytes(&mut buf);

        group.bench_with_input(BenchmarkId::new("sha1", size), &size, |b, _size| {
            b.iter(|| sha1(&buf))
        });

        group.bench_with_input(BenchmarkId::new("fastmurmur3", size), &size, |b, _size| {
            b.iter(|| fastmurmur3::hash(&buf))
        });

        #[cfg(feature = "murmur3c")]
        group.bench_with_input(BenchmarkId::new("murmur3c", size), &size, |b, _size| {
            b.iter(|| fastmurmur3::murmur3c::hash(&buf))
        });

        #[cfg(feature = "fasthash")]
        group.bench_with_input(BenchmarkId::new("fasthash", size), &size, |b, _size| {
            b.iter(|| Hash128_x64::hash(&buf))
        });

        group.bench_with_input(BenchmarkId::new("murmur3", size), &size, |b, _size| {
            b.iter(|| murmur3::murmur3_x64_128(&mut Cursor::new(&buf), 0))
        });

        group.bench_with_input(
            BenchmarkId::new("twox_hash::Xxh3Hash128", size),
            &size,
            |b, _size| {
                b.iter(|| {
                    let mut h = twox_hash::xxh3::Hash128::default();
                    h.write(&buf);
                    h.finish()
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("twox_hash::Xxh3Hash64", size),
            &size,
            |b, _size| {
                b.iter(|| {
                    let mut h = twox_hash::xxh3::Hash64::default();
                    h.write(&buf);
                    h.finish()
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("xxhash_rust::xxh3_64", size),
            &size,
            |b, _size| b.iter(|| xxhash_rust::xxh3::xxh3_64(&buf)),
        );

        group.bench_with_input(
            BenchmarkId::new("xxhash_rust::xxh3_128", size),
            &size,
            |b, _size| b.iter(|| xxhash_rust::xxh3::xxh3_128(&buf)),
        );

        group.bench_with_input(
            BenchmarkId::new("highway::HighwayHasher::hash128", size),
            &size,
            |b, _size| b.iter(|| highway::HighwayHasher::default().hash128(&buf)),
        );
    }

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
