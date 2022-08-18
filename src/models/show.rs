use std::fmt::Display;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::DisplayMoreInfo;

use super::character::Character;

/// Represents a TV series or movie
#[derive(Serialize, Deserialize)]
pub struct Show {
    pub id: String,
    pub name: String,
    pub release_year: i16,
    pub characters: Vec<Character>,
}

impl Show {
    pub fn new(name: String, release_year: i16) -> Show {
        Show {
            id: Uuid::new_v4().to_string(),
            name,
            release_year,
            characters: Vec::new(),
        }
    }

    pub fn get_id(&self) -> &String {
        &self.id
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn add_character(&mut self, character: Character) -> Result<&Character, &'static str> {
        // check if character exists
        if let Some(_) = self
            .characters
            .iter()
            .enumerate()
            .find(|c| c.1.name == character.name)
        {
            return Err("Character already exists");
        }
        self.characters.push(character);
        Ok(self.characters.last().unwrap())
    }

    pub fn update_character(&mut self, old_name: &str, character: Character) -> Result<&Character, &'static str> {
        // check if character exists
        if let Some(index) = self
            .characters
            .iter()
            .position(|c| c.name == old_name)
        {
            self.characters[index] = character;
            return Ok(self.characters.get(index).unwrap())
        }
        return Err("Character not found");
    }

    pub fn get_character_by_name(&self, name: &str) -> Option<&Character> {
        self.characters.iter().find(|&c| c.name == name)
    }
}

impl Display for Show {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}", self.name, self.release_year)
    }
}

impl DisplayMoreInfo for Show {
    fn more_info(&self) -> String {
        let mut message = String::from(format!("{self}\n"));
        message.push_str(&format!("Characters:\n"));
        for character in self.characters.iter() {
            message.push_str(&format!("\t- {character}\n"));
        }

        message
    }
}
