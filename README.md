# Inspect [![Build Status](https://secure.travis-ci.org/reem/rust-inspect.png?branch=master)](https://travis-ci.org/reem/rust-inspect)

A lightweight library providing a very useful `inspect!` macro that
logs meta information like the filename and line number in addition
to the expressions you debug and their results.

## Example

```rust
#![feature(phase)]

extern crate debug;
#[phase(syntax, link)]
extern crate inspect;

fn main() {
    let a = 7;
    let b = 4;
    inspect!(a, b, a + b);
    // => file.rs - 10: a = 7, b = 4, a + b = 11
}
```

To use just clone, run make, then copy the desired binary from
target/cpu-vendor-os

## License

MIT
