use macrotime::*;

fn fib(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }
    fib(n - 1) + fib(n - 2)
}

fn main() {
    println!("Getting the 8th fibonacci number...");
    let result = dbg_time!("fib 8", { fib(8) });
    println!("Result: {}", result);

    println!("Getting the 16th fibonacci number...");
    let result = dbg_time!("fib 16", { fib(16) });
    println!("Result: {}", result);
}
