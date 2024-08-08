use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Sets {
    pub reps: u32,
    pub weight: f32,
}