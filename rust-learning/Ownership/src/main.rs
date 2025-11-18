// Ownership

// Ownership Rules
// Each value has one owner
// When the owner goes out of scope, the value is deleted
// You can only have one owner at a time, unless you borrow it (covered in the next chapter)

// Basic Ownership Example
fn main() {
    let a = String::from("Hello");
    let b = a;

    // println!("{}", a); Error: a no longer owns the value
    println!("{}", b); // Ok: b now owns the value
}


// When we assign a to b, the ownership moves. This means only b can use the value now, because a is no longer valid.

// But simple types like numbers, characters and booleans are copied, not moved.

// This means you can still use the original variable after assigning it to another:

// Example
// let a = 5;
// let b = a;
// println!("a = {}", a); 
// println!("b = {}", b); 



// Clone
// .clone()
// Example
let a = String::from("Hello");
let b = a.clone(); 

println!("a = {}", a);  
println!("b = {}", b);  