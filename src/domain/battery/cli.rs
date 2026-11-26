use anyhow::Result;
use clap::Args;

use super::battery::Spec;

#[derive(Args)]
pub struct BatteryArgs {
    #[arg(short = 'l' , long = "limit" , value_parser = parse_percentage)]
    limit: Option<u8>,
}
pub fn handle(args: BatteryArgs) -> Result<()> {
    match args.limit {
        Some(limit) => Spec::set_limit(limit),
        None => {
            println!("{}", Spec::spec());
            Ok(())
        }
    }
}
pub fn parse_percentage(raw: &str) -> Result<u8, String> {
    let value: u8 = raw
        .strip_suffix('%')
        .unwrap_or(raw)
        .parse()
        .map_err(|_| format!("enter the valid limit '{raw}"))?;
    if value > 100 {
        return Err("percentage must be between 0 and 100".into());
    }
    Ok(value)
}
