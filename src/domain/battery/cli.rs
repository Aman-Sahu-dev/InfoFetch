use anyhow::Result;
use clap::Args;

use super::{battery::Spec};

#[derive(Args)]
pub struct BatteryArgs {
    #[arg(short = 'l' , long = "limit" , value_parser = parse_percentage)]
    limit: Option<u8>,
}
pub fn handle(BatteryArgs) -> Result<()> {

}
