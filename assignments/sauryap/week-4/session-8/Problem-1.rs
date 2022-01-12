//random guessing game

#[macro_use] extern crate random_number;



fn main() {
    

    loop {
        let mut play = String::new();
        println!("\n***********How do you want to play***************\n 1. Guess a number\n 2. Guess a range of number\n 3. Guess multiple numbers\n 4. Number higher than\n 5. Number lower than");
        println!("Enter your choice: ");
        println!("***********");
        let mut choice = String::new();
        let user_choice = std::io::stdin().read_line(&mut choice).expect("Failed to read from the input");
        let trimmed_choice = choice.trim();
        let mut res:bool = false;
        let n: u8 = random!(1, 100);

        
        match trimmed_choice.parse::<u8>() {
            Ok(c) => {
                match c {
                    1 => res = guess_number(n),
                    2 => res = guess_range(n),
                    3 => res = guess_multiple(n),
                    4 => res = guess_higher_than(n),
                    5 => res = guess_lower_than(n),
                    _=> println!("Wrong Choice"),
                }

                if res == true {
                    println!("Your guess is correct. The number is {}. You win", n);
                }
                else if res == false && c <= 5 {
                    println!("Your guess is incorrect. The number is {}. You lose", n);
                }
            }
            Err(..) => println!("this was not an integer: {}", trimmed_choice),
        }


        println!("Do you want to exit the game? (Y/N)");
        let play_choice = std::io::stdin().read_line(&mut play).expect("Failed to read from the input");
        
        let trimmed_play = play.trim();
        match trimmed_play.parse::<String>() {
            Ok(p) => {
                match p.as_str() {
                    "Y" => { break; },
                    "y" => { break; },
                    _=> {continue;},


                }
                
            }
            Err(..) => println!("this was not a string: {}", trimmed_play),
        }

    }

    
    

}

fn guess_number(n : u8) -> bool {
    let mut guess = String::new();
    println!("Please type your guess: ");
    //random number generator

    let user_guess = std::io::stdin().read_line(&mut guess).expect("Failed to read from the input");
    let trimmed = guess.trim();
    match trimmed.parse::<u8>() {
        Ok(i) => {
            if i == n {
                return true;
            }
            else {
                return false;
            }
        }
        Err(..) => {return false},
    };
}

fn guess_range(n : u8) -> bool {
    let mut guess1 = String::new();
    let mut guess2 = String::new();
    println!("Please enter the range of the guess: ");
    //random number generator
    
    println!("Please enter the first number: ");
    let user_guess1 = std::io::stdin().read_line(&mut guess1).expect("Failed to read from the input");
    println!("Please enter the second number: ");
    let user_guess2 = std::io::stdin().read_line(&mut guess2).expect("Failed to read from the input");
    let trimmed1 = guess1.trim();
    let trimmed2 = guess2.trim();
    match trimmed1.parse::<u8>() {
        Ok(i) => {
            match trimmed2.parse::<u8>() {
                    Ok(j) => {
                    
                        if (n >= i && n <= j) || (n <= i && n >= j)  {
                            return true;
                        }
                        else {
                            return false;
                        }
                        
                    }
                    Err(..) => return false,
                        
            }
        }
        Err(..) => return false,
    }

}

fn guess_multiple(n : u8) -> bool {
    
    let mut number = String::new();
    // Vec for storing array values from user
    let mut vec = Vec::new();
    println!("Numbers you want to guess: ");
    
    let user_selected_number = std::io::stdin().read_line(&mut number).expect("Failed to read from the input");
    let trimmed = number.trim();
    match trimmed.parse::<u8>() {
        Ok(i) => {
            println!("Enter the numbers you want: ");
            for c in 1..i+1 {
                let mut guess = String::new();
                let user_guess = std::io::stdin().read_line(&mut guess).expect("Failed to read from the input");

                let trimmed1 = guess.trim();
                match trimmed1.parse::<u8>() {
                    Ok(i) => {
                        
                        vec.push(i);
                    }
                    Err(..) => println!("this was not an integer: {}", trimmed1),
                }
            }    
                        
        }
        Err(..) => {return false},
    }
    let len = vec.len();

    for x in 0..len {
        
        if n == vec[x] {
            return true;
        }
    }
    return false;

}

fn guess_higher_than (n : u8) -> bool {
    let mut guess = String::new();
    println!("Please enter the number: ");
    //random number generator
    let user_guess1 = std::io::stdin().read_line(&mut guess).expect("Failed to read from the input");
    let trimmed = guess.trim();
    match trimmed.parse::<u8>() {
        Ok(i) => {
            if n > i && i <= 100 {
                return true;
            }
            else {
                return false;
            }           
        }
        Err(..) => {return false}
        
    }
    
}

fn guess_lower_than (n : u8) -> bool {
    let mut guess = String::new();
    println!("Please enter the number: ");

    let user_guess1 = std::io::stdin().read_line(&mut guess).expect("Failed to read from the input");
    let trimmed = guess.trim();
    match trimmed.parse::<u8>() {
        Ok(i) => {
            if n < i {
                return true;
            }
            else {
                return false;
            }
                        
        }
        Err(..) => return false,
    }
}
