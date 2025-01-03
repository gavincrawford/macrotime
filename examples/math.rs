use timeit::*;

fn main() {
    println!("Adding 1 million times...");
    dbg_time!({
        let mut sum: i32 = 0;
        for i in 0..1_000_000 {
            sum = sum.wrapping_add(i);
        }
    });
    println!("Adding 5 million times...");
    dbg_time!({
        let mut sum: i32 = 0;
        for i in 0..5_000_000 {
            sum = sum.wrapping_add(i);
        }
    });
    println!("Multiplying 1 million times...");
    dbg_time!({
        let mut sum: i32 = 0;
        for i in 0..1_000_000 {
            sum = sum.wrapping_mul(i);
        }
    });
    println!("Multiplying 5 million times...");
    dbg_time!({
        let mut sum: i32 = 0;
        for i in 0..5_000_000 {
            sum = sum.wrapping_mul(i);
        }
    });
}
