use std::fmt::Display;

/// Represents a TV series or movie
pub struct Show {
    name: String,
    release_year: i16
}

impl Show {
    pub fn new(name: String, release_year: i16) -> Show {
        Show {
            name,
            release_year
        }
    }
}

impl Display for Show {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}", self.name, self.release_year)
    }
}