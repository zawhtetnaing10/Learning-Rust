fn main() {
    // Reference
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of {s1} is {len}");

    // Mutable references
    let mut s2 = String::from("Changable");
    change(&mut s2);
    println!("The new string is {s2}");

    // Mutable reference after
    // normal references go out of scope
    let mut s3 = String::from("Mutable String");
    let r1 = &s3;
    let r2 = &s3;

    println!("The references are {r1} and {r2}");

    let r3 = &mut s3;
    println!("The mutable reference is {r3}");

    let mut s4 = String::from("String no.4");
    let r4 = &mut s4;
    r4.push_str(" Mutated");
    println!("The mutable reference is {r4}");

    let r5 = &s4;
    let r6 = &s4;
    println!("The new references are {r5} and {r6}");
}

fn calculate_length(s: &String) -> usize {
    return s.len();
}

fn change(s: &mut String) {
    s.push_str(" String");
}
