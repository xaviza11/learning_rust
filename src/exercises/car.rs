use std::io::{self};

pub struct Car {
    name: String,
    horsepower: u32,
    color: String,
}

impl Car {
    fn new(name: &str, horsepower: u32, color: &str) -> Car {
        Car {
            name: String::from(name),
            horsepower,
            color: String::from(color),
        }
    }

    fn start_engine(&self) {
        println!("The engine of {} is started, {} horsepower", self.name, self.horsepower);
    }

    fn change_color(&mut self, new_color: &str) {
        self.color = String::from(new_color);
    }
}

pub fn main() {
    let mut my_car = Car::new("Ferrari", 500, "Red");
    
    my_car.start_engine(); 
    println!("The car is currently: {}", my_car.color);
    
    loop {
        println!("Enter a new color for the car (or 'exit' to quit):");
        
        let mut new_color = String::new();
        io::stdin().read_line(&mut new_color).expect("Failed to read line");
        let new_color = new_color.trim();

        if new_color == "exit" {
            println!("Exiting the program...");
            break;
        }

        my_car.change_color(new_color);
        println!("The car is now: {}", my_car.color);
    }
}
