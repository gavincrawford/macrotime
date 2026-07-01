# [MacroTime](https://crates.io/crates/macrotime)

[![Crates.io](https://img.shields.io/crates/v/macrotime.svg)](https://crates.io/crates/macrotime)
[![docs.rs](https://img.shields.io/docsrs/macrotime)](https://docs.rs/macrotime)

This crate implements macros that can be used to time arbitrary snippets of code. This allows you to easily test functions on the go, without any fancy boilerplate for benchmarking.

# Installation

Use `cargo add` to add `macrotime` to any package/library:
```sh
cargo add macrotime
```

# Example

To time a snippet of code, simply:
```rust
use macrotime::*;
dbg_time!("task", {
    // write any code here...
});
```
And the execution time will be logged in the most relevant unit with the 'task' context added for readability. It's that simple.

`dbg_time!` also returns the result of the expression it times, so it can be dropped into existing code without disrupting the flow of the program:
```rust
use macrotime::*;
let result = dbg_time!("task", {
    // write any code here...
    123
});
```

If you'd rather handle the timing yourself, `time!` returns a plain `Duration` instead of printing anything:
```rust
use macrotime::*;
let duration = time!({
    // write any code here...
});
println!("This operation took {} ms!", duration.as_millis());
```

# Documentation

If you'd like to see more examples or learn more about `MacroTime`, take a look at the [documentation](https://docs.rs/macrotime/). All information listed there should be up to date and tested for the most recent version of the package.
