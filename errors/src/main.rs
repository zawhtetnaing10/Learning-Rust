use core::{num, panic};
use std::{fs::File, io::ErrorKind};

fn main() {
    //panic!("crash and burn");

    // let nums = vec![1, 2, 3, 4];
    // let result = &nums[99];

    // Handle error with match
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        // Matching different errors
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create("hello.txxt") {
                Ok(file) => file,
                Err(e) => panic!("Problem creating the file {e:?}"),
            },
            _ => {
                panic!("Problem opening the file: {err:?}")
            }
        },
    };
}
