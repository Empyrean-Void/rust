use std::io;

fn main() {
    loop {
        println!("Select an operation:");
        println!("1. Add");
        println!("2. Subtract");
        println!("3. Multiply");
        println!("4. Divide");
        println!("5. Quit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        match choice.trim().parse() {
            Ok(1)  => add(),
            Ok(2)  => subtract(),
            Ok(3)  => multiply(),
            Ok(4)  => divide(),
            Ok(5)  => break,
            _=> println!("Invalid choice: Select 1 - 5."),
        }
    }
}

fn add() {
    let (x, y) = get_numbers();
    let result = x + y;
    println!("{} + {} = {}", x, y, result);
}

fn subtract() {
    let (x, y) = get_numbers();
    let result = x - y;
    println!("{} - {} = {}", x, y, result);

}

fn multiply() {
    let (x, y) = get_numbers();
    let result = x * y;
    println!("{} * {} = {}", x, y, result);

}

fn divide() {
    let (x, y) = get_numbers();

    if y == 0.0 {
        println!("Error: Cannot divide by zero!");
        return;
    }

    let result = x / y;
    println!("{} / {} = {}", x, y, result);
}

fn get_numbers() -> (f64, f64) {
    let mut x = String::new();
    let mut y = String::new();

    println!("Enter the first number:");
    io::stdin().read_line(&mut x).expect("Failed to read line");

    println!("Enter the second number:");
    io::stdin().read_line(&mut y).expect("Failed to read line");

    let x = x.trim().parse().expect("Not a valid number");
    let y = y.trim().parse().expect("Not a valid number");

    (x, y)
}
