// Borrowing and References

// What is a Reference?
// A reference lets you look at a value without owning it. You create a reference using the & symbol:
// Example
let a = String::from("Hello");
let b = &a;

println!("a = {}", a);
println!("b = {}", b);


// Mutable References
let name = String::from("samujal");
let nameref = &mut name;
nameref.push_str(" phukan")

println!("{}", nameref)