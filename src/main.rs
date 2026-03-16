use blocker::Cli;
use clap::Parser;

fn main() {
    let _cli = Cli::parse().command.run();
}