use std::io;

fn main() {
    println!("Enter a number:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let number: f64 = input.trim().parse().expect("Input is not a number");
    println!("The number is: {}", number);
}
