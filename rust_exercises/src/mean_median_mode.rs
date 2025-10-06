use std::collections::HashMap;

pub fn find_mean(numbers: &Vec<i32>) -> f64 {
    let length = numbers.len() as f64;
    let mut sum = 0;

    for num in numbers {
        sum += num;
    }

    round_to_decimal_place((sum as f64) / length, 2)
}

pub fn find_mode(numbers: &Vec<i32>) -> i32 {
    let mut hashmap: HashMap<i32, i32> = HashMap::new();

    for num in numbers {
        let count = hashmap.entry(*num).or_insert(0);
        *count += 1
    }

    // Find the key of the max value
    let mut max = 0;
    let mut result = 0;
    for (key, value) in hashmap {
        if value > max {
            result = key;
            max = value;
        }
    }

    return result;
}

pub fn find_median(numbers: &Vec<i32>) -> f64 {
    if numbers.len() % 2 == 0 {
        // Even
        let first_median_index = numbers.len() / 2;
        let second_median_index = (numbers.len() / 2) + 1;

        let first_median_number = numbers[first_median_index - 1] as f64;
        let second_median_number = numbers[second_median_index - 1] as f64;

        (first_median_number + second_median_number) / 2.0
    } else {
        // Odd
        let median_position = (numbers.len() + 1) / 2;
        numbers[median_position - 1] as f64
    }
}

fn round_to_decimal_place(input: f64, decimal_places: u32) -> f64 {
    let scale_factor = (10i32.pow(decimal_places)) as f64;

    (input * scale_factor).round() / scale_factor
}
