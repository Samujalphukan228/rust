// fn add(a:i32, b:i32) -> i32 {
//     a+b
// }

// fn main() {
//     let ans:i32 = add(10, 20);
//     println!("sum of the nums are {ans}")
// }

// fn is_even(n:i32) -> bool {
//     if n % 2 ==0 {
//         true
//     } else {
//         false
//     }
// }


// fn main() {
//     println!("{}", is_even(1) )
// }


// fn process(num: i32) -> (i32, i32) {
//     (num * 2, num * 3)
// }

// fn main() {
//     let (a, b): (i32, i32) = process(5);
//     println!("Double = {}, Triple = {}", a, b);
// }


fn print_message(msg: &String) {
    println!("Message is: {}", msg);
}

fn main() {
    let text: String = String::from("Hello world");
    print_message(&text); 
}