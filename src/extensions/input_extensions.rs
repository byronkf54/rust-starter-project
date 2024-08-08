use std::io;

pub fn read_input(prompt: &str) -> u32 {
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

pub fn read_float_input(prompt: &str) -> f32 {
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