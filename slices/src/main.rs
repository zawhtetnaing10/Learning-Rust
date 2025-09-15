fn main() {
    let first_word = get_first_word("Hello World");
    // s1.clear(); // error
    println!("{first_word}");

    let a = [1, 2, 3, 4, 5];
    let first_slice = &a[0..3];
    println!("The slice is {:?}", first_slice);
}

fn get_first_word(text: &str) -> &str {
    let input_bytes = text.as_bytes();
    for (index, &item) in input_bytes.iter().enumerate() {
        if item == b' ' {
            return &text[0..index];
        }
    }

    return &text[..];
}
