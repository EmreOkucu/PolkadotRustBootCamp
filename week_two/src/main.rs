use std::io;

// Create an enum called Operation
enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

// Create a function called calculate
fn calculate(op: Operation) -> f64 {
    match op {
        Operation::Add(x, y) => x + y,
        Operation::Subtract(x, y) => x - y,
        Operation::Multiply(x, y) => x * y,
        Operation::Divide(x, y) => x / y,
    }
}

fn main() {
    // Prompt the user to input the first number
    let mut input = String::new();
    println!("Enter the first number:");
    io::stdin().read_line(&mut input).unwrap();
    let num1: f64 = input.trim().parse().unwrap();

    // Prompt the user to input the operation
    let mut input = String::new();
    println!("Enter the operation (Add, Subtract, Multiply, Divide):");
    io::stdin().read_line(&mut input).unwrap();
    let operation = input.trim();

    // Prompt the user to input the second number
    let mut input = String::new();
    println!("Enter the second number:");
    io::stdin().read_line(&mut input).unwrap();
    let num2: f64 = input.trim().parse().unwrap();

    // Create an Operation enum instance
    let op = match operation {
        "Add" => Operation::Add(num1, num2),
        "Subtract" => Operation::Subtract(num1, num2),
        "Multiply" => Operation::Multiply(num1, num2),
        "Divide" => Operation::Divide(num1, num2),
        _ => unimplemented!(), // Placeholder for other operations
    };

    // Call the calculate function
    let result = calculate(op);

    // Print the result to the console
    println!("The result is: {}", result);
}
