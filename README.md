<div id="top"></div>

<p align="center">
<a href="https://github.com/kurtbuilds/fastmurmur3/graphs/contributors">
    <img src="https://img.shields.io/github/contributors/kurtbuilds/fastmurmur3.svg?style=flat-square" alt="GitHub Contributors" />
</a>
<a href="https://github.com/kurtbuilds/fastmurmur3/stargazers">
    <img src="https://img.shields.io/github/stars/kurtbuilds/fastmurmur3.svg?style=flat-square" alt="Stars" />
</a>
<a href="https://github.com/kurtbuilds/fastmurmur3/actions">
    <img src="https://img.shields.io/github/workflow/status/kurtbuilds/fastmurmur3/test?style=flat-square" alt="Build Status" />
</a>
<a href="https://crates.io/crates/fastmurmur3">
    <img src="https://img.shields.io/crates/d/fastmurmur3?style=flat-square" alt="Downloads" />
</a>
<a href="https://crates.io/crates/fastmurmur3">
    <img src="https://img.shields.io/crates/v/fastmurmur3?style=flat-square" alt="Crates.io" />
</a>

</p>

# Fastmurmur3

Murmur3 is a fast, non-cryptographic hash function. Fastmurmur3 is, in my testing, the fastest implementation of Murmur3.

# Usage

    let bytes: &[u8] = b"hello world";
    let x: u128 = fastmurmur3::hash(bytes);

# Installation

    cargo install fastmurmur3

# Benchmarks

### Summary

According to current testing (and excluding `FxHasher`):

- `fastmurmur3` is the fastest.
- `xxh3_64` is 1.66x slower and only has a 64-bit value.
- `xxh3_128` is 2.50x slower.
- `fasthash` contains the next fastest murmur3 implementation, but is still 4.47x slower than `fastmurmur3`.

### Data

```
rustc_hash::FxHasher    time:   [243.91 ps 244.60 ps 245.30 ps]

fastmurmur3             time:   [3.0878 ns 3.1215 ns 3.1619 ns]
xxhash_rust::xxh3_64    time:   [5.1473 ns 5.1872 ns 5.2456 ns]
xxhash_rust::xxh3_128   time:   [7.8066 ns 7.8271 ns 7.8499 ns]
fasthash                time:   [13.909 ns 13.960 ns 14.018 ns]
murmur3c                time:   [14.529 ns 14.604 ns 14.684 ns]
murmur3                 time:   [26.084 ns 26.163 ns 26.249 ns]
twox_hash::Xxh3Hash64   time:   [124.23 ns 126.46 ns 128.55 ns]
twox_hash::Xxh3Hash128  time:   [134.62 ns 136.75 ns 138.77 ns]
sha1                    time:   [209.55 ns 211.71 ns 214.88 ns]
```

### Benchmark Future Work

- These benchmarks are run on a limited input set and with a limited seed. It'd be better to run these benchmarks 
  on a larger input set and with a larger seed.

- Besides speed, these benchmarks could also measure other important hash properties like collision-resistance. 

- I exclude `FxHash` from the benchmark summary because I don't know if it's been tested for general use beyond
  the rust compiler. If that has been done, and it proves to be general purpose, 
  FxHash could be preferable to `fastmurmur3`.

- It'd be nice to have pretty charts of the benchmarks.

# Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request