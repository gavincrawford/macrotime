pub use std::time::{Duration, Instant};

#[macro_export]
macro_rules! dbg_time {
    ($x:expr) => {
        // time expression
        let now = $crate::Instant::now();
        $x();
        let elapsed = now.elapsed();

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

// TODO tests
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn testfn() {}
// }
