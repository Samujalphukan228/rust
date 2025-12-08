struct User {
    name: String,
    age: u32,
}


fn main() {
    let n: User = User {
        name:"Samujal".to_string(),
        age: 19,
    };

    println!("Name: {name}, Age: {age}", name = n.name, age = n.age);
}



