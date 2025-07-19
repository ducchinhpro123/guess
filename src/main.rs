use rand::Rng;
use std::io::stdin;
const MAX_ATTEMPTS: usize = 3;
const LOW: usize = 1;
const HIGH: usize = 100;

fn main() {
    let mut answer = rand::rng().random_range(LOW..=HIGH);
    let mut attempt = 0;
    let mut shutdown = false;
    println!(
        "I created a random number from {LOW} to {HIGH}, you have {MAX_ATTEMPTS} attemps to guess"
    );

    loop {
        if attempt < MAX_ATTEMPTS {
            let mut buf = String::new();
            match stdin().read_line(&mut buf) {
                Ok(_) => match buf.trim().parse::<usize>() {
                    Ok(num) => {
                        if num == answer {
                            println!("You guessed it right, congratulation!");
                            break;
                        } else {
                            if num < answer {
                                println!("Your guess is too small!");
                            } else {
                                println!("Your guess is too large!");
                            }
                        }
                    }
                    Err(err) => {
                        println!("parse error: {err}");
                    }
                },
                Err(err) => {
                    println!("error: {err}");
                }
            }
            attempt += 1;
        } else {
            if shutdown {
                break;
            }
            // You are ran out of attemps, what's next?
            println!("Sorry, you ran out of attempts! The answer was {answer}");
            println!("Try again? [Y/n]");
            loop {
                let mut try_again = String::new();
                match stdin().read_line(&mut try_again) {
                    Ok(_) => {
                        let user_answer = try_again.trim();
                        if user_answer.is_empty() // User hit enter
                                                   || user_answer == "Y" || user_answer == "y" {
                            answer = rand::rng().random_range(LOW..=HIGH);
                            println!(
                                "I created a random number from {LOW} to {HIGH}, you have {MAX_ATTEMPTS} attemps to guess"
                            );
                            attempt = 0;
                            break;
                        }
                        if user_answer == "n".to_string() {
                            println!("Goodbye! have a good day");
                            shutdown = true;
                            break;
                        } else {
                            println!("Sorry, I don't understand. Please input Y/y or n");
                        }
                    },
                    Err(e) => {
                        panic!("Sorry, something went wrong: {e}");
                    }
                }
            }
        }
    }
}
