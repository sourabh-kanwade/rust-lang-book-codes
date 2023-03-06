use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("The Guessing Game!");
    println!("Please guess the number ?");
    let secret_number = rand::thread_rng().gen_range(1..101);
    // println!("Secret Number is {secret_number}");
    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to ReadLine");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}", "Please Input a Number!".cyan());
                continue;
            }
        };
        println!("You Guessed {guess}");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too Small!".red()),
            Ordering::Greater => println!("{}", "Too Big!".red()),
            Ordering::Equal => {
                println!("{}", "You Win".green());
                break;
            }
        }
    }
}
