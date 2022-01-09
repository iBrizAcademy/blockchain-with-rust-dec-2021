use rand::Rng;
use std::io;

fn take_input() -> String {
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    guess.trim().to_string()
}

fn check_guess_num(number: u32, guess_input: &str) {
    // loop {
    //     let guess = String::from(guess_input);
    //     let guess: u32 = match guess.trim().parse() {
    //         Ok(num) => num,
    //         Err(_) => continue,
    //     };

    //     if guess < number {
    //         println!("Your guess is too low");
    //     } else if guess > number {
    //         println!("Your guess is too high")
    //     } else {
    //         println!("Correct");
    //         break;
    //     }
    // }

    let guess = String::from(guess_input);
    let guess = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };
    if guess < number {
        println!("Your guess is too low");
    } else if guess > number {
        println!("Your guess is too high")
    } else {
        println!("Correct");
    }
}

fn check_range(num: u32, guess_input: String) {
    let guess = guess_input;
    let arr: Vec<_> = guess.trim().split("..").collect();
    let first: u32 = arr[0].parse().unwrap();
    let last: u32 = arr[1].parse().unwrap();

    let mut numbers: Vec<u32> = Vec::new();

    for number in first..=last {
        numbers.push(number);
    }

    if numbers.contains(&num) {
        println!("Correct: {}", num);
    } else {
        println!("Failed the number was {}", num);
    }
}

fn multivalued_number(num: u32, guess_input: String) {
    let guess = guess_input;
    let numbers: Vec<_> = guess.split_whitespace().collect();

    for i in numbers {
        if i.parse::<u32>().unwrap() == num {
            println!("won {}", num);
        }
    }
    println!("Loss {}", num);
}

fn main() {
    let number = rand::thread_rng().gen_range(1..=100);

    println!(
        "Enter the guess like single number (25), multiple number(25 50), range number(20..40)"
    );

    let guess = take_input();
    if guess.len() == 2 {
        check_guess_num(number, guess.as_str())
    } else if guess.len() == 6 {
        check_range(number, guess)
    } else {
        multivalued_number(number, guess);
    }
}
