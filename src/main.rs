use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    let mut tries: u32 = 5;

    loop {
        if tries < 1
        {
            println!("You ran out of tries!");
            break;
        }

        println!("Please input your guess.");

        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
            
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("You guessed: {guess}");
    
        match guess.cmp(&secret_number) {
            Ordering::Less => {
                tries-=1;
                println!("Too small! Number of tries left:{}", tries);
            }
            Ordering::Greater => {
                tries-=1;
                println!("Too big! Number of tries left:{}", tries);
            }
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

    }
}
