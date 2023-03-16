use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {     
        println!("Please input your guess.");

        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        println!("You guessed: {guess}");


        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue
            },
        };

        let mut diff = guess - secret_number;
        diff = diff.abs();
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small! the difference is {}", diff),
            Ordering::Greater => println!("Too big! the difference is {}", diff),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }  
    }
    
    println!("The secret number is: {secret_number}");
}