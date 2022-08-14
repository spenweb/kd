//!  # kd - Korean Drama
//!
//! kd helps easily document Korean Dramas making watching Korean Dramas more fun!
use std::process;

use clap::{Parser, Subcommand};
use inquire::{Confirm, CustomType, CustomUserError, Text};
use kd::{actor::Actor, character::Character, korean::utils, show::Show};

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
        #[clap(short, long)]
        name: String,

        /// Role of character
        #[clap(short, long)]
        role: String,

        /// Gender of character
        #[clap(short, long)]
        gender: String,
    },
}

// fn format_float(input: f64) -> String {
//     let input: String = format!("")
//     input
// }

fn main() {
    let cli = Cli::parse();
    // Set up config directory
    let config_dir = dirs::config_dir().unwrap();
    let config_dir = config_dir.join("kd");
    if !config_dir.exists() {
        if let Err(_) = std::fs::create_dir_all(&config_dir) {
            eprintln!(
                "Unabled to create config directory: {:?}",
                config_dir.as_os_str()
            );
            process::exit(1);
        } else {
            println!("Created config directory: {:?}", config_dir.as_os_str());
        }
    }
    // Optionally load env variables from config .env file
    let env_file = config_dir.join(".env");
    if let Ok(_) = dotenv::from_path(env_file.as_path()) {
        if cli.verbose > 0 {
            println!("Loaded from .env file");
        }
    }

    match cli.command {
        Some(Commands::Config) => {
            println!("Config directory: {:?}", config_dir.as_os_str());
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
                let character = Character::new(name, role, gender);
                println!("Added new character: {character}");
            }
            None => {}
        },
        None => {}
    }
}

fn add_show_controller(name: Option<String>, release_year: Option<i16>) {
    let validated_name: String;
    let validated_release_year: i16;
    if let Some(name) = name {
        validated_name = name;
    } else {
        validated_name = Text::new("Show's title:")
            .with_suggester(&show_suggestor)
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
        println!("Added new show: {show}");
    }
}

fn show_suggestor(input: &str) -> Result<Vec<String>, CustomUserError> {
    let input = input.to_lowercase();
    let shows = vec![
        "Our Blues",
        "It's Okay Not To Be Okay",
        "Crash Landing On You",
        "Extraordinary Attorney Woo Young Woo",
    ];

    Ok(shows
        .iter()
        .filter(|p| p.to_lowercase().contains(&input))
        .take(5)
        .map(|p| String::from(*p))
        .collect())
}
