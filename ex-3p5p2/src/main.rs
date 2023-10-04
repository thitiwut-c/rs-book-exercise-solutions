use std::io::{self, Write};

fn main() {
    println!("Fibonacci generation");
    print!("n: ");
    io::stdout().flush().expect("Flush failed");

    let mut n = String::new();

    io::stdin().
        read_line(&mut n).
        expect("Failed read line");

    let n: u32 = n.trim().parse().expect("Parse failed");
    let ans = fib(n);

    println!("Fib({n})={ans}")
}

fn fib(n: u32) -> u64 {
    if n <= 1 {
        return n.into();
    }

    fib(n-1)+fib(n-2)
}
