use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn guessing_game() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed {guess}");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please guess a number!");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("Equal, you win!");
                break;
            }
        }
    }
}

fn run_guessing_game() {
    loop {
        println!("List of the games:");
        println!("1. Guessing Game");
        println!("0. Exit");

        let mut option: String = String::new();
        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read option");
        let option: u32 = match option.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number");
                continue;
            }
        };
        match option {
            0 => {
                "Closing the app... goodbye!";
                process::exit(0);
            }
            1 => guessing_game(),
            _ => println!("Game doesn't exist yet, todo"),
        }
    }
}
