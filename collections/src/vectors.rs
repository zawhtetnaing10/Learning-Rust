pub fn test_vectors() {
    // Vectors
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    let third = &v[2];
    println!("The third element is {third}");

    if let Some(hundred) = &v.get(100) {
        println!("The hundred element exists and it's {hundred}")
    } else {
        println!("The vector doesn't have 100 elements");
    }

    // Borrow Checker
    let mut numbers_vec = vec![1, 2, 3];

    let first_num: &i32 = &numbers_vec[0];
    print!("The first element is {first_num}");

    numbers_vec.push(6);

    for i in &numbers_vec {
        println!("The current number is {i}");
    }

    // Mutate the items inside using mutable reference
    for i in &mut numbers_vec {
        *i += 50;
    }
    // Read the items again
    for i in &numbers_vec {
        println!("The current number is {i}");
    }

    // Using enums to store differenct types in Vectors
    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Float(10.12),
        SpreadSheetCell::Text(String::from("blue")),
    ];

    for i in &row {
        match i {
            SpreadSheetCell::Int(i) => println!("The cell has a number {i}"),
            SpreadSheetCell::Float(f) => println!("The cell has a float {f}"),
            SpreadSheetCell::Text(s) => println!("The cell has a string {s}"),
        }
    }
}

enum SpreadSheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
