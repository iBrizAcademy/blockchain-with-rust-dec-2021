use std::collections::HashMap;

fn main() {
    let mut hashmap_instance = HashMap::new();
    let str = hashmap_instance.entry("key").or_insert("default");
    println!("{}", str);
}