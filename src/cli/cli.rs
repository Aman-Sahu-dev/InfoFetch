use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::domain::battery::{self, cli::BatteryArgs};

#[derive(Parser)]
#[command(name = inf)]
pub struct cli {
    #[derive(Subcommand)]
    domain: Command,
}
#[derive(Subcommand)]
pub enum Command {
    #[command(visible_alias = "b")]
    Battery(BatteryArgs),
}
