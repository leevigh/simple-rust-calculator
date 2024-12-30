use std::io;

fn main() {
    println!("Hello, world!");

    let mut first_number = String::new();
    let mut second_number = String::new();

    let mut operation = String::new();

    println!("Enter first number: ");
    io::stdin()
        .read_line(&mut first_number)
        .expect("Failed to read line");
    let first_number: f64 = first_number
        .trim()
        .parse()
        .expect("Please enter valid number");

    println!("Enter Operation: ");
    io::stdin()
        .read_line(&mut operation)
        .expect("Failed to read line");
    let operation_str = operation.trim();

    println!("Enter second number: ");
    io::stdin()
        .read_line(&mut second_number)
        .expect("Failed to read line");
    let second_number: f64 = second_number
        .trim()
        .parse()
        .expect("Please enter valid number");

    let operation = match operation_str {
        "+" => Operation::Add(first_number, second_number),
        "-" => Operation::Subtract(first_number, second_number),
        "*" => Operation::Multiply(first_number, second_number),
        "/" => Operation::Divide(first_number, second_number),

        "Add" => Operation::Add(first_number, second_number),
        "Subtract" => Operation::Subtract(first_number, second_number),
        "Multiply" => Operation::Multiply(first_number, second_number),
        "Divide" => Operation::Divide(first_number, second_number),

        "add" => Operation::Add(first_number, second_number),
        "subtract" => Operation::Subtract(first_number, second_number),
        "multiply" => Operation::Multiply(first_number, second_number),
        "divide" => Operation::Divide(first_number, second_number),

        _ => {
            println!("Invalid operation");
            return;
        }
    };

    match calculate(operation) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}

enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

fn calculate(operation: Operation) -> Result<f64, String> {
    match operation {
        Operation::Add(x, y) => Ok(x + y),
        Operation::Subtract(x, y) => Ok(x - y),
        Operation::Multiply(x, y) => Ok(x * y),
        Operation::Divide(x, y) => {
            if y != 0.0 || y > 0.0 {
                Ok(x / y)
            } else {
                Err(String::from("Cannot divide by zero or negative number"))
            }
        }
    }
}
