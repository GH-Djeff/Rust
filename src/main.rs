use std::io;
//use rand::Rng;
use std::cmp::Ordering;

fn main() {
    //println!("Guess the number!");
    println!("Please type the secret number!");

    //let secret_number = rand::thread_rng().gen_range(1..3);
    let secret_number: u32 = 7;

    //println!("The secret number is: {}", secret_number);

    //println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type the secret number!");

    println!("You guessed: {}", guess);

    // --snip--

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You are in!"),
    }
}
