use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::domain::battery::{self, cli::BatteryArgs};

#[derive(Parser)]
#[command(name = "x")]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}
#[derive(Subcommand)]
pub enum Command {
    #[command(visible_alias = "b")]
    Battery(BatteryArgs),
}
pub fn run() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Battery(args) => battery::cli::handle(args),
    }
}
