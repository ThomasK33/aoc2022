mod day1;
mod day2;
mod day3;

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
    Day1A { path: PathBuf },
    /// Completes day 1 task B
    Day1B { path: PathBuf },
    /// Completes day 2 task A
    Day2A { path: PathBuf },
    /// Completes day 2 task B
    Day2B { path: PathBuf },
    /// Completes day 3 task A
    Day3A { path: PathBuf },
    /// Completes day 3 task B
    Day3B { path: PathBuf },
}

fn main() {
    let cli = Cli::parse();

    env_logger::Builder::new()
        .filter_level(cli.verbose.log_level_filter())
        .init();

    if let Err(err) = match cli.command {
        Command::Day1A { path: file } => day1::solve_a(file),
        Command::Day1B { path: file } => day1::solve_b(file),
        Command::Day2A { path: file } => day2::solve(file, false),
        Command::Day2B { path: file } => day2::solve(file, true),
        Command::Day3A { path: file } => day3::solve(file),
        Command::Day3B { path: file } => day3::solve_b(file),
    } {
        log::error!("An error occurred while running the command: {err}");
    };
}
