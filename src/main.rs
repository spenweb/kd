//!  # kd - Korean Drama
//!
//! kd helps easily document Korean Dramas making watching Korean Dramas more fun!
use std::{env, process};

use clap::{Parser, Subcommand};
use kd::{actor::Actor, character::Character, korean::utils, show::Show};
use num_format::{Buffer, Locale, ToFormattedString, WriteFormatted};

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
        #[clap(short, long)]
        name: String,

        /// Release year of show
        #[clap(short, long)]
        release_year: i16,
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
                let show = Show::new(name, release_year);
                println!("Added new show: {show}");
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
