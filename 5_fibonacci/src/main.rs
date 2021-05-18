use std::io;

fn get_n() -> usize {

    println!("Enter a number n to find the nth fibonacci number for");
    let mut n = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read input.");

    let n: usize = n.trim().parse().expect("Not a number.");
    n
}

fn get_fibonacci_arr(n: usize) -> Vec<usize> {

    let mut arr: Vec<usize> = vec![0; n];
    arr[1] = 1;
    for index in 2..arr.len() {
        arr[index] = arr[index-1] + arr[index-2];
    }
    arr
}

fn print_arr(arr: &Vec<usize>) {
    for i in 0..arr.len() { 
        if i == arr.len() - 1 { print!("{}", arr[i]); }
        else { print!("{}, ", arr[i]); }
    } 
}

fn main() {
    let n: usize = get_n();
    let arr = get_fibonacci_arr(n);
    print!("Fibonacci series ({}) = ", n);
    print_arr(&arr);
    println!();
    println!("{}th fibonacci number is {}", n, arr[arr.len() - 1]);
}