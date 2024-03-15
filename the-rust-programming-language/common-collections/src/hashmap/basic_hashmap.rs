use std::collections::HashMap;

pub fn initialize_hashmap() {
    // Creating a new hashmap
    let mut scores = HashMap::new();

    // Inserting new key-value pairs into the hashmap
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("Scores value after inserting {:?}", scores);

    // Accessing values in the hashmap
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("Score of Blue team is {:?}", score);

    // Iterating through hashmap using for loop
    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}
