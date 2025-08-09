mod utils;

use rand::random_range;
use std::cmp::Ordering;

fn main() {
    let random_number: u8 = random_range(0..100);
    println!("Cheat: {random_number}");

    loop {
        let guess_number: u8 = loop {
            // Parse is Polimorphic so we can create a Struct from it's string representation
            let guess = utils::input("Try guess: ");
            match guess.parse() {
                // only `loop`` can return values... so guess_number is returned after the break
                Ok(guess_number) => break guess_number,
                Err(_) => println!("Numero invalido"),
            }
        };

        match guess_number.cmp(&random_number) {
            Ordering::Less => println!("Try a greater one"),
            Ordering::Equal => {
                println!("You guess it!");
                break;
            }
            Ordering::Greater => println!("Try a lower one"),
        }
    }
}
