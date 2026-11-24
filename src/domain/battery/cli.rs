use clap::{Arg, Command};

pub fn command() -> Command {
    Command::new("battery")
        .about("Battery information and controls")
        .subcommand(Command::new("status").about("Show battery status"))
        .subcommand(Command::new("health").about("Show battery health"))
}
