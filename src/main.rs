mod day1;

use std::path::PathBuf;

use clap::{Parser, Subcommand};
use clap_verbosity_flag::InfoLevel;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity<InfoLevel>,
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Completes day 1 task A
    Day1A { file: PathBuf },
    /// Completes day 1 task B
    Day1B { file: PathBuf },
}

fn main() {
    let cli = Cli::parse();

    env_logger::Builder::new()
        .filter_level(cli.verbose.log_level_filter())
        .init();

    if let Err(err) = match cli.command {
        Command::Day1A { file } => day1::solve_a(file),
        Command::Day1B { file } => day1::solve_b(file),
    } {
        log::error!("An error occurred while running the command: {err}");
    };
}
