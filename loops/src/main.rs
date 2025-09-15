fn main() {
    // loop {
    //     println!("again!");
    // }

    // let mut counter = 0;

    // let result = loop {
    //     counter += 1;

    //     if counter == 10 {
    //         break counter * 2;
    //     }
    // };

    // loop
    let mut count = 0;
    'counting_up: loop {
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }

            remaining -= 1
        }

        count += 1;
    }

    println!("End count = {count}");

    // while
    let mut num = 3;

    while num != 0 {
        println!("{num}!");
        num -= 1;
    }

    println!("LIFTOFF!!!");

    // for
    let a = [1, 2, 3, 4, 5];
    for element in a {
        println!("{element}!");
    }

    for rangeElement in (1..4).rev() {
        println!("{rangeElement}");
    }
    println!("LIFTOFF!!!")
}
