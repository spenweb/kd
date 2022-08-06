//!  # kd - Korean Drama
//!
//! kd helps easily document Korean Dramas making watching Korean Dramas more fun!
use clap::{Parser, Subcommand};
use kd::{actor::Actor, korean::utils, show::Show, character::Character};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
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
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Convert { won }) => {
            println!("Converting {} won to usd...", won);
            let usd = utils::krw_to_usd(won);
            println!("{} won = {} usd", won, usd);
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
