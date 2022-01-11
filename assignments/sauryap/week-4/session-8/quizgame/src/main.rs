use std::vec;

struct QuizQuestion {
    question:String,
    options: [String; 4],
    answer:String,

}

fn main() {
    println!("*******************\n\tQuiz Game\n*******************");

    let mut questions: Vec<QuizQuestion> = Vec::new();
    questions.push(QuizQuestion{question:"What is capital of Bangladesh?".to_string(),options:["1. Dhaka".to_string(),"2. Khulna".to_string(),"3. Rangpur".to_string(), "4. Kathmandu".to_string()],answer: "1. Dhaka".to_string()});
    questions.push(QuizQuestion{question:"Who is the prime minister of Sweden?".to_string(),options:["1. Magdalena Andersson".to_string(),"2. KP Oli".to_string(),"3. Prachanda".to_string(), "4. Jarcinda".to_string()],answer: "1. Magdalena Andersson".to_string()});
    questions.push(QuizQuestion{question:"What is the name of the road linking Kathmandu and Pokhara?".to_string(),options:["1. Prithvi Highway".to_string(),"2. Tribhuwan Highway".to_string(),"3. BP Highway".to_string(), "Araniko Highway".to_string()],answer: "1. Prithvi Highway".to_string()});
    questions.push(QuizQuestion{question:"What is the capital of Nepal?".to_string(),options:["1. Delhi".to_string(),"2. Kathmandu".to_string(),"3. Dhaka".to_string(), "4. Tokyo".to_string()],answer: "2. Kathmandu".to_string()});
    questions.push(QuizQuestion{question:"Who invented zero?".to_string(),options:["1. Mohit Chauhan".to_string(),"2. Balaji Shrinivasan".to_string(),"3. Arya Bhatt".to_string(), "4. Naval".to_string()],answer: "3. Arya Bhatt".to_string()});
    questions.push(QuizQuestion{question:"What is the chemical formula for water?".to_string(),options:["1. CH2".to_string(),"2. N20".to_string(),"3. CHO".to_string(), "4. H20".to_string()],answer: "4. H20".to_string()});
    questions.push(QuizQuestion{question:"Which represents a constellation?".to_string(),options:["1. Orion".to_string(),"2. Andromeda".to_string(),"3. Perseus".to_string(), "4. All".to_string()],answer: "4. All".to_string()});



    let mut j = 0;
    let mut k = 0;
    loop {
        
        let len = questions.len();

        println!("\n{}",questions[j].question);
        println!("Your options:");
        println!("\t{}",questions[j].options[k]);
        println!("\t{}",questions[j].options[k+1]);
        println!("\t{}",questions[j].options[k+2]);
        println!("\t{}",questions[j].options[k+3]);
        println!("Your answer(Please use the numbers besides the options): ");

        let mut input_user = String::new();
        let user_input = std::io::stdin().read_line(&mut input_user).unwrap();
        let trimmed = input_user.trim();
        match trimmed.parse::<usize>() {
            Ok(i) => {
                if questions[j].options[i-1] == questions[j].answer {
                    println!("You win. Correct answer is {}", questions[j].options[i-1]);

                }
                else{
                    println!("You lose. Correct answer is {}", questions[j].answer);
                    break;
                }
                
                },
            Err(..) => println!("this was not an integer: {}", trimmed),
        };
        j = j + 1;
        if j == len {
            break;
        }

    }
    println!("Congrats. Your total score is: {}", j);



}