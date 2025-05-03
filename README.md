# `next`

[![Crates.io](https://img.shields.io/crates/v/next.svg)](https://crates.io/crates/next)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/Seldom-SE/next#license)
[![Crates.io](https://img.shields.io/crates/d/next.svg)](https://crates.io/crates/next)

`next` is a crate that provides a trait that gets the next value. That value is the next in the
sequence implied by `PartialOrd`.

## Usage

Add to your `Cargo.toml`

```toml
# Replace * with your desired version
[dependencies]
next = "*"
```

You can put `#[derive(Next)]` on your types and get the next value of a type with `.next()`.

## License

`next` is dual-licensed under MIT and Apache 2.0 at your option.

## Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion
in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above,
without any additional terms or conditions.
