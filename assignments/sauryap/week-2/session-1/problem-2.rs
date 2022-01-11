fn main() {
    let word1:&str = "helodi";
    let word2:&str = "olehdi";

    println!("The strings {} and {} are {} anagrams", word1, word2, checkanagram(word1, word2));
}

fn checkanagram(word1:&str, word2:&str) -> bool {
    let mut c = 0;

    if word1.len() > word2.len() {
        return false
    }else {
        for i in word1.chars(){
            for j in word2.chars(){
                
                
                if i == j {
                    
                    c = c+1;
                }
            }
            
        }
    }

    if c == word1.len(){
        return true
    }else {
        return false
    }
}
