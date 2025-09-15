fn main() {
    // Stack Memory
    // let s = "hello";
    // println!("{}", s);

    // Heap Memory
    // let mut s = String::from("hello");
    // s.push_str(", world!");
    // println!("{s}");

    // Cloning
    // let s1 = String::from("hello");
    // let s2 = s1.clone();
    // println!("s1 : {s1}, s2: {s2}");

    // Ownership and Functions
    let s1 = String::from("hello");
    take_ownership(s1);

    //println!("Is s1 still valid? {s1}"); // s1 lost ownership

    let s2: String = String::from("second string");

    let s3 = take_ownership_and_give_back(s2);
    //println!("Is s2 still valid? {s2}") // s2 lost ownership
    println!("{s3}");

    let s4 = gives_ownership();
    println!("{s4}");
}

fn take_ownership(s: String) {
    println!("The value is {s}");
}

fn gives_ownership() -> String {
    return String::from("I give you ownership");
}

fn take_ownership_and_give_back(s: String) -> String {
    return s;
}
