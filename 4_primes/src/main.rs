use std::io;

fn is_prime(x: usize) -> bool {
    let mut ct = 0;
    for i in 1..x+1 {
        if x % i == 0 { ct += 1; }
    }
    if ct == 2 { return true; }
    false
}

fn create_array(n: usize) -> Vec<usize>{
    let mut arr = vec![0; n];
    for i in 0..arr.len() { arr[i] = i; }
    arr
}

fn main() {

    println!("Enter a number: ");

    let mut n = String::new();
    
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read input.");

    let n: usize = n.trim().parse().expect("Expected a number.");

    let arr = create_array(n);

    for elem in arr.iter() {

        if is_prime(*elem) {
            println!("{} is prime.", elem)
        }
        else {
            println!("{} is not prime.", elem)
        }
    }

}
