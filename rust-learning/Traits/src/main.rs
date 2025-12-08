// SIMPLE TRAIT

// trait Hello {
//     fn say_hi(&self);
// }


// IMPLEMENT

// trait Hello {
//     fn say_hi(&self);
// }

// struct User {
//     name: String,
//     age: i32,
// }

// impl Hello for User {
//     fn say_hi(&self) {
//         println!("Hi, I am {} and I am {} years old", self.name, self.age);
//     }
// }

// fn main() {
//     let u = User {
//         name: "samujal".to_string(),
//         age: 19,
//     };

//     u.say_hi();
// }


                                           // PRACTICE 2

// trait Math {
//     fn add(&self, a: i32, b: i32) -> i32;
//     fn mul(&self, a: i32, b: i32) -> i32;
//     fn sub(&self, a: i32, b: i32) -> i32;
//     fn div(&self, a: i32, b: i32) -> i32;
// }

// struct Calculator;

// impl Math for Calculator {
//     fn add(&self, a: i32, b: i32) -> i32 {
//         a+b
//     }
//     fn mul(&self, a: i32, b: i32) -> i32 {
//         a*b
//     }
//     fn sub(&self, a: i32, b: i32) -> i32 {
//         a-b
//     }
//     fn div(&self, a: i32, b: i32) -> i32 {
//         a/b
//     }
// }

// fn main() {
//     let calc: Calculator = Calculator;
//     println!("{}", calc.add(10, 5));
//     println!("{}", calc.mul(10, 5));
//     println!("{}", calc.sub(10, 5));
//     println!("{}", calc.div(10, 5));
// }


                                             // PRACTICE 3


trait Animal {
    fn sound(&self);
}

struct Dog;
struct Cat;
struct Lion;
struct Snake;

impl Animal for Dog {
    fn sound(&self) {
        println!("Woof!");
    }
}

impl Animal for Cat {
    fn sound(&self) {
        println!("Meow!");
    }
}

impl Animal for Lion {
    fn sound(&self) {
        println!("Roar!");
    }
}

impl Animal for Snake {
    fn sound(&self) {
        println!("Hiss!");
    }
}

fn main() {
    let d: Dog = Dog;
    let c: Cat = Cat;
    let l: Lion = Lion;
    let s: Snake = Snake;

    d.sound();
    c.sound();
    l.sound();
    s.sound();
}
