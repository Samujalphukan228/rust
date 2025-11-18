// The for Loop
// When you know exactly how many times you want to loop through a block of code, use the for loop together with the in keyword, instead of a while loop:

// Example
for i in 1..6 {
    println!("i is: {}", i);
}

// Break and Continue
// Example
for i in 1..=10 {
    if i == 3 {
        continue; // skip 3
    }
    if i == 5 {
        break; // stop before printing 5
    }
    println!("i is: {}", i);
}

