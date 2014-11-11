#![deny(missing_docs)]
#![feature(macro_rules)]

//! A lightweight library for quickly debugging rust code.

/// Logs the file, line number, and expressions along with what they equal.
///
/// For instance:
///
/// ```ignore
/// let a = 7;
/// inspect!(a, a + 4); //=> file.rs - 2: a = 7, a + 4 = 11
/// ```
///
#[macro_export]
macro_rules! inspect(
    ($($a:expr),*) => {
        println!(
            "{} - {}: {}",
            file!(), line!(),
            format!(
                concat!($(stringify!($a), " = {}, "),*), $($a),*
            )
        );
    }
)

