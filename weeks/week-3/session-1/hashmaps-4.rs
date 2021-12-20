use std::collections::HashMap;

fn main() {
    let mut hashmap_instance = HashMap::new();
    hashmap_instance.insert("key1", "value1");
    hashmap_instance.insert("key2", "value2");
    hashmap_instance.insert("key3", "value3");


    println!("{:?}\n", hashmap_instance);

    for (key, value)  in hashmap_instance.iter() {
        println!("{}, {}", key, value);
    }
}