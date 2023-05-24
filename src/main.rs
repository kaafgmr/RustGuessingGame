use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() 
{
    println!("Guess the number (it is between 1 and 100)");
 
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Input your guess");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number)
    {
        Ordering::Less => println!("Too small, try again"),
        Ordering::Greater => println!("Too big, try again"),
        Ordering::Equal => println!("You win"),
    }
}
