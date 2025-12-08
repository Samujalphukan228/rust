// #[tokio::main]
// async fn main() {
//     println!("Hello from Samujal. Quick confession: I'm learning Rust and if someone in the future sees this code, I want you to know that I hate myself for learning Rust.");
// }


// async  fn add(a: i32, b: i32) -> i32 {
//     a + b
// }

// #[tokio::main]
// async  fn main() {
//     let result: i32 = add(10,4).await;
//     println!("The sun of the two number is : {result}")

// }


use tokio::time::{sleep, Duration};

async fn task(name: &str) {
    println!("Starting {}", name);
    sleep(Duration::from_secs(1)).await;
    println!("Finished {}", name);
}

#[tokio::main]
async fn main() {
    tokio::join!(
        task("samujal"),
        task("Task B"),
    );
}
