// The while Loop
// The while loop runs as long as a condition is true.

// Example;
let mut count = 1;

while count <= 5 {
    println!("Count: {}", count);
    count += 1;
}

// Stop a While Loop
// Example
let mut num = 1;

while num <= 10 {
    if num == 6 {
        break;
    }
    println!("Number: {}", num);
    num += 1;
}

// Skip a Value
// Example
let mut num = 1;

while num <= 10 {
    if num == 6 {
        num += 1;
        continue;
    }

    println!("Number: {}", num);
    num += 1;
}
