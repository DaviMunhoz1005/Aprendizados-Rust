use std::{cmp::Ordering, io};
use rand::Rng;

fn main() {

    println!("Guessing the number!");
    println!("Input your guess:");

    let mut guess = String::new();
    let mut attempts = 0;
    let secret_number : u32 = rand::thread_rng().gen_range(1..=100);

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read Line");

    let number : u32 = guess.trim().parse().expect("This is not a positive number!");

    match number.cmp(&secret_number) {
        Ordering::Less => {
            println!("Too Small!");
            attempts += 1;
        },
        Ordering::Equal => {
            println!("You Win!")
        },
        Ordering::Greater => {
            println!("Too Big!");
            attempts += 1;
        }
    }

    println!("Number digited {}", number);
    println!("Secret Number is {}", secret_number);
    println!("Attemps: {}", attempts);
}
