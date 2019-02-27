[![Build Status](https://travis-ci.com/kurtlawrence/cratesiover.svg?branch=master)](https://travis-ci.com/kurtlawrence/cratesiover)
[![Latest Version](https://img.shields.io/crates/v/cratesiover.svg)](https://crates.io/crates/cratesiover)
[![Rust Documentation](https://img.shields.io/badge/api-rustdoc-blue.svg)](https://docs.rs/cratesiover)
[![codecov](https://codecov.io/gh/kurtlawrence/cratesiover/branch/master/graph/badge.svg)](https://codecov.io/gh/kurtlawrence/cratesiover)

Query and compare the semver of a crate on crates.io.

See the [rs docs](https://docs.rs/cratesiover/). [Github repo.](https://github.com/kurtlawrence/cratesiover)

# Example

```rust
use cratesiover::Status;
let query = cratesiover::query("cratesiover", &env!("CARGO_PKG_VERSION")).unwrap();

match query {
  Status::Behind(ver) => println!("crate is behind the version on crates.io {}", ver),
  Status::Equal(ver) => println!("crate is equal to the version on crates.io {}", ver),
  Status::Ahead(ver) => println!("crate is ahead of the version on crates.io {}", ver),
}
```