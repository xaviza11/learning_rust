use std::io;

mod exercises;

fn main() {
    println!("Welcome to exercises index");

    loop {
        println!("\nPlease choose an exercise by number (or type 'exit' to quit): ");
        println!("1. hello_world");
        println!("2. simple_calculator");
        println!("3. car");
        println!("4. vending_machine");
        println!("5. exit");

        let mut exercise = String::new();

        io::stdin()
            .read_line(&mut exercise)
            .expect("Failed to read line");

        let exercise = exercise.trim();

        if exercise == "5" {
            println!("Exiting the program...");
            break;
        }

        println!("Running: {}", exercise);

        match exercise {
            "1" => exercises::hello_world::hello_world(),
            "2" => exercises::simple_calculator::simple_calculator(),
            "3" => exercises::car::main(),
            "4" => exercises::vending_machine::main(),
            _ => println!("Invalid choice. Please select a valid exercise."),
        }
    }
}
