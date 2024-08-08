use std::io;
mod file_extensions;
mod extensions {
    pub mod input_extensions;
}
mod workout_access;

mod structs {
    pub mod workouts {
        pub mod exercise;
        pub mod sets;
        pub mod workout;
    }
}

use file_extensions::*;
use workout_access::*;
use structs::workouts::workout::*;

fn main() {
    let mut workouts: Vec<Workout> = load_file("workouts.json");

    loop {
        println!("\n1. Add Workout");
        println!("2. View Workouts");
        println!("3. Exit");
        print!("Choose an option: ");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        
        if choice.trim() == "1" {
            add_workout(&mut workouts);
        } else if choice.trim() == "2" {
            view_workouts(&workouts);
        } else if choice.trim() == "3" || choice.trim() == "" {
            break;
        } else {
            println!("Invalid option");
        }
    }
}
