set dotenv-load := false

export RUSTFLAGS := "-Lbuild"

bootstrap:
    cargo install checkexec cargo-criterion

help:
    @just --list --unsorted

build:
    cargo build
alias b := build

run *args:
    cargo run {{args}}
alias r := run

release:
    cargo build --release

install:
    cargo install --path .

test *args: build-murmur3c
    cargo test {{args}} --features murmur3c

check:
    cargo check
alias c := check

bench:
    cargo criterion --features murmur3c

fix:
    cargo clippy --fix

# Bump version. level=major,minor,patch
version level:
    git diff-index --exit-code HEAD > /dev/null || ! echo You have untracked changes. Commit your changes before bumping the version.
    cargo set-version --bump {{level}}
    cargo update # This bumps Cargo.lock
    VERSION=$(rg  "version = \"([0-9.]+)\"" -or '$1' Cargo.toml | head -n1) && \
        git commit -am "Bump version {{level}} to $VERSION" && \
        git tag v$VERSION && \
        git push origin v$VERSION
    git push

publish:
    cargo publish

patch: test
    just version patch
    just publish

build-murmur3c:
    checkexec -v build/libmurmur3.so murmur3c/murmur3.c -- just _murmur3c

_murmur3c:
    mkdir -p build/
    cargo clean -p fastmurmur3
    export DIR=$(pwd) && cd murmur3c && gcc -shared -O3 -Wall -fPIC -o $DIR/build/libmurmur3.so -c murmur3.c