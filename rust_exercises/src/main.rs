use pig_latin::pig_latin;

mod employees;
mod mean_median_mode;
mod pig_latin;

fn main() {
    let mut nums = vec![2, 4, 5, 5, 5, 6, 6, 7, 8, 8, 8, 8];
    nums.sort();

    // 1. Mean / Median / Mode
    // Mean
    let mean = mean_median_mode::find_mean(&nums);
    println!("The mean of the numbers is {mean}");

    // Median
    let median = mean_median_mode::find_median(&nums);
    println!("The median of the numbers is {median}");

    // Mode
    let mode = mean_median_mode::find_mode(&nums);
    println!("The mode of the numbers is {mode}");

    // 2. Pig Latin
    let string = String::from("first");
    let pig_latin_no_vowel = pig_latin::pig_latin(&string);
    println!("The print latin value is {pig_latin_no_vowel}");

    let string_vowel: String = String::from("apple");
    let pig_latin_vowel = pig_latin::pig_latin(&string_vowel);
    println!("The print latin value is {pig_latin_vowel}");

    // 3. Add Employees to Departments
    employees::add_employees_to_departments();
}
