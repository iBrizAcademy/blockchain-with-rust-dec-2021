use std::io;
extern crate rand;
use rand::Rng;

fn main(){
    let mut choose_game = String::new();

    let mut lower_number = String::new();
    let mut upper_number = String::new();


    println!("Welcome to the game. You can either choose to play dice or guessing game. Enter 1 for dice game Enter 2 for guessing game");
    io::stdin().read_line(&mut choose_game).expect("Failed to receive the number.");
    let game: i32 = choose_game.trim().parse().expect("Failed to parse the number");
    
    if game ==0 || game > 2{
        panic!("Invalid game number. Choose any one of 1 or 2.");
    }
    if game == 1{
        println!("Welcome to the dice game. To begin enter two numbers between 1 and 100.");
        println!("Enter the lower number");
        io::stdin().read_line(&mut lower_number).expect("Failed to receive the first number.");
        println!("Enter the upper number");
        io::stdin().read_line(&mut upper_number).expect("Failed to receive the upper number.");
        let trimmed_lower_number = lower_number.trim();
        let trimmed_upper_number = upper_number.trim();

        let lower: u32 = trimmed_lower_number.parse().expect("couldnot parse");
        let upper: u32 = trimmed_upper_number.parse().expect("couldnot parse");

        if lower > upper {
            panic!("Lower cannot be greater than upper.");
        }   
        if lower > 100 || upper > 100 || lower < 1 || upper < 1 || lower == upper{
            panic!("Upper or lower numbers cannot be less than 1 or greater than 100 or be equal.")
        }
        let random_number = get_random(); 
        
        if random_number < upper && lower < random_number{
            println!("You have won!! The winning number is {}", random_number)
        }
        else{
            println!("Sorry you have lost. The winning number was {}", random_number)
        } 
    }
    else if game == 2{
        println!("Welcome to the guessing game. To begin enter your luck number between 1 and 100.");
        println!("Enter the number");
        io::stdin().read_line(&mut lower_number).expect("Failed to receive the first number.");
       
        let trimmed_lower_number = lower_number.trim();

        let lower: u32 = trimmed_lower_number.parse().expect("couldnot parse");

        if lower > 100 || lower < 1 {
            panic!("Number cannot be greater than 100 or less than 1")
        }
        let random_number = get_random();
        if random_number == lower{
            println!("Congratulations you won! The winning number is {}", random_number)
        }
        if random_number != lower{
            println!("Sorry you have lost. The winning number was {}", random_number);
        }
    }    
}

fn get_random() -> u32{
    let mut random_number = rand::thread_rng().gen_range(1,101);
    return random_number
}