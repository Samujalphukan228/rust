// fn name(name: &str) {
//     println!("{name}");
// }

// fn main() {
//     let n: String = String::from("maaha") ;
//     name(&n);
// }

// fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
//     if a.len() > b.len() {
//         a
//     } else {
//         b
//     }
// }

// fn main() {
//     let x: &str = "ariana";
//     let y: &str = "aisha";

//     let res: &str = longest(x, y);
//     println!("Longest is : {res}")
// }


// fn first<'a>(x: &'a str) -> &'a str {
//     x
// }

// fn main() {
//     let name: String = String::from("samujal");
//     let res: &str = first(&name);
//     println!("name is {res}")
// }


fn bigger<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let a: &str = "samujal";
    let b: &str = "phukan";

    let res: &str = bigger(a,b);
    println!("Bigger = {res}")
}