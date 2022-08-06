use std::fmt::Display;


/// Represents an actor in a show
pub struct Actor {
    name: String,
    birth_year: i64
}

impl Actor {
    pub fn new(name: String, birth_year: i64)  -> Actor{
        Actor {
            name,
            birth_year
        }
    }
}

impl Display for Actor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - born {}", self.name, self.birth_year)
    }
}