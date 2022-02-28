// pass struct variable as reference and dereferencing to modify value of object
struct Roc {
    ind: u16,
    mineral: String,
}

fn main() {
    let mut rock = Roc {
        ind: 2,
        mineral: String::from("Calcu"),
    };
    println!("Before {},{}",rock.ind,rock.mineral);
    refre(&mut rock);
    println!("After {},{}",rock.ind,rock.mineral);
}

fn refre(rock: &mut Roc){
    rock.ind =4;
    rock.mineral = String::from("Kalk")
}



