// // Vector

// // Creating a Vector
// let fruits = vec!["apple", "banana", "orange"];

// // Access Vector Elements
// // Example
// let fruits = vec!["apple", "banana", "orange"];
// println!("First fruit: {}", fruits[0]);

// // Change Vector Values
// // Example
// let mut fruits = vec!["apple", "banana", "orange"];
// fruits[0] = "grape";
// println!("New first fruit: {}", fruits[0]);

// // Add Elements to a Vector
// // Example
// let mut fruits = vec!["apple", "banana"];
// fruits.push("cherry");
// println!("{:?}", fruits); 

// // Remove Elements from a Vector
// // Example
// let mut fruits = vec!["apple", "banana", "cherry"];
// fruits.pop();
// println!("{:?}", fruits); 

// // Add or Remove Elements at a Specified Index
// // Example
// let mut fruits = vec!["banana", "orange"];
// fruits.insert(0, "apple");
// println!("{:?}", fruits);

// // Loop Through a Vector
// // Example
// let fruits = vec!["apple", "banana", "orange"];
// for fruit in &fruits {
//   println!("I like {}.", fruit);
// }


fn main() {
    let mut vec: Vec<&str> = Vec::new();
    vec.push("test");
    vec.push("dora");
    println!("before: {:?}, capacity = {}", vec, vec.capacity());

    vec.reserve();
    println!("after: {:?}, capacity: {}",vec, vec.capacity())
}
