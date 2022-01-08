// Writing a simple Quiz game in RUST programming language

use rand::Rng;
use std::io;

fn main() {
    println!("**************************************************************************");
    println!("\t\t WELCOME TO QUIZ GAME");
    println!("\tplease enter the number between 1-4 to select correct option");
    println!("**************************************************************************\n");

    let mut score: u32 = 0;
    let mut question_index: usize = 0;
    let mut choices = Vec::<usize>::new();

    while choices.len() != 5 {
        let num = rand::thread_rng().gen_range(0..5);
        if choices.contains(&num) {
            continue;
        } else {
            choices.push(num);
        }
    }

    let questions = vec![
        "Control unit of a digital computer is often called the",
        "Fifth generation digital computer will be",
        "In distributed computer system",
        "Real time computing is possible due to",
        "Modern computers compared to earlier computers are",
    ];
    let answer_choices = vec![
        vec!["ICS", "Clock", "Nerve centre", "All of these"],
        vec![
            "Artificial intelligence",
            "Extremely low cost",
            "Very expensive",
            "Versatility",
        ],
        vec![
            "The task is distributed throughout the system",
            "There are many computers and terminals",
            "The task is executed by a number of processors",
            "All of these",
        ],
        vec![
            "Accuracy",
            "Storage capability",
            "High speed",
            "Versatility",
        ],
        vec![
            "Faster and smaller",
            "Larger and stronger",
            "Less reliable",
            "Faster and larger",
        ],
    ];
    let answers = vec![
        "Nerve centre",
        "Artificial intelligence",
        "The task is distributed throughout the system",
        "High speed",
        "Faster and smaller",
    ];

    loop {
        let num = choices[question_index];
        let mut answer: String = String::new();
        println!("{}", questions[num]);
        println!("{:?}", answer_choices[num]);
        io::stdin()
            .read_line(&mut answer)
            .expect("Please enter a correct value");

        let my_num: usize = answer
            .trim()
            .parse()
            .expect("please give me correct string number!");

        if answer_choices[num][my_num - 1].to_lowercase() == answers[num].to_lowercase() {
            println!("CORRECT!!\n");
            score += 1;
        } else {
            println!("WRONG!!")
        }
        question_index += 1;
        if question_index >= questions.len() {
            question_index = 0;
            answer = String::new();

            println!("Quiz finished, results: {}/{}", score, questions.len());
            println!("Would you like to quit?(yes or y)");
            io::stdin()
                .read_line(&mut answer)
                .expect("Please enter a correct value");
            match answer.trim().to_lowercase().as_str() {
                "yes" | "y" => break,
                _ => println!("You choose to continue"),
            }
            score = 0;
        }
    }
}
