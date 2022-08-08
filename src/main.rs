use std::{io, process};

fn main() {
    println!("Enter triangle bottom size! (number)");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let triangle_bottom_size: usize = input.trim_end().parse().unwrap_or_else(|_| {
        eprintln!("Wrong number format!");
        process::exit(1);
    });

    for n in 1..=triangle_bottom_size {
        println!("{}", "*".repeat(n));
    }

    println!("Here's yours triangle!");
}
