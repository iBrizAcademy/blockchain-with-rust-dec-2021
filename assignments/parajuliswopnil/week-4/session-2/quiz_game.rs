use std::io;
use rand::Rng;

struct Questions{
    questions: Vec<Vec<String>>,
}

fn build_questions(questions: Vec<String>)-> Questions{
    let questions = Questions{
        questions: [questions].to_vec(),
    };
    return questions;
}

fn add_questions(question_instance: &mut Questions, question: Vec<String>){
    question_instance.questions.push(question);
}

fn main(){
    let mut questions: Questions = build_questions([String::from("questions"), String::from("options comma seperated"), String::from("answer")].to_vec());
    println!("{:?}", questions.questions);
    let random_number_vector: Vec<u32>;


    add_questions(&mut questions, [String::from("What is the biggest country in the world?"), String::from("China,Russia,India,Nepal"), String::from("Russia")].to_vec());
    add_questions(&mut questions, [String::from("What is the tallest mountain in the world?"), String::from("K2,Everest,Manaslu,Machhapuchre"), String::from("Everest")].to_vec());
    add_questions(&mut questions, [String::from("What is the biggest continent in the world?"), String::from("Asia,Europe,Australia,Africa"), String::from("Asia")].to_vec());
    add_questions(&mut questions, [String::from("What is the longest river in the world?"), String::from("Nile,Koshi,Gangas,Bisnumati"), String::from("Nile")].to_vec());
    add_questions(&mut questions, [String::from("What is the official language of Nepal?"), String::from("Hindi,English,Russian,Nepali"), String::from("Nepali")].to_vec());

    println!("{:?}", questions.questions);

    // displaying game
    println!("Welcome to the quiz game. To enter the game please press 1. To exit enter 0 ");
    let mut game_playing = String::new();
    io::stdin().read_line(&mut game_playing).expect("Couldnot read.");
    let game = game_playing.trim();
    if game == String::from("1"){
        let random_number_list = get_random_numbers(1, questions.questions.len() as u32);
        println!("{:?}", random_number_list);
        for i in 1..questions.questions.len(){
            // let mut random_number: u32 = rand::thread_rng().gen_range(1, questions.questions.len() as u32);
            // println!("{}", random_number);
            println!("{}", questions.questions[random_number_list[i-1] as usize][0]);
            let options = get_options(&questions.questions[random_number_list[i-1] as usize][1]);
            println!("The options are: \n {} {} {} {}", options[0], options[1], options[2], options[3]);
            println!("To answer just enter the number as per the position of the options.");
            let mut choose_option = String::new();
            io::stdin().read_line(&mut choose_option).expect("Couldnot get the answer.");
            let option: u32 = choose_option.trim().parse().expect("Could not parse");
            let index: u32 = option - 1;
            if options[index as usize] == questions.questions[random_number_list[i-1] as usize][2]{
                println!("You are correct");
            }
            else {
                println!("The answer you gave is wrong the correct answer is {}", questions.questions[random_number_list[i-1] as usize][2]);
            }
        }
    }

}

fn get_random_numbers(lower_limit: u32, upper_limit: u32) -> Vec<u32>{
    println!("{} {}", lower_limit, upper_limit);
    let mut random_number_list = Vec::new(); 
    while random_number_list.len() < (upper_limit-1) as usize{
        let mut random_number: u32 = rand::thread_rng().gen_range(lower_limit, upper_limit);
        if !random_number_list.contains(&random_number){
            println!("{}", random_number);
            random_number_list.push(random_number);
        }
    }
    return random_number_list;
}

fn get_options(options_string: &String)-> Vec<&str>{
    let v: Vec<&str> = options_string.split(',').collect();
    println!("{:?}", v);
    return v;

}
