use std::fmt::Display;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents a character in a show
#[derive(Serialize, Deserialize)]
pub struct Character {
    pub id: String,
    pub name: String,
    pub role: String,
    pub gender: String,
}

impl Character {
    pub fn new(name: String, role: String, gender: String) -> Character {
        Character {
            id: Uuid::new_v4().to_string(),
            name,
            role,
            gender,
        }
    }
}

impl Display for Character {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({}) - {}", self.name, self.gender, self.role)
    }
}
