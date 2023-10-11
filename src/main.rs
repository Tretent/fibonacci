use std::io;

fn main() {
    println!("Fibonacci sequence");
    println!("Enter an index for the sequence: ");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index = index.trim().parse::<u8>().unwrap();

    println!("The {index}-th element of the sequence is: {}", fibonacci(index))
}

fn fibonacci(n: u8) -> u16 {
    if n < 2 { n } else { fibonacci(n - 2) + fibonacci(n - 1) } as u16
}
