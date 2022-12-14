use std::{collections::HashMap, error::Error, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::config::Config;

use super::{character::Character, show::Show};

pub const SHOWS_FILE_NAME: &'static str = "shows.json";

#[derive(Serialize, Deserialize)]
pub struct ShowCollection {
    pub shows: HashMap<String, Show>,
}

impl ShowCollection {
    pub fn get_file_path() -> PathBuf {
        let config = Config::new();
        config.get_data_dir().join(SHOWS_FILE_NAME)
    }

    pub fn load() -> Result<ShowCollection, Box<dyn Error>> {
        if !Self::get_file_path().exists() {
            return Ok(ShowCollection {
                shows: HashMap::new(),
            });
        }
        let content = std::fs::read_to_string(Self::get_file_path())?;
        Ok(serde_json::from_str(&content)?)
    }

    pub fn save(&mut self) -> Result<(), Box<dyn Error>> {
        let content = serde_json::to_string_pretty(&self)?;
        match std::fs::write(Self::get_file_path(), content) {
            Ok(_) => return Ok(()),
            Err(e) => return Err(Box::new(e)),
        }
    }

    pub fn add(&mut self, show: Show) {
        self.shows.insert(show.get_id().to_string(), show);
    }

    pub fn get_show_names(&self) -> Vec<&str> {
        self.shows
            .values()
            .map(|show| show.get_name().as_str())
            .collect()
    }

    pub fn add_character(
        &mut self,
        show_name: &str,
        character: Character,
    ) -> Result<&Character, &str> {
        // Find show with name
        for (_key, show) in self.shows.iter_mut() {
            if show.get_name() == show_name {
                return show.add_character(character);
            }
        }
        Err("Show not found")
    }

    pub fn update_character(
        &mut self,
        show_name: &str,
        old_name: &str,
        character: Character,
    ) -> Result<&Character, &str> {
        // Find show with name
        for (_key, show) in self.shows.iter_mut() {
            if show.get_name() == show_name {
                return show.update_character(old_name, character);
            }
        }
        Err("Show not found")
    }

    pub fn get_show_by_name(&self, name: &str) -> Option<&Show> {
        self.shows.values().find(|&show| show.get_name() == name)
    }

    // TODO: Consider not passing a new show object but instead just passing new name and release year
    pub fn update(&mut self, old_name: &str, show: Show) -> Result<&Show, &str> {
        let old_show = match self.shows.values().find(|&old_show| old_show.name == old_name ) {
            Some(old_show) => old_show,
            None => return Err("Unable to find show with old name")
        };
        let id = String::from(old_show.get_id());
        let old_show = self.shows.get_mut(&id).unwrap();
        old_show.name = show.name;
        old_show.release_year = show.release_year;
        Ok(old_show)
    }
}
