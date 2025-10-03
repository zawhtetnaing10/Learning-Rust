use std::iter;

fn main() {
    // Creating Strings
    // let mut s = String::new();

    // let data = "initial contents";
    // let data_string = data.to_string();

    // let literal_to_string = "initial contents".to_string();

    // let string_from = String::from("initial contents");

    // Updating Strings
    let mut s = String::from("foo");
    s.push_str("bar");
    s.push('b');
    s.push('a');
    s.push('z');
    println!("{s}");

    // Concat strings
    let s1 = String::from("Hello");
    let s2 = String::from("World");
    let s3 = s1 + &s2; // Note s1's ownership will be moved to s3

    // Format macro uses references. It doesn't take any ownership.
    // s1, s2 and s3 are all valid after the format! call.
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s4 = format!("{s1}-{s2}-{s3}");
    println!("{s4}");

    // Indexing into a string is a bad idea

    // Iterating over string
    let iterated_string = String::from("Зд");
    for c in iterated_string.chars() {
        println!("The character is {c}")
    }

    for b in iterated_string.bytes() {
        println!("The byte is {b}")
    }
}
