use rand::Rng;
use std::io;

fn main() {
    println!("Quiz Game");

    let mut score: u32 = 0;
    let mut question_index: usize = 0;

    let questions = vec![
        "Control unit of a digital computer is often called the",
        "Fifth generation digital computer will be",
        "In distributed computer system",
        "Real time computing is possible due to",
        "Modern computers compared to earlier computers are",
    ];
    let answer_choices = vec![
        vec!["1. ICS", "2. Clock", "3. Nerve centre", "4. All of these"],
        vec![
            "1. Artificial intelligence",
            "2. Extremely low cost",
            "3. Very expensive",
            "4. Versatility",
        ],
        vec![
            "1. The task is distributed throughout the system",
            "2. There are many computers and terminals",
            "3. The task is executed by a number of processors",
            "4. All of these",
        ],
        vec![
            "1. Accuracy",
            "2. Storage capability",
            "3. High speed",
            "4. Versatility",
        ],
        vec![
            "1. Faster and smaller",
            "2. Larger and stronger",
            "3. Less reliable",
            "4. Faster and larger",
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
        let num = rand::thread_rng().gen_range(0..5);
        println!("{}", num);
        let mut answer: String = String::new();
        println!("{}", questions[num]);
        println!("{:?}", answer_choices[num]);
        io::stdin()
            .read_line(&mut answer)
            .expect("Please enter a correct value");
        if answer.trim().to_lowercase() == answers[num].to_lowercase() {
            println!("Well done, this is the correct answer\n");
            score += 1;
        } else {
            println!("Sorry, try again")
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
