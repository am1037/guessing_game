use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");
    let mut guess = String::new();
    let secret_number = rand::thread_rng().gen_range(1..=100);

    io::stdin().read_line(&mut guess)
               .expect("Faild to road line");
    let guess: i32 = guess.trim().parse().expect("Please type a number!");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small! the difference is {}", guess-secret_number),
        Ordering::Greater => println!("Too big! the difference is {}", guess-secret_number),
        Ordering::Equal => println!("You win!"),
    }

    println!("You guessed: {guess}");
    println!("The secret number is: {secret_number}");
}