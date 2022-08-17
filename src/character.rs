use std::fmt::Display;

use serde::{Serialize, Deserialize};


/// Represents a character in a show
#[derive(Serialize, Deserialize)]
pub struct Character {
    pub name: String,
    role: String,
    gender: String
}

impl Character {
    pub fn new(name: String, role: String, gender: String) -> Character {
        Character {
            name,
            role,
            gender
        }
    }
}

impl Display for Character {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({}) - {}", self.name, self.gender, self.role)
    }
}