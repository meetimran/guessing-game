use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    println!("Guess the Number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number was: {secret_number}");

   
    loop{
        println!("Enter your guess:");
        let mut guess = String::new();

        io::stdin()
          .read_line(&mut guess)
          .expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        //println!("You guessed: {guess}");
        println!("You guessed {}", &guess);

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!"),
        }

    }
    
}
