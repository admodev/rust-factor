use std::io::stdin;

fn factorial(n: u64) -> u64 {
    if n == 0 || n == 1 {
        return n;
    }

    return n * factorial(n - 1);
}

fn parse_factorial() -> String {
    let mut stdinput = String::new();

    println!("Please, enter a number: ");

    let user_input = stdin().read_line(&mut stdinput).unwrap();
    let user_input_str = user_input.to_string();
    let factor = user_input_str.parse::<u64>().unwrap();

    println!("Factorial of {} is: {}", stdinput, factorial(factor));

    return stdinput;
}

fn main() {
    parse_factorial();
}
