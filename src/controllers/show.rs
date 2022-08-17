use inquire::{Text, CustomType, Confirm};
use kd::{models::{show_collection::ShowCollection, show::Show}, DisplayMoreInfo};

use super::show_suggestor;


pub fn add_show_controller(name: Option<String>, release_year: Option<i16>) {
    let validated_name: String;
    let validated_release_year: i16;
    let mut show_collection = match ShowCollection::load() {
        Ok(show_collection) => show_collection,
        Err(e) => return eprintln!("Unable to load shows: {e}"),
    };
    if let Some(name) = name {
        validated_name = name;
    } else {
        validated_name = Text::new("Show's title:")
            .with_suggester(&|input: &str| show_suggestor(&show_collection, input))
            .prompt()
            .unwrap();
    }
    if let Some(release_year) = release_year {
        validated_release_year = release_year;
    } else {
        validated_release_year = CustomType::new("Show's release year:")
            .with_error_message("Please enter a valid year")
            .prompt()
            .unwrap();
    }
    let show = Show::new(validated_name, validated_release_year);
    if let Ok(true) = Confirm::new(format!("Does this info look correct: {show}").as_str())
        .with_default(true)
        .with_help_message("Will save if correct")
        .prompt()
    {
        show_collection.add(show);

        match show_collection.save() {
            Ok(_) => println!("Saved show"),
            Err(e) => return eprintln!("Unable to save show collection: {e}"),
        }
    }
}

pub fn display_more_info(name: Option<String>) {
    let show_collection = ShowCollection::load().unwrap();

    // Get name if not provided from command arguments
    let name = match name {
        Some(name) => name,
        None => {
            Text::new("Show name")
            .with_suggester(&|input: &str| show_suggestor(&show_collection, input)).prompt().unwrap()
        }
    };
    match show_collection.shows.values().find(|&show| {
        show.get_name() == name.as_str()
    }) {
        Some(show) => println!("{}", show.more_info()),
        None => eprintln!("Couldn't find show by that name")
    }
}

