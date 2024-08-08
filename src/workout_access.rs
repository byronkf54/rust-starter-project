use std::io::{self};

use crate::extensions::input_extensions::*;
use crate::file_extensions::*;
use crate::structs::workouts::exercise::*;
use crate::structs::workouts::sets::*;
use crate::structs::workouts::workout::*;

pub fn add_workout(workouts: &Vec<Workout>) -> Vec<Workout> {
    let date = read_date();

    let exercises = get_exercise_details();

    workouts.push(Workout {
        date: date.trim().to_string(),
        exercises,
    });

    save_workouts(&workouts);

    workouts
}

pub fn view_workouts(workouts: &Vec<Workout>) {
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

fn save_workouts(workouts: &Vec<Workout>) {
    let workout_str = match serde_json::to_string(workouts) {
        Ok(ws) => ws,
        Err(e) => {
            eprintln!("Failed to serialize workouts: {}", e);
            return; // Explicitly return `()` here
        }
    };

    if let Err(e) = save_file("workouts.json", workout_str) {
        eprintln!("Failed to save workouts: {}", e);
        return; // Explicitly return `()` here as well
    }

    // Function implicitly returns `()` at the end
}

fn get_exercise_details() -> Vec<Exercise> {
    let mut exercises = Vec::new();
    loop {
        let name = read_exercise_name();

        if name == "done" || name == "" {
            break;
        }

        let set_count = read_input("Enter sets: ");
        let sets = read_sets_reps(set_count);

        exercises.push(Exercise { name, sets });
    }

    return exercises;
}

fn read_date() -> String {
    println!("Enter workout date (YYYY-MM-DD) (T - For Today): ");
    let mut date = String::new();
    io::stdin()
        .read_line(&mut date)
        .expect("Failed to read line");

    if date.trim() == "T" {
        let now = chrono::Local::now();
        date = now.format("%Y-%m-%d").to_string();
    }

    return date;
}

fn read_exercise_name() -> String {
    println!("Enter exercise name (or 'done' to finish): ");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    return name.trim().to_owned();
}

fn read_sets_reps(set_count: u32) -> Vec<Sets> {
    let mut sets = Vec::new();

    for i in 0..set_count {
        let reps = read_input(&format!("Enter reps for set {}: ", i + 1));
        let weight = read_float_input(&format!("Enter weight (kg) for set {}: ", i + 1));
        sets.push(Sets { reps, weight });
    }

    return sets;
}
