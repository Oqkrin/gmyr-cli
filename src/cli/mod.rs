mod commands;
use clap::Parser;
use crate::cli::commands::Command;

#[derive(Parser)]
#[command(version, about, long_about)]
pub struct GmyrCli {
    #[arg(long)]
    make: Option<std::path::PathBuf>,

    #[arg(long, num_args = 0..=1)]
    config: Option<Option<std::path::PathBuf>>,
}

pub fn parse() -> anyhow::Result<()> {
    let args = GmyrCli::parse();

    let mut execution_queue: Vec<Box<dyn Command>> = Vec::new();

    if let Some(option_config_path) = args.config {
        execution_queue.push(Box::new(commands::config::Config {
            option_config_path
        }));
    }

    if let Some(source_path) = args.make {
        execution_queue.push(Box::new(commands::make::Make { source_path }));
    }

    for cmd in execution_queue {
        cmd.on_execution()?;
    }

    Ok(())
}