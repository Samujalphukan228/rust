// Loops
// Rust has three types of loops: loop, while, and for

loop {
    println!("This will repeat forever!");// infinite loop  Ctrl + C to end  or use break
}


// Example
let mut count = 0;

loop {
    println! ("hi i am samujal");

    if count === 3 {
        break;
    }

    count += 1
}

// Return a Value
// Example
let mut count = 0;

let result = loop{
    println!("samujal")

    if count == 10 {
        break count
    }
    count += 1;
}

println!("The loop stopped at: {}", result);
