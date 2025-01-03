//! # MacroTime
//! `MacroTime` is very easy to use. To time the execution of a block of code:
//! ```
//! use macrotime::*;
//! let time = time!({
//!     // do some stuff...
//! });
//! println!("This operation took {} ms!", time.as_millis());
//! ```
//! You can also have `MacroTime` print the time taken to perform a task on its own. To do this:
//! ```
//! use macrotime::*;
//! dbg_time!({
//!     // do some stuff...
//! });
//! ```
//! In this scenario, the time will printed in the most relevant unit, so no need for formatting.

pub use std::time::{Duration, Instant};

/// Prints the execution time of the provided expression.
#[macro_export]
macro_rules! dbg_time {
    ($x:expr) => {
        // time expression
        let elapsed = $crate::time!($x);

        // log elapsed time
        if elapsed < $crate::Duration::from_millis(1) {
            println!("{} us", elapsed.as_nanos());
        } else if elapsed < $crate::Duration::from_secs(1) {
            println!("{} ms", elapsed.as_millis());
        } else {
            println!("{} s", elapsed.as_secs());
        }
    };
}

/// Returns a `Duration` of the execution time of the provided expression.
#[macro_export]
macro_rules! time {
    ($x:expr) => {{
        let now = $crate::Instant::now();
        $x();
        now.elapsed()
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Asserts that two operations of different complexities result in rational computation times.
    #[test]
    fn time() {
        let a = time!({
            let mut sum: i32 = 0;
            for i in 0..1_000 {
                sum = sum.wrapping_add(i);
            }
        });
        let b = time!({
            let mut sum: i32 = 0;
            for a in 0..1_000 {
                for b in 0..1_000 {
                    sum = sum.wrapping_add(a * b);
                }
            }
        });
        assert!(a < b);
    }
}
