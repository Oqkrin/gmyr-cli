use super::Command;
use std::path::PathBuf;

pub struct Make {
    pub source_path: PathBuf,
}

impl Command for Make {
    fn on_execution(&self) -> anyhow::Result<()> {
        println!("🔨 making {:?}", self.source_path);
        Ok(())
    }
}