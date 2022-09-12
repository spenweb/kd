use inquire::CustomUserError;
use kd::models::{show::Show, show_collection::ShowCollection};

pub mod character;
pub mod show;

fn show_suggestor(
    show_collection: &ShowCollection,
    input: &str,
) -> Result<Vec<String>, CustomUserError> {
    let shows: Vec<&str> = show_collection.get_show_names();
    let input = input.to_lowercase();

    Ok(shows
        .iter()
        .filter(|p| p.to_lowercase().contains(&input))
        .take(5)
        .map(|p| String::from(*p))
        .collect())
}

fn character_suggestor(show: &Show, input: &str) -> Result<Vec<String>, CustomUserError> {
    let characters: Vec<&str> = show.characters.iter().map(|c| c.name.as_str()).collect();
    let input = input.to_lowercase();

    Ok(characters
        .iter()
        .filter(|p| p.to_lowercase().contains(&input))
        .take(5)
        .map(|p| String::from(*p))
        .collect())
}

fn relationship_suggestor(show: &Show, input: &str) -> Result<Vec<String>, CustomUserError> {
    let input = input.to_lowercase();
    let relationships = show
        .relationships
        .values()
        .filter(|&p| p.kind.to_lowercase().contains(&input))
        .take(5)
        .map(|r| r.kind.clone())
        .collect();

    Ok(relationships)
}
