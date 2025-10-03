use std::collections::{HashMap, btree_map::Values};

fn main() {
    // Creating hashmap
    let mut scores = HashMap::new();

    scores.insert(String::from("Chelsea"), 35);
    scores.insert(String::from("Man Utd"), 26);

    // Getting a value out of hash map
    let chelsea_score = scores.get("Chelsea").copied().unwrap_or(0);
    println!("Chelsea has {chelsea_score} points");

    // Iterating over hash map
    for (key, value) in &scores {
        println!("{key} {value}");
    }

    // Hashmap Ownership
    let field_name = String::from("favorite color");
    let field_value = String::from("red");

    let mut color_map = HashMap::new();
    color_map.insert(field_name, field_value);
    // field_name and field_value ownerships are moved into color_map.
    // Both field_name and field_value variables are invalid here.

    // Updating Hashmap
    let mut game_reviews = HashMap::new();

    // Normal Insert. Ovewrites data
    game_reviews.insert("Expedition 33", 8);
    game_reviews.insert("Expedition 33", 10);
    let game_score = game_reviews.get("Expedition 33").copied().unwrap_or(0);
    println!("The review score for expedition 33 is {game_score}");

    // Insert Or. Will insert only if the key doesn't exist
    game_reviews.entry("Sekiro").or_insert(10);
    game_reviews.entry("Sekiro").or_insert(8);
    println!("The game reviews are {game_reviews:?}");

    // Updating data based on old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("The word count {map:?}");
}
