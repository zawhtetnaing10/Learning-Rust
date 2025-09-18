use std::fmt::format;

fn main() {
    let home = IpAddrKind::V4(String::from("127.0.0.1"));

    let m = Message::Write(String::from("Hello"));
    m.call();

    // Matching enums with value
    let coin = Coin::Quarter(UsState::Alaska);
    //println!("The cent value is {}", coin.value_in_cents());

    let mut count = 0;
    if let Coin::Quarter(state) = &coin {
        print!("State quarter from {state:?}")
    } else {
        count += 1;
    }

    if let Some(joke) = coin.describe_state_quarter() {
        println!("The joke for the quarter is {joke}")
    }

    // Matching the value of Option
    let five: Option<i32> = Some(5);
    let none_number: Option<i32> = None;
    let six = plus_one(five);
    let none = plus_one(none_number);

    if let Some(x) = six {
        println!("The value of plus one is {x}")
    }
}

enum IpAddrKind {
    V4(String),
    V6(String),
}

enum Message {
    Quit,
    Movie { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {}
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
        }
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl Coin {
    fn value_in_cents(&self) -> u8 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {state:?}");
                25
            }
        }
    }

    fn describe_state_quarter(&self) -> Option<String> {
        // if let Coin::Quarter(state) = self {
        //     if state.existed_in(1900) {
        //         Some(format!("{state:?} is pretty old, for America"))
        //     } else {
        //         Some(format!("{state:?} is relatively new."))
        //     }
        // } else {
        //     None
        // }

        // let state = if let Coin::Quarter(state) = self {
        //     state
        // } else {
        //     return None;
        // };

        // if state.existed_in(1900) {
        //     Some(format!("{state:?} is pretty old, for America"))
        // } else {
        //     Some(format!("{state:?} is relatively new"))
        // }

        let Coin::Quarter(state) = self else {
            return None;
        };

        if state.existed_in(1900) {
            Some(format!("{state:?} is pretty old, for America"))
        } else {
            Some(format!("{state:?} is relatively new"))
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(value) => Some(value + 1),
        None => None,
    }
}
