#![feature(phase)]

extern crate debug;
#[phase(syntax, link)]
extern crate inspect;

fn main() {
    let a = 7;
    inspect!(a, a + 4, a - 3);
    // Logs: "hello.rs - 9: a = 7, a + 4 = 11, a - 3 = 4"
}
