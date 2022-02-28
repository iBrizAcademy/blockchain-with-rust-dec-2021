struct Question {
    question: String,
    option1: String,
    option2: String,
    option3: String,
    option4: String,
    correct_option: u32,
}
fn main() {
    let question: [Question; 5] = [
        Question {
            question: "How are you?".to_string(),
            option1: "a".to_string(),
            option2: "b".to_string(),
            option3: "c".to_string(),
            option4: "d".to_string(),
            correct_option: 2,
        },
        Question {
            question: "who are you?".to_string(),
            option1: "a".to_string(),
            option2: "b".to_string(),
            option3: "c".to_string(),
            option4: "d".to_string(),
            correct_option: 3,
        },
        Question {
            question: "Hello hello hello?".to_string(),
            option1: "a".to_string(),
            option2: "b".to_string(),
            option3: "c".to_string(),
            option4: "d".to_string(),
            correct_option: 1,
        },
        Question {
            question: "Question 4?".to_string(),
            option1: "a".to_string(),
            option2: "b".to_string(),
            option3: "c".to_string(),
            option4: "d".to_string(),
            correct_option: 2,
        },
        Question {
            question: "Question 5".to_string(),
            option1: "a".to_string(),
            option2: "b".to_string(),
            option3: "c".to_string(),
            option4: "d".to_string(),
            correct_option: 4,
        },
    ];
    let mut score: i32 = 0;
    for i in 0..question.len() {
        let q = &question[i];
        println!(
            "{}\nOption A: {}\nOption B: {}\nOption C: {}\nOption D: {}",
            q.question, q.option1, q.option2, q.option3, q.option4
        );
        let mut line = String::new();
        println!("Choose an option:");
        std::io::stdin().read_line(&mut line).unwrap();
        let trimmed = line.trim();
        match trimmed.parse::<u32>() {
            Ok(i) => {
                if i == q.correct_option {
                    score = score + 1;
                    println!("Correct Answer\n");
                } else {
                    println!("Incorrect\n");
                }
            }
            Err(..) => println!("this was not an number: {}", trimmed),
        };
    }
    println!("You answered {} question correctly", score);
}
