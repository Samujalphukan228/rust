use std::io;

fn main() {
    println!("Enter first number:");
    let mut first: String = String::new();
    io::stdin().read_line(&mut first).unwrap();
    let a: f64 = first.trim().parse().unwrap();

    println!("Enter operator (+ - * /):");
    let mut op: String = String::new();
    io::stdin().read_line(&mut op).unwrap();
    let operator: &str = op.trim();

    println!("Enter second number:");
    let mut second: String = String::new();
    io::stdin().read_line(&mut second).unwrap();
    let b: f64 = second.trim().parse().unwrap();

    let result = match operator {
        "+" => a + b,
        "-" => a - b,
        "*" => a * b,
        "/" => a / b,
        _ => {
            println!("Invalid operator");
            return;
        }
    };

    println!("Result = {}", result);
}
