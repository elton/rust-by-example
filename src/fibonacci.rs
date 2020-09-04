use std::io;
pub fn main() {
    println!("Fibonacci sequence");
    println!("Please enter nth:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");

    let nth: u32 = input.trim().parse().expect("Please type a number.");
    println!("first {} nth fabonacci:", nth);

    for elem in 1..nth + 1 {
        print!("{}, ", fibonacci(elem));
    }
}

fn fibonacci(n: u32) -> u32 {
    let mut a = 1;
    let mut b = 1;
    let mut i = 3;
    let mut fi = 0;

    if n == 1 || n == 2 {
        return 1;
    }

    while i <= n {
        fi = a + b;
        a = b;
        b = fi;
        i = i + 1;
    }

    fi
}
