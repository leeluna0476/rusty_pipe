use std::io;

fn main() {
    println!("Enter a number in a range of [0, 93]");

    let number: u8 = loop {
        let mut number = String::new();
        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read line");

        let number: u8 = match number.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("{e}");
                continue;
            },
        };

        if number > 93 {
            println!("Too big. Please enter a number in a range of [0, 93]");
        } else {
            break number;
        }
    };

    let fib = fibonacci(number);
    println!("F({number}) = {fib}");
}

fn fibonacci(n: u8) -> u64 {
    let mut memo: [u64; 256] = [0; 256];
    _inner_fibonacci(n, &mut memo)
}

fn _inner_fibonacci(n: u8, memo: &mut [u64; 256]) -> u64 {
    if n == 1 {
        memo[n as usize] = 1;
    } else if n > 0 && memo[n as usize] == 0 {
        memo[n as usize] = _inner_fibonacci(n - 1, memo) + _inner_fibonacci(n - 2, memo);
    }
    memo[n as usize]
}
