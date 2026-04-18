use super::Command;
use std::path::PathBuf;

pub struct Config {
    pub option_config_path: Option<PathBuf>,
}

impl Command for Config {
    fn on_execution(&self) -> anyhow::Result<()> {
        match &self.option_config_path {
            Some(new_config_path) => {
                println!("⚙️ Setting configuration path to: {:?}", new_config_path);
            }
            None => {
                // Logic to retrieve the current path
                let current_config_path = "/etc/gmyr/config.toml"; // Example default
                println!("🔍 Current configuration location: {}", current_config_path);
            }
        }
        Ok(())
    }
}