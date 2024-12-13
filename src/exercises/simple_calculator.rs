use std::io::{self};

pub fn simple_calculator() {
    loop {
        println!("Input operation: (+, -, *, /)");

        let mut operation = String::new();
        io::stdin().read_line(&mut operation).expect("Failed to read line");
        let operation = operation.trim();

        if operation == "exit" {break;}

        println!("Input first value: (int)");

        let mut first_string = String::new();
        io::stdin().read_line(&mut first_string).expect("Failed to read line");
        let FIRST_INT: i32 = match first_string.trim().parse() {
            Ok(val) => val,
            Err(_) => {
                println!("Invalid input for the first value. Please enter an integer.");
                continue;
            }
        };

        if first_string == "exit" {break;}

        println!("Input second value: (int)");

        let mut second_string = String::new();
        io::stdin().read_line(&mut second_string).expect("Failed to read line");
        let SECOND_INT: i32 = match second_string.trim().parse() {
            Ok(val) => val,
            Err(_) => {
                println!("Invalid input for the second value. Please enter an integer.");
                continue;
            }
        };

        
        if second_string == "exit" {break;}

        match operation {
            "+" => println!("Result: {}", FIRST_INT + SECOND_INT),
            "-" => println!("Result: {}", FIRST_INT - SECOND_INT),
            "*" => println!("Result: {}", FIRST_INT * SECOND_INT),
            "/" => {
                if SECOND_INT != 0 {
                    println!("Result: {}", FIRST_INT / SECOND_INT);
                } else {
                    println!("Error: Division by zero is not allowed.");
                }
            }
            _ => println!("Invalid operation! Please use one of (+, -, *, /)."),
        }
    }
}
