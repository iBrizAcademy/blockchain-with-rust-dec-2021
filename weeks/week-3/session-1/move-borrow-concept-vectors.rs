fn main() {
    let mut vec = vec![0, 0, 0];
    borrow(&mut vec);

    println!("{:?}", vec);
}

fn borrow(vec: &mut Vec<i32>) {
    println!("{:?}",vec );
    vec.push(1);
}