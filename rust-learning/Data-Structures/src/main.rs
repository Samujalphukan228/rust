// Data Structure types
// Array
// Vector (Vec)
// Tuple
// HashMap

// Array
let fruits = ["apple", "banana", "orange"]
println!("Last fruit: {}", fruits[2]);

// Vector
let mut fruits = vec!["apple","banana"];
fruits.push("orrange");
println!("Last fruit: {}", fruits[2]);

// Tuples
let name = ("samual", 19, true);
println!("Name: {}", name.0);
println!("Age: {}", name.1);
println!("Is active: {}", name.2);

// HashMaps
// Import 
use std::collections::HashMap;

fn main() {
    let mut  capitalCities = HashMap::new();
    capitalCities.insert("france", "paris")
    capitalCities.insert("Japan", "Tokyo");

    println!(" Capital of japan is {}", capitalCities["Japan"])
}