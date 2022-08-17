//!  # kd - Korean Drama
//!
//! kd helps easily document Korean Dramas making watching Korean Dramas more fun!
use clap::{Parser, Subcommand};
use inquire::{Confirm, CustomType, CustomUserError, Text};
use kd::{
    actor::Actor, character::Character, config, korean::utils, show::Show,
    show_collection::ShowCollection,
};
use std::process;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,

    #[clap(global = true, short, long, action = clap::ArgAction::Count)]
    verbose: u8,
}

#[derive(Subcommand)]
enum Commands {
    /// Convert currencies
    Convert {
        /// Korean Won to convert to USD
        #[clap(value_parser)]
        won: f64,
    },

    /// Add entities to database (e.g., shows, actors, etc.)
    Add {
        #[clap(subcommand)]
        command: Option<AddCommands>,
    },

    /// View config info
    Config,
}

#[derive(Subcommand)]
enum AddCommands {
    /// Add show
    Show {
        /// Name of show
        #[clap(short, long, required(false))]
        name: Option<String>,

        /// Release year of show
        #[clap(short, long, required(false))]
        release_year: Option<i16>,
    },

    /// Add actor
    Actor {
        /// Name of actor
        #[clap(short, long)]
        name: String,

        /// Birth year
        #[clap(short, long)]
        birth_year: i64,
    },

    /// Add character
    Character {
        /// Name of character
        #[clap(short, long, required(false))]
        name: Option<String>,

        /// Role of character
        #[clap(short, long, required(false))]
        role: Option<String>,

        /// Gender of character
        #[clap(short, long, required(false))]
        gender: Option<String>,
    },
}

// fn format_float(input: f64) -> String {
//     let input: String = format!("")
//     input
// }

fn main() {
    let cli = Cli::parse();

    let config = config::Config::new();
    if let Err(e) = config.init() {
        eprintln!("{e}");
        process::exit(1);
    }

    // Optionally load env variables from config .env file
    let env_file = config.get_config_dir().join(".env");
    if let Ok(_) = dotenv::from_path(env_file.as_path()) {
        if cli.verbose > 0 {
            println!("Loaded from .env file");
        }
    }

    match cli.command {
        Some(Commands::Config) => {
            println!(
                "Config directory: {:?}",
                config.get_config_dir().as_os_str()
            );
            println!("Data directory: {:?}", config.get_data_dir().as_os_str());
        }
        Some(Commands::Convert { won }) => {
            println!("Converting {} won to usd...", won);
            match utils::krw_to_usd(won) {
                Ok(usd) => {
                    println!("{} won = {} usd", won, usd);
                }
                Err(e) => println!("{e}"),
            };
        }
        Some(Commands::Add { command }) => match command {
            Some(AddCommands::Show { name, release_year }) => {
                add_show_controller(name, release_year)
            }
            Some(AddCommands::Actor { name, birth_year }) => {
                let actor = Actor::new(name, birth_year);
                println!("Added new actor: {actor}");
            }
            Some(AddCommands::Character { name, role, gender }) => {
                add_character_controller(name, role, gender);
            }
            None => {}
        },
        None => {}
    }
}

fn add_character_controller(name: Option<String>, role: Option<String>, gender: Option<String>) {
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

fn add_show_controller(name: Option<String>, release_year: Option<i16>) {
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
