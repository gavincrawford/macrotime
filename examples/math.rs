use macrotime::*;

fn main() {
    println!("Adding 1 million times...");
    dbg_time!("sum 1million", {
        let mut sum: i32 = 0;
        for i in 0..1_000_000 {
            sum = sum.wrapping_add(i);
        }
    });
    println!("Adding 5 million times...");
    dbg_time!("sum 5million", {
        let mut sum: i32 = 0;
        for i in 0..5_000_000 {
            sum = sum.wrapping_add(i);
        }
    });
    println!("Multiplying 1 million times...");
    dbg_time!("mul 1million", {
        let mut sum: i32 = 0;
        for i in 0..1_000_000 {
            sum = sum.wrapping_mul(i);
        }
    });
    println!("Multiplying 5 million times...");
    dbg_time!("mul 5million", {
        let mut sum: i32 = 0;
        for i in 0..5_000_000 {
            sum = sum.wrapping_mul(i);
        }
    });
}
