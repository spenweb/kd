use inquire::{Confirm, CustomUserError, Text};
use kd::models::{character::Character, show_collection::ShowCollection};

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

pub fn update_character_controller(
    old_name: Option<String>,
    new_name: Option<String>,
    role: Option<String>,
    gender: Option<String>,
) {
    let mut show_collection = match ShowCollection::load() {
        Ok(show_collection) => show_collection,
        Err(e) => return eprintln!("Unable to load shows: {e}"),
    };
    let show_name = Text::new("Show's title:")
        .with_suggester(&|input: &str| show_suggestor(&show_collection, input))
        .prompt()
        .unwrap();
    let show = match show_collection.get_show_by_name(&show_name) {
        Some(show) => show,
        None => return eprintln!("Unable to find show"),
    };
    let old_name = match old_name {
        Some(name) => name,
        None => inquire::Text::new("Character's old name:")
            .with_suggester(&|input: &str| character_suggestor(&show, input))
            .prompt()
            .unwrap(),
    };
    let character = match show.get_character_by_name(&old_name) {
        Some(c) => c,
        None => return eprintln!("Unable to find character"),
    };
    let new_name = match new_name {
        Some(name) => name,
        None => inquire::Text::new("Character's new name:")
            .with_initial_value(&old_name)
            .prompt()
            .unwrap(),
    };
    let role = match role {
        Some(role) => role,
        None => {
            let options = vec!["protagonist", "antagonist", "comic-relief"];
            let selected_index = options
                .iter()
                .position(|&option| option == character.role)
                .unwrap_or(0);
            inquire::Select::new("Role:", options)
                .with_starting_cursor(selected_index)
                .with_vim_mode(true)
                .prompt()
                .unwrap()
                .to_string()
        }
    };
    let gender = match gender {
        Some(gender) => gender,
        None => {
            let options = vec!["female", "male", "other"];
            let selected_index = options
                .iter()
                .position(|&option| option == character.gender)
                .unwrap_or(0);
            inquire::Select::new("Gender:", options)
                .with_starting_cursor(selected_index)
                .with_vim_mode(true)
                .prompt()
                .unwrap()
                .to_string()
        }
    };

    let character = Character::new(new_name, role, gender);

    if let Ok(true) = Confirm::new(format!("Does this info look correct: {character}").as_str())
        .with_default(true)
        .with_help_message("Will save if correct")
        .prompt()
    {
        let character = match show_collection.update_character(&show_name, &old_name, character) {
            Ok(character) => character,
            Err(e) => return eprintln!("{e}"),
        };
        let character_string = character.to_string();
        match show_collection.save() {
            Ok(_) => println!("Updated character: {character_string}"),
            Err(e) => eprintln!("{e}"),
        }
    }
}

fn character_suggestor(
    show: &&kd::models::show::Show,
    input: &str,
) -> Result<Vec<String>, CustomUserError> {
    let character_names: Vec<&str> = show.characters.iter().map(|c| c.name.as_str()).collect();
    let input = input.to_lowercase();

    Ok(character_names
        .iter()
        .filter(|&p| p.to_lowercase().contains(&input))
        .take(5)
        .map(|p| String::from(*p))
        .collect())
}
