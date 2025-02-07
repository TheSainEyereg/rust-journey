use rand::Rng;
use std::{cmp::Ordering, io};
fn main() {
    let secret_number = rand::rng().random_range(1..=100);

    loop {
        println!("Enter yor guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading stdin");

        // Shadowing
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("NaN");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too high"),
            Ordering::Equal => {
                println!("yay!!!");
                break;
            }
        }
    }
}
