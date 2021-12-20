fn main() {
    let mut vector_elements = vec![1, 2, 3];
    println!("{:?}", vector_elements);

    vector_elements.push(4);

    println!("{:?}", vector_elements);

    println!("using iter()");
    for element  in vector_elements.iter() {
        println!("{}", element);
    }

    println!("using reference to vector collection");
    for element  in &vector_elements {
        println!("{}", element);
    }

}