use rand::Rng;
use std::io;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("Please input your guess:");

    let mut guess = String::new();
    println!("{} number!", secret_number);

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => 400,
    };

    println!(
        "You've guessed : {}, but you still have other 3 options",
        guess
    );
    println!("Select any one option");
    println!("1. Number is greater than: {}", guess);
    println!("2. Number is smaller than: {}", guess);
    println!("3. Number is equal to: {}", guess);

    let mut option = String::new();
    io::stdin()
        .read_line(&mut option)
        .expect("Failed to read line");
    let option: u32 = match option.trim().parse() {
        Ok(num) => num,
        Err(_) => 400,
    };
    loop {
        match option {
            1 => {
                if guess < secret_number {
                    println!("you guessed correct");
                    break;
                } else {
                    println!("wrong guess!");
                    break;
                }
            }
            2 => {
                if guess > secret_number {
                    println!("you guessed correct");
                    break;
                } else {
                    println!("wrong guess! 2");
                    break;
                }
            }
            3 => {
                if guess == secret_number {
                    println!("you guessed correct");
                    break;
                } else {
                    println!("wrong guess!");
                    break;
                }
            }
            _ => {
                println!("bad choice");
                break;
            }
        }
    }
}
