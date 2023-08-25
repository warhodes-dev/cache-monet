use cache_monet::cached;

use std::env;

#[cached(size = 50)]
fn fib(n: u128) -> u128{
    if n <= 1 { n } 
    else { fib(n - 1) + fib(n - 2) }
}

fn main() {
    let mut args = env::args();

    let n = args.nth(1)
        .unwrap_or_else(||{
            eprintln!("usage: fib <number>");
            std::process::exit(1);
        })
        .parse::<u128>()
        .expect("Failed to parse argument");

    let result = fib(n);
    println!("{result}")
}