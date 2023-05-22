use cache_monet::cached;

use std::env;

#[cached]
fn fib(n: u128) -> u128 {
    if n <= 1 { n } 
    else { fib(n - 1) + fib(n - 2) }
}

fn main() {
    let n;

    let mut args = env::args();

    if args.len() != 2 {
        eprintln!("usage: fib <number>");
        std::process::exit(1);
    } else {
        n = args.nth(1)
            .unwrap()
            .parse::<u128>()
            .expect("Could not parse input as integer");
    }

    let answer = fib(n);

    println!("{answer}")
}