use inquire::CustomUserError;
use kd::models::show_collection::ShowCollection;

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