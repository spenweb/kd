//!  # kd - Korean Drama
//!
//! kd helps easily document Korean Dramas making watching Korean Dramas more fun!
use clap::{Parser, Subcommand};
use kd::{config, korean::utils, models::actor::Actor};
use std::process;

pub mod controllers;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(arg_required_else_help(true))]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,

    #[clap(global = true, short, long, action = clap::ArgAction::Count)]
    verbose: u8,
}

#[derive(Subcommand)]
enum Commands {
    /// View config info
    Config,

    /// Convert currencies
    Convert {
        /// Korean Won to convert to USD
        #[clap(value_parser)]
        won: f64,
    },

    /// Interact with shows
    Show {
        #[clap(subcommand)]
        command: Option<ShowCommands>,
    },

    /// Interact with characters
    Character {
        #[clap(subcommand)]
        command: Option<CharacterCommands>,
    },

    /// Interact with actors
    Actor {
        #[clap(subcommand)]
        command: Option<ActorCommands>,
    },
}

#[derive(Subcommand)]
enum ShowCommands {
    /// Add show
    Add {
        /// Name of show
        #[clap(short, long, required(false))]
        name: Option<String>,

        /// Release year of show
        #[clap(short, long, required(false))]
        release_year: Option<i16>,
    },

    /// Updae show
    Update {
        /// Old name of show
        #[clap(short, long, required(false))]
        old_name: Option<String>,

        /// New name of show
        #[clap(short, long, required(false))]
        new_name: Option<String>,

        /// Release year of show
        #[clap(short, long, required(false))]
        release_year: Option<i16>,
    },

    /// Show info on show
    Info {
        /// Name of show
        #[clap(short, long, required(false))]
        name: Option<String>,
    },
}

#[derive(Subcommand)]
enum CharacterCommands {
    /// Add character
    Add {
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

    /// Update character
    Update {
        /// Old name of character
        #[clap(short, long, required(false))]
        old_name: Option<String>,

        /// New name of character
        #[clap(short, long, required(false))]
        new_name: Option<String>,

        /// Role of character
        #[clap(short, long, required(false))]
        role: Option<String>,

        /// Gender of character
        #[clap(short, long, required(false))]
        gender: Option<String>,
    },
}

#[derive(Subcommand)]
enum ActorCommands {
    /// Add actor
    Add {
        /// Name of actor
        #[clap(short, long)]
        name: String,

        /// Birth year
        #[clap(short, long)]
        birth_year: i64,
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
        Some(Commands::Show { command }) => match command {
            Some(ShowCommands::Add { name, release_year }) => {
                controllers::show::add_show_controller(name, release_year)
            }
            Some(ShowCommands::Update {
                old_name,
                new_name,
                release_year,
            }) => controllers::show::update_show_controller(old_name, new_name, release_year),
            Some(ShowCommands::Info { name }) => controllers::show::display_more_info(name),
            None => {}
        },
        Some(Commands::Character { command }) => match command {
            Some(CharacterCommands::Add { name, role, gender }) => {
                controllers::character::add_character_controller(name, role, gender);
            }
            Some(CharacterCommands::Update {
                old_name,
                new_name,
                role,
                gender,
            }) => {
                controllers::character::update_character_controller(
                    old_name, new_name, role, gender,
                );
            }
            None => {}
        },
        Some(Commands::Actor { command }) => match command {
            Some(ActorCommands::Add { name, birth_year }) => {
                let actor = Actor::new(name, birth_year);
                println!("Added new actor: {actor}");
            }
            None => {}
        },
        None => {}
    }
}
