mod cli;
mod state;
mod hosts;
mod blocker;

use cli::Cli;
use clap::Parser;

fn main() {
    Cli::parse().run();
}