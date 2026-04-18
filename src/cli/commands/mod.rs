pub mod config;
pub mod make;

pub trait Command {
    fn on_execution(&self) -> anyhow::Result<()>;
}