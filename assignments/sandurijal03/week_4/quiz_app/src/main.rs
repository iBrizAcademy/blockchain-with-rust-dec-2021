use serde::Deserialize;
use serde_json;
use std::fs::File;
use std::io::stdin;

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
struct Question {
    question: String,
    optionA: String,
    optionB: String,
    optionC: String,
    optionD: String,
    answer: String,
}

fn show_header() {
    println!("");
    println!("---------------- QUIZ GAME ------------------");
    println!("----------Select The proper option-----------");
    println!("");
}

fn main() {
    show_header();
    let mut score = 0;
    let mut question_index = 0;
    let file = File::open("questions.json").unwrap();
    let questions: Vec<Question> = serde_json::from_reader(file).expect("error");

    loop {
        let mut guess: String = String::new();
        println!("{}", &questions[question_index].question);
        let a = &questions[question_index].optionA;
        let b = &questions[question_index].optionB;
        let c = &questions[question_index].optionC;
        let d = &questions[question_index].optionD;
        let answer = &questions[question_index].answer;
        let statement = format!("1. {}, 2. {}, 3. {}, 4. {}", a, b, c, d);
        println!("{}", statement);
        stdin()
            .read_line(&mut guess)
            .expect("Please enter correct value");

        if guess.trim() == answer.to_string() {
            score += 1;
        } else {
            println!("Sorry wrong guess");
        }
        question_index += 1;
        if question_index >= questions.len() {
            question_index = 0;
            guess = String::new();
            println!("Quiz finished, results: {}/{}", score, questions.len());
            println!("Would you like to quit? y or n");
            stdin()
                .read_line(&mut guess)
                .expect("Please enter the correct value.");

            match guess.trim().to_ascii_lowercase().as_str() {
                "y" | "bye" => break,
                _ => println!("you choose to continue"),
            }
            score = 0;
        }
    }
}
