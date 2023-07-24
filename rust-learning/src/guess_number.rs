use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn guess_number(){
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");
    loop {
        println!("Please type a number");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to readline");

        let guess: u32 = guess.trim().parse().expect("Please type a number!");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
        println!("You guessed {guess}");
        println!("**************************");
    }
}