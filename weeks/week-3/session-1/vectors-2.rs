fn main() {
    let mut vector_elements = vec![1, 2, 3];
    println!("{:?}", vector_elements);

    vector_elements.push(4);

    println!("{:?}", vector_elements);
    println!("Length: {}", vector_elements.len());
}