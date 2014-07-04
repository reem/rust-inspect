# Inspect [![Build Status](https://secure.travis-ci.org/reem/rust-inspect.png?branch=master)](https://travis-ci.org/reem/rust-inspect)

A lightweight library providing a very useful `inspect!` macro that
logs meta information like the filename and line number in addition
to the expressions you debug and their results.

## Example

```rust
#![feature(phase)]

extern crate debug;
#[phase(plugin, link)]
extern crate inspect;

fn main() {
    let a = 7u;
    let b = 4u;
    inspect!(a, b, a + b);
    // => file.rs - 10: a = 7, b = 4, a + b = 11
}
```

## Install

Add:

```toml
[dependencies.inspect]

git = "https://github.com/reem/rust-inspect"
```

to your `Cargo.toml`.

## License

MIT

