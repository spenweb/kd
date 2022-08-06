use std::fmt::Display;


/// Represents a character in a show
pub struct Character {
    name: String,
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