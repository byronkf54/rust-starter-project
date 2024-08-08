use serde::{Serialize, Deserialize};
use crate::structs::workouts::sets::Sets;

#[derive(Debug, Serialize, Deserialize)]
pub struct Exercise {
    pub name: String,
    pub sets: Vec<Sets>,
}