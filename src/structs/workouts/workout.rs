use serde::{Serialize, Deserialize};
use crate::structs::workouts::exercise::Exercise;

#[derive(Debug, Serialize, Deserialize)]
pub struct Workout {
    pub date: String,
    pub exercises: Vec<Exercise>,
}