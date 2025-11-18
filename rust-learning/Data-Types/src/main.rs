// Basic Variable Examples
let my_num = 5;         // integer (default i32)
let my_double = 5.99;   // float (default f64)
let my_letter = 'D';    // character
let my_bool = true;     // boolean
let my_text = "Hello"; // string slice (&str)


// Same Examples With Explicit Types
let my_num: i32 = 5;          // integer
let my_double: f64 = 5.99;    // floating-point number
let my_letter: char = 'D';    // character
let my_bool: bool = true;     // boolean
let my_text: &str = "Hello"; // string slice

// -------------------------
// Data Types
// -------------------------

// Numbers

// Integer (i32)
// i32 is used for whole numbers (positive or negative), without decimals.
// Example:
let age: i32 = 25;
println!("Age is: {}", age);

// Floating Point (f64)
// f64 is used for numbers with decimals.
// Example:
let price: f64 = 19.99;
println!("Price is: ${}", price);

// Characters (char)
// char stores a single Unicode character and must use single quotes.
// Example:
let myGrade: char = 'B';
println!("{}", myGrade);

// Strings (&str)
// &str stores text as a string slice, using double quotes.
// Example:
let name: &str = "John";
println!("Hello, {}!", name);

// Booleans (bool)
// bool stores true or false.
// Example:
let is_logged_in: bool = true;
println!("User logged in? {}", is_logged_in);


//Combining Data Types
//You can mix different types in the same program:
// Example
let name = "John";
let age = 28;
let is_admin = false;
println!("Name: {}", name);
println!("Age: {}", age);
println!("Admin: {}", is_admin);