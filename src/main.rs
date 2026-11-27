mod cli;
mod domain;

pub fn main() {
    if let Err(e) = cli::cli::run() {
        eprintln!("Error: {e:#}");
        std::process::exit(1);
    }
}
