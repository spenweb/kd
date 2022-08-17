use inquire::{Confirm, Text};
use kd::models::{show_collection::ShowCollection, character::Character};

use super::show_suggestor;

pub fn add_character_controller(
    name: Option<String>,
    role: Option<String>,
    gender: Option<String>,
) {
    let mut show_collection = match ShowCollection::load() {
        Ok(show_collection) => show_collection,
        Err(e) => return eprintln!("Unable to load shows: {e}"),
    };
    let show = Text::new("Show's title:")
        .with_suggester(&|input: &str| show_suggestor(&show_collection, input))
        .prompt()
        .unwrap();
    let name = match name {
        Some(name) => name,
        None => inquire::Text::new("Character name:").prompt().unwrap(),
    };
    let role = match role {
        Some(role) => role,
        None => {
            let options = vec!["protagonist", "antagonist", "comic-relief"];
            inquire::Select::new("Role:", options)
                .prompt()
                .unwrap()
                .to_string()
        }
    };
    let gender = match gender {
        Some(gender) => gender,
        None => {
            let options = vec!["female", "male", "other"];
            inquire::Select::new("Gender:", options)
                .prompt()
                .unwrap()
                .to_string()
        }
    };

    let character = Character::new(name, role, gender);

    if let Ok(true) = Confirm::new(format!("Does this info look correct: {character}").as_str())
        .with_default(true)
        .with_help_message("Will save if correct")
        .prompt()
    {
        let character = match show_collection.add_character(show.as_str(), character) {
            Ok(character) => character,
            Err(e) => return eprintln!("{e}"),
        };
        let character_string = character.to_string();
        match show_collection.save() {
            Ok(_) => println!("Added new character: {character_string}"),
            Err(e) => eprintln!("{e}"),
        }
    }
}
