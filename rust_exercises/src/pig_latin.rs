pub fn pig_latin(text: &String) -> String {
    let mut first_char = 'c';

    let mut index = 0;

    let mut result = String::from("");

    // Leave the first char and push everything else
    for char in text.chars() {
        if index == 0 {
            first_char = char;
            // If first character is vowel, we'll also push the first char to the result
            if is_vowel(first_char) {
                result.push(first_char);
            }
        } else {
            result.push(char);
        }
        index += 1;
    }

    // Add Dash
    result.push('-');

    if is_vowel(first_char) {
        // If vowel, add 'h'
        result.push('h');
    } else {
        // If not vowel, add the first character back.
        result.push(first_char);
    }

    // Add ay at the end
    result.push_str("ay");

    result
}

fn is_vowel(char: char) -> bool {
    let vowels = ['a', 'e', 'i', 'o', 'u'];

    for vowel in vowels {
        if char == vowel {
            return true;
        }
    }

    false
}
