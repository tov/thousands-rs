# thousands

[![Build Status](https://travis-ci.org/tov/thousands-rs.svg?branch=master)](https://travis-ci.org/tov/thousands-rs)
[![Crates.io](https://img.shields.io/crates/v/thousands.svg?maxAge=2592000)](https://crates.io/crates/thousands)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE-MIT)
[![License: Apache 2.0](https://img.shields.io/badge/license-Apache_2.0-blue.svg)](LICENSE-APACHE)

Provides a trait, `Separable`, for formatting numbers with separators
between the digits. Typically this will be used to add commas or spaces
every three digits from the right, but can be configured via a
`SeparatorPolicy`.

## Usage

It’s [on crates.io](https://crates.io/crates/thousands), so you can add

```toml
[dependencies]
thousands = "0.1.0"
```

to your `Cargo.toml`.

This crate supports Rust version 1.22 and newer.
