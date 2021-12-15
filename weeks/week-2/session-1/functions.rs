fn main() {
    print_labeled_measurement(5, 'h');
}

fn print_labeled_measurement(value: i32, unit_label: char){
    println!("This measurement is : {}{}", value, unit_label);
}