//! # MacroTime
//!
//! ## `time!`
//!
//! `MacroTime` is very easy to use. To time the execution of a block of code:
//! ```
//! use macrotime::*;
//! let time = time!({
//!     // do some stuff...
//! });
//! println!("This operation took {} ms!", time.as_millis());
//! ```
//!
//! ## `dbg_time!`
//!
//! You can also have `MacroTime` print the time taken to perform a task on its own. To do this:
//! ```
//! use macrotime::*;
//! dbg_time!("context...", {
//!     // do some stuff...
//! });
//! ```
//! In this scenario, the time will printed in the most relevant unit, so no need for formatting.
//!
//! Return statements within the debug macro will be propogated to the outer scope, and can be used
//! elsewhere in the code, for minimal disruption to program flow.
//! ```
//! # use macrotime::*;
//! let result = dbg_time!("returning 123...", {123});
//! ```

/// Prints the execution time of the provided expression along with a relevant title/message.
/// Returns the result of the expression.
///
/// # Example
/// ```
/// # use macrotime::*;
/// let result = dbg_time!("some stuff", {
///     println!("do some stuff...");
///     42
/// });
/// assert_eq!(result, 42);
/// ```
#[macro_export]
macro_rules! dbg_time {
    ($id:expr, $x:expr) => {{
        let now = std::time::Instant::now();
        let result = $x;
        let elapsed = now.elapsed();

        // log elapsed time
        print!("{}: ", $id);
        if elapsed < std::time::Duration::from_nanos(1000) {
            println!("{} ns", elapsed.as_nanos());
        } else if elapsed < std::time::Duration::from_micros(1000) {
            println!("{:.2} μs", elapsed.as_nanos() as f64 / 1000.0);
        } else if elapsed < std::time::Duration::from_millis(1000) {
            println!("{:.2} ms", elapsed.as_micros() as f64 / 1000.0);
        } else {
            println!("{:.2} s", elapsed.as_millis() as f64 / 1000.0);
        }

        result
    }};
}

/// Returns a `Duration` of the execution time of the provided expression.
/// # Example
/// ```
/// # use macrotime::*;
/// let duration = time!({
///     println!("do some stuff...");
/// });
/// ```
#[macro_export]
macro_rules! time {
    ($x:expr) => {{
        let now = std::time::Instant::now();
        let _result = $x;
        now.elapsed()
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::hint::black_box;

    #[test]
    fn time_is_nonzero() {
        let time = time!({
            let mut sum: i32 = black_box(0);
            for i in 0..1_000 {
                sum = sum.wrapping_add(i);
            }
        });
        assert!(!time.is_zero());
    }

    #[test]
    fn time_returns_rational_duration() {
        let duration = time!({
            std::thread::sleep(std::time::Duration::from_millis(1));
        });
        assert!(duration >= std::time::Duration::from_millis(1));
        assert!(duration < std::time::Duration::from_millis(100));
    }

    #[test]
    fn dbg_time_returns_value() {
        let value = dbg_time!("computation", { 5 * 10 });
        assert_eq!(value, 50);
    }
}
