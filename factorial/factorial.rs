// The factorial function will call itself recursively and implement
// the expression n! = n * (n-1) * (n-2) *...(n - (n-1))
fn factorial(n: u64) -> u64 {
    if n <= 1 {
        return 1;
    }
    n * factorial(n - 1)
}


fn main() {
    let mut input_text = String::new();
    // Get the users input from the command line
    std::io::stdin()
        .read_line(&mut input_text)
        .expect("Failed to read line");

    let num: u64 = input_text.trim().parse::<u64>().expect("That's not an integer");

    println!("Factorial: {}", factorial(num));
}
