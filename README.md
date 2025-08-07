# [MacroTime](https://crates.io/crates/macrotime)

This crate implements macros that can be used to time arbitrary snippets of code. This allows you to easily test functions on the go, without any fancy boilerplate for benchmarking.

# Example

To time a snippet of code, simply:
```rust
use macrotime::*;
dbg_time!("task", {
    // write any code here...
});
```
And the execution time will be logged in the most relevant unit with the 'task' context added for readability. It's that simple.

# Documentation

If you'd like to see more examples or learn more about `MacroTime`, take a look at the [documentation](https://docs.rs/macrotime/). All information listed there should be up to date and tested for the most recent version of the package.
