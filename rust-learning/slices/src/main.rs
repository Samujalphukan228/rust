// fn main() {
//     let mut arr: Vec<i32> = vec![1,2,3,4,5,6,7,8,9,10,11];
//     println!("{:?}",arr)
// }


fn main() {
    let mut vec:Vec<&str> = Vec::new();
    vec.extend(["a","b","c"]);
    println!("{:?}", vec);
    
    let slc: &[&str] = &vec[..2];
    println!("{:?}", slc);
}
