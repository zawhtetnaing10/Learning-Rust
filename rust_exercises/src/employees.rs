use std::{collections::HashMap, io};

pub fn add_employees_to_departments() {
    let mut employees_map: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!("Please select an action.");
        println!("1. Add Employee");
        println!("2. Quit");

        // Get Choice
        let mut choice = String::new();
        let choice_result = io::stdin().read_line(&mut choice);
        let Ok(_) = choice_result else {
            println!("There was an error reading the input");
            return;
        };

        if choice.trim() == String::from("1") {
            println!("Please enter department name");

            let mut department = String::new();
            let department_result = io::stdin().read_line(&mut department);
            let Ok(_) = department_result else {
                return;
            };

            // Department Clone
            let department_name_clone = department.clone();
            let department_name_to_print = department_name_clone.trim();

            // Add department key to employees map
            let employees_list = employees_map.entry(department).or_insert(Vec::new());

            // Ask for employee name
            println!("Please enter employee name");

            let mut employee_name = String::new();
            let read_employee = io::stdin().read_line(&mut employee_name);
            let Ok(_) = read_employee else {
                return;
            };

            // Employee Clone
            let employee_name_clone = employee_name.clone();
            let employee_name_to_print = employee_name_clone.trim();

            // Add to employee list
            employees_list.push(employee_name);

            println!("Added {employee_name_to_print} to {department_name_to_print}");
        } else if choice.trim() == String::from("2") {
            println!("Terminating session!");

            println!("The list of departments and their employees");

            // Print out all the departments and employees
            for (department, employee_list) in employees_map {
                println!("-{department}");

                for index in 0..employee_list.len() {
                    let position = index + 1;
                    let employee = &employee_list[index];
                    println!("{position}. {employee}");
                }
            }

            // Break out of the loop
            break;
        } else {
            // Invalid Choice. Continue the Loop.
            println!("Your choice is invalid.");
            continue;
        }
    }
}
