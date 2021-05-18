use std::io;

fn main() {

    println!("Enter a number:");
    let mut n = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read input.");

    let n: usize = n.trim().parse().expect("Enter a number pls.");
    let mut arr = vec![0; n];

    for i in 0..arr.len() {
        arr[i] = i * 2;
        println!("{}", arr[i]);
    }
}