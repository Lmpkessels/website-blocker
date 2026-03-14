use blocker::Cli;
use clap::{ Parser, Subcommand, CommandFactory };

fn main() {
    let cli = Cli::parse().command.run();
}