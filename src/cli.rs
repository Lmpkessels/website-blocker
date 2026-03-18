use clap::{Parser, Subcommand};
use crate::blocker::{set_block, set_unblock, daemon};

#[derive(Parser)]
#[command(name = "blocker")]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { domain: String },
    Remove { domain: String },
    List,
    Block { time: u64 },
    Unblock,
    Daemon,
}

impl Cli {
    pub fn run(self) {
        match self.command {
            Commands::Add { domain } => crate::hosts::add_domain(&normalize(&domain)),
            Commands::Remove { domain } => crate::hosts::remove_domain(&normalize(&domain)),
            Commands::List => crate::hosts::list_domains(),
            Commands::Block { time } => set_block(time),
            Commands::Unblock => set_unblock(),
            Commands::Daemon => daemon(),
        }
    }
}

fn normalize(d: &str) -> String {
    d.trim().trim_start_matches("www.").to_string()
}