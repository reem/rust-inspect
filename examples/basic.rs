#[macro_use(inspect)]
extern crate inspect;

fn main() {
    let a = 7;
    inspect!(a, a + 4, a - 3);
    // Logs: "basic.rs - 6: a = 7, a + 4 = 11, a - 3 = 4"
}

