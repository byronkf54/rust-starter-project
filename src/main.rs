use serde::{Deserialize, Serialize};
use std::io;
mod file_extensions;

#[derive(Debug, Serialize, Deserialize)]
struct Exercise {
    name: String,
    sets: Vec<Sets>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Sets {
    reps: u32,
    weight: f32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Workout {
    date: String,
    exercises: Vec<Exercise>,
}

fn main() {
    let mut workouts: Vec<Workout> = file_extensions::load_file("workouts.json");

    loop {
        println!("\n1. Add Workout");
        println!("2. View Workouts");
        println!("3. Exit");
        print!("Choose an option: ");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        match choice.trim() {
            "1" => add_workout(&mut workouts),
            "2" => view_workouts(&workouts),
            "3" => break,
            _ => println!("Invalid option"),
        }
    }
}

fn add_workout(workouts: &mut Vec<Workout>) {
    println!("Enter workout date (YYYY-MM-DD) (T - For Today): ");
    let mut date = String::new();
    io::stdin()
        .read_line(&mut date)
        .expect("Failed to read line");

    if date.trim() == "T" {
        let now = chrono::Local::now();
        date = now.format("%Y-%m-%d").to_string();
    }

    let mut exercises = Vec::new();

    loop {
        println!("Enter exercise name (or 'done' to finish): ");
        let mut name = String::new();
        io::stdin()
            .read_line(&mut name)
            .expect("Failed to read line");
        let name = name.trim();

        if name == "done" {
            break;
        }

        let set_count = read_input("Enter sets: ");
        let mut sets = Vec::new();
        for i in 0..set_count {
            let reps = read_input(&format!("Enter reps for set {}: ", i + 1));
            let weight = read_float_input(&format!("Enter weight (kg) for set {}: ", i + 1));
            sets.push(Sets { reps, weight });
        }

        exercises.push(Exercise {
            name: name.to_string(),
            sets,
        });
    }

    workouts.push(Workout {
        date: date.trim().to_string(),
        exercises,
    });

    save_workouts(workouts);
}

fn save_workouts(workouts: &Vec<Workout>) {
    let workout_str = serde_json::to_string(workouts).unwrap();
    file_extensions::save_file("workouts.json", workout_str);
}

fn view_workouts(workouts: &Vec<Workout>) {
    for workout in workouts {
        println!("Date: {}", workout.date);
        for exercise in &workout.exercises {
            println!("  {}: {} sets", exercise.name, exercise.sets.len());
            for set in &exercise.sets {
                println!("    reps: {}, weight: {}", set.reps, set.weight);
            }
        }
        println!("");
    }
}

fn read_input(prompt: &str) -> u32 {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        match input.trim().parse::<u32>() {
            Ok(num) => return num,
            Err(_) => println!("Invalid input. Please enter a number."),
        }
    }
}

fn read_float_input(prompt: &str) -> f32 {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        match input.trim().parse::<f32>() {
            Ok(num) => return num,
            Err(_) => println!("Invalid input. Please enter a number."),
        }
    }
}
