use std::path::PathBuf;

pub const REVERS_DOMAIN: &'static str = "com.webspence.kd";

pub struct Config {
    config_dir: PathBuf,
    data_dir: PathBuf,
}

impl Config {
    pub fn new() -> Config {
        let config_dir = dirs::config_dir()
            .unwrap()
            .join(format!("{REVERS_DOMAIN}/config"));
        let data_dir = dirs::data_local_dir()
            .unwrap()
            .join(format!("{REVERS_DOMAIN}/data"));

        Config {
            config_dir,
            data_dir,
        }
    }
    pub fn init(&self) -> Result<(), String> {
        // Set up config directory
        if !&self.config_dir.exists() {
            if let Err(_) = std::fs::create_dir_all(&self.config_dir) {
                return Err(format!(
                    "Unabled to create config directory: {:?}",
                    self.config_dir.as_os_str()
                ));
            } else {
                println!(
                    "Created config directory: {:?}",
                    self.config_dir.as_os_str()
                );
            }
        }

        // Set up app data directory
        if !self.data_dir.exists() {
            if let Err(_) = std::fs::create_dir_all(&self.data_dir) {
                return Err(format!(
                    "Unabled to create data directory: {:?}",
                    self.data_dir.as_os_str()
                ));
            } else {
                println!("Created data directory: {:?}", self.data_dir.as_os_str());
            }
        }
        Ok(())
    }

    pub fn get_config_dir(&self) -> &PathBuf {
        &self.config_dir
    }

    pub fn get_data_dir(&self) -> &PathBuf {
        &self.data_dir
    }
}
