[![Build Status](https://travis-ci.com/kurtlawrence/cratesiover.svg?branch=master)](https://travis-ci.com/kurtlawrence/cratesiover) [![Latest Version](https://img.shields.io/crates/v/cratesiover.svg)](https://crates.io/crates/cratesiover) [![Rust Documentation](https://img.shields.io/badge/api-rustdoc-blue.svg)](https://docs.rs/cratesiover) [![codecov](https://codecov.io/gh/kurtlawrence/cratesiover/branch/master/graph/badge.svg)](https://codecov.io/gh/kurtlawrence/cratesiover)
[![Rustc Version 1.30+](https://img.shields.io/badge/rustc-1.30+-blue.svg)](https://blog.rust-lang.org/2018/10/25/Rust-1.30.0.html)

Query and compare the semver of a crate on crates.io.

See the [rs docs](https://docs.rs/cratesiover/). [Github repo.](https://github.com/kurtlawrence/cratesiover)

# Example

```rust
use cratesiover::Status;
let query = cratesiover::query("cratesiover", &env!("CARGO_PKG_VERSION")).unwrap();

match query {
  Status::Behind => println!("crate is behind the version on crates.io"),
  Status::Equal => println!("crate is equal to the version on crates.io"),
  Status::Ahead => println!("crate is ahead of the version on crates.io"),
}
```