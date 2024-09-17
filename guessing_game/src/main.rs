use std::io;

fn main() {

    println!("Guessing the number!");
    println!("Input your guess:");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read Line");

    println!("Number digited {}", guess);
}
