use rand::Rng;
fn main() {
    println!("1: Enter a number.\n2: Enter multiple numbers.\n3: Enter number between a range.\n4: Higher than a number.\n5: Lower than a number.");
    let mut choosed_option = String::new();
    println!("Choose an option :");
    std::io::stdin().read_line(&mut choosed_option).unwrap();
    if choosed_option.trim() == "1" {
        let mut line = String::new();
        println!("Enter your number :");
        std::io::stdin().read_line(&mut line).unwrap();
        let trimmed = line.trim();
        match trimmed.parse::<u32>() {
            Ok(i) => find_match(i),
            Err(..) => println!("this was not an number: {}", trimmed),
        };
    } else if choosed_option.trim() == "2" {
        let mut nums: Vec<i32> = Vec::new();
        let mut add = true;
        while add{
            let mut num = String::new();
            println!("Enter your number :");
            std::io::stdin().read_line(&mut num).unwrap();
            nums.push(num.trim().parse::<i32>().unwrap());
            let mut c = String::new();
            println!("1 to add another number 2 to continue with game:");
            std::io::stdin().read_line(&mut c).unwrap();
            if c.trim() == "2" {
                add = false;
            }
        }
        find_match_between_multiple(nums);
    } else if choosed_option.trim() == "3" {
        let mut first_num = String::new();
        let mut second_num = String::new();
        println!("Enter first number :");
        std::io::stdin().read_line(&mut first_num).unwrap();
        println!("Enter second number :");
        std::io::stdin().read_line(&mut second_num).unwrap();
        find_match_between_range(first_num, second_num);
    } else if choosed_option.trim() == "4" {
        let mut num = String::new();
        println!("Enter your number :");
        std::io::stdin().read_line(&mut num).unwrap();
        find_match_higher_than(num);
    } else if choosed_option.trim() == "5" {
        let mut num = String::new();
        println!("Enter your number :");
        std::io::stdin().read_line(&mut num).unwrap();
        find_match_lower_than(num);
    }
}
fn find_match_between_multiple(nums: Vec<i32>){
    let rand_num = rand::thread_rng().gen_range(0..100);
    println!("Numbers you choosed:");
    println!("{:?}", nums);
    println!("Number that was predicted: {}", rand_num);
    if nums.contains(&i32::from(rand_num)) {
        println!("You win");
    } else {
        println!("You lose")
    }
}
fn find_match(num: u32) {
    let rand_num = rand::thread_rng().gen_range(0..100);
    println!("Your number: {}, predicted number: {}", num, rand_num);
    if rand_num == num {
        println!("You win");
    } else {
        println!("You lose")
    }
}

fn find_match_between_range(first_num: String, second_num: String) {
    let rand_num = rand::thread_rng().gen_range(0..100);
    println!(
        "Your number was between: {} and {}, predicted number was: {}",
        first_num, second_num, rand_num
    );
    if rand_num >= first_num.trim().parse().unwrap()
        && rand_num <= second_num.trim().parse().unwrap()
    {
        println!("You win");
    } else {
        println!("You lose")
    }
}

fn find_match_higher_than(num: String) {
    let rand_num = rand::thread_rng().gen_range(0..100);
    println!(
        "Your number was higher than: {}, predicted number: {}",
        num, rand_num
    );
    if rand_num > num.trim().parse().unwrap() {
        println!("You win");
    } else {
        println!("You lose")
    }
}

fn find_match_lower_than(num: String) {
    let rand_num = rand::thread_rng().gen_range(0..100);
    println!(
        "Your number was lower than: {}, predicted number: {}",
        num, rand_num
    );
    if rand_num < num.trim().parse().unwrap() {
        println!("You win");
    } else {
        println!("You lose")
    }
}
