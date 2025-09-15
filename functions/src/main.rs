fn main() {
    println!("Hello, world!");

    another_function(5);

    print_label_measurement(5, 'h');

    let y = {
        let x = 5;
        x + 1
    };
    println!("The value of y is: {y}");

    let five = five();
    println!("The result of five function: {five}");

    let result = plus_one(6);
    println!("The result is: {result}");

    let n = 3;
    let fab_result = fibonacci(n);
    println!("The fabonacci result is: {fab_result}")
}

fn five() -> i32 {
    5
}

fn plus_one(num: i32) -> i32 {
    num + 1
}

fn print_label_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn another_function(x: i32) {
    println!("The value of x is : {x}");
}

fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }

    if n == 1 {
        return 1;
    }

    return fibonacci(n - 1) + fibonacci(n - 2);
}
