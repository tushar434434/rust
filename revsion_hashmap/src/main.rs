use std::collections::HashMap;//using hashmap library

fn main() {
    let mut map = HashMap::new();//creating new hash

    map.insert("a", 1);//inserting the key value
    map.insert("b",3);
    if let Some(v) = map.get("b") {
        println!("{}", v);//it will print the value 
    }
     // Loop through all key-value pairs
    for (key, value) in &map {
        println!("{} => {}", key, value);
    }
}