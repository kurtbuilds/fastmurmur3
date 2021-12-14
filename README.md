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

Murmur3 is a fast, non-cryptographic hash function.

# Usage

# Installation

    cargo install fastmurmur3

# Benchmarks

```
sha1                    time:   [202.16 ns 202.75 ns 203.33 ns]
                        change: [-11.940% -10.505% -9.0308%] (p = 0.00 < 0.05)
                        Performance has improved.

fastmurmur3             time:   [10.463 ns 10.501 ns 10.540 ns]
                        change: [-14.418% -12.927% -11.468%] (p = 0.00 < 0.05)
                        Performance has improved.

murmur3c                time:   [14.760 ns 14.856 ns 14.957 ns]
                        change: [-3.7754% -2.6999% -1.6890%] (p = 0.00 < 0.05)
                        Performance has improved.

fasthash                time:   [13.950 ns 14.002 ns 14.055 ns]
                        change: [-2.1067% -1.2594% -0.1494%] (p = 0.00 < 0.05)
                        Change within noise threshold.

murmur3                 time:   [26.682 ns 26.781 ns 26.914 ns]
                        change: [+0.2948% +1.9060% +4.0986%] (p = 0.05 > 0.05)
                        No change in performance detected.

twox_hash::Xxh3Hash128  time:   [130.32 ns 131.41 ns 132.56 ns]

twox_hash::Xxh3Hash64   time:   [129.68 ns 130.57 ns 131.37 ns]

xxhash_rust::xxh3_64    time:   [5.1691 ns 5.2001 ns 5.2356 ns]

xxhash_rust::xxh3_128   time:   [7.8371 ns 7.8689 ns 7.9027 ns]
```

For murmur3, fastmurmur3 is the fastest, 1.33x faster than the next best implementation.

However, the xxhash_rust implementation of xxh3hash is the fastest, 2.02x faster than fastmurmur3.

# Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

# Acknowledgments

Use this space to list resources you find helpful and would like to give credit to. I've included a few of my favorites to kick things off!

* [Choose an Open Source License](https://choosealicense.com)
* [GitHub Emoji Cheat Sheet](https://www.webpagefx.com/tools/emoji-cheat-sheet)
* [Malven's Flexbox Cheatsheet](https://flexbox.malven.co/)
* [Malven's Grid Cheatsheet](https://grid.malven.co/)
* [Img Shields](https://shields.io)
* [GitHub Pages](https://pages.github.com)
* [Font Awesome](https://fontawesome.com)
* [React Icons](https://react-icons.github.io/react-icons/search)

<p align="right">(<a href="#top">back to top</a>)</p>