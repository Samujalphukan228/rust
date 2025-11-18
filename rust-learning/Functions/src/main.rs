// Calling a Function

// Example
fn print() {
    println!("samujal")
}
print();

// Functions with Parameters
// Example
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

greet("John");

// Functions with Return Values
// Example
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

let sum = add(3, 4);
println!("Sum is: {}", sum);
