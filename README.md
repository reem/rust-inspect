# Line Debug

A lightweight library providing a very useful `debug!` macro that
logs meta information like the filename and line number in addition
to the expressions you debug and their results.

## Example

```rust
#![feature(phase)]

extern crate debug;
#[phase(syntax, link)]
extern crate line_debug;

fn main() {
    let a = 7;
    let b = 4;
    debug!(a, b, a + b);
    // => file.rs - 10: a = 7, b = 4, a + b = 11
}
```

To use just clone, run make, then copy the desired binary from
target/cpu-vendor-os

## License

MIT
