use std::{cmp::Ordering, io};
use rand::Rng;

fn main() {

    println!("Guessing the number!");

    let mut attempts = 0;
    let secret_number : u32 = rand::thread_rng().gen_range(1..=100);

    'game : loop {

        let mut guess = String::new();

        println!("Input your guess:");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read Line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        attempts += 1;

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too Small!");
            },
            Ordering::Equal => {
                println!("You Win!");
                println!("Attempts: {}", attempts);
                break 'game;
            },
            Ordering::Greater => {
                println!("Too Big!");
            }
        }
    };
}
