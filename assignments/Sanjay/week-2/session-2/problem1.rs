//pass ownership
struct Roc {
    ind: u16,
    mineral: String,
}

fn main() {
    let rock1 = Roc{
        ind: 2,
        mineral: String::from("Calc"),
    };
    let returoc = refre(rock1);
    println!("{},{}",returoc.ind, returoc.mineral);

} 


fn refre(rock:Roc)-> Roc{
    rock

}