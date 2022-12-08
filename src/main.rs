mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

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
    /// Completes day 4 tasks
    Day4 { path: PathBuf },
    /// Completes day 5 task A
    Day5A { path: PathBuf },
    /// Completes day 5 task B
    Day5B { path: PathBuf },
    /// Completes day 6 task A
    Day6A { path: PathBuf },
    /// Completes day 6 task B
    Day6B { path: PathBuf },
    /// Completes day 7
    Day7 { path: PathBuf },
    /// Completes day 8
    Day8 { path: PathBuf },
}

fn main() {
    let cli = Cli::parse();

    env_logger::Builder::new()
        .filter_level(cli.verbose.log_level_filter())
        .init();

    if let Err(err) = match cli.command {
        Command::Day1A { path } => day1::solve_a(path),
        Command::Day1B { path } => day1::solve_b(path),
        Command::Day2A { path } => day2::solve(path, false),
        Command::Day2B { path } => day2::solve(path, true),
        Command::Day3A { path } => day3::solve(path),
        Command::Day3B { path } => day3::solve_b(path),
        Command::Day4 { path } => day4::solve(path),
        Command::Day5A { path } => day5::solve(path, true),
        Command::Day5B { path } => day5::solve(path, false),
        Command::Day6A { path } => day6::solve(path, 4),
        Command::Day6B { path } => day6::solve(path, 14),
        Command::Day7 { path } => day7::solve(path),
        Command::Day8 { path } => day8::solve(path),
    } {
        log::error!("An error occurred while running the command: {err}");
    };
}
