use simple_user_input::get_input;

fn main() {
    println!("Welcome to anagram checker!!");

    let input1: String = get_input("Please type the first string:");
    let input2: String = get_input("Please type the second string:");

    let result: bool = is_anagram(input1.clone(),input2.clone());

    if result{
        println!("{} and {} are anagrams.",input1,input2);
    }
    else{
        println!("{} and {} are not anagrams.",input1,input2);
    }

}

fn is_anagram(inp_string1: String, inp_string2: String) -> bool{

    let mut anagram_flag: bool =true;

    let inp_string1_upper: String = inp_string1.to_uppercase();
    let inp_string2_upper: String = inp_string2.to_uppercase();

    let mut inp_string1_vec: Vec<char> = inp_string1_upper.chars().collect();
    let mut inp_string2_vec: Vec<char> = inp_string2_upper.chars().collect();

    inp_string1_vec.sort();
    inp_string2_vec.sort();
    
    let string1_len=inp_string1_vec.len();
    let string2_len=inp_string2_vec.len();
     
    let alpha_index_1 = find_alphabet_index(inp_string1_vec.clone());
    let alpha_index_2 = find_alphabet_index(inp_string2_vec.clone());

    let num_alphabets_1=string1_len-(alpha_index_1);
    let num_alphabets_2=string2_len-(alpha_index_2);

    if num_alphabets_1!=num_alphabets_2{
        anagram_flag=false;
    }

    else{

        if alpha_index_1>=alpha_index_2{

            let alpha_index_diff = alpha_index_1-alpha_index_2;
    
            for i in alpha_index_2..string2_len{
    
                let c1=inp_string1_vec.get(i+alpha_index_diff).unwrap();
                let c2=inp_string2_vec.get(i).unwrap();
                
                if c1!=c2{
                    anagram_flag=false;
                    break;
                }
            }
        }
        else{
            let alpha_index_diff = alpha_index_2-alpha_index_1;
    
            for i in alpha_index_1..string1_len{
    
                let c1=inp_string1_vec.get(i).unwrap();
                let c2=inp_string2_vec.get(i+alpha_index_diff).unwrap();
                
                if c1!=c2{
                    anagram_flag=false;
                    break;
                }
            }
        }
    }  
   
    anagram_flag
}

fn find_alphabet_index(string_vec: Vec<char>) -> usize{

    let mut alpha_index: usize = 0;

    for c in &string_vec{
        if c==&' '{
            alpha_index=alpha_index+1;
        }

        else{
            break;
        }
    }    
    alpha_index
}

mod simple_user_input {
    use std::io;
    pub fn get_input(prompt: &str) -> String{
        println!("{}",prompt);
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_goes_into_input_above) => {},
            Err(_no_updates_is_fine) => {},
        }
        input.trim().to_string()
    }
}