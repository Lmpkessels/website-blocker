use clap::{Parser, Subcommand};
use crate::blocker::{Unit, set_block, set_unblock};

#[derive(Parser)]
#[command(name = "blocker")]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { 
        domain: String 
    },
    Remove { 
        domain: String 
    },
    List,
    Block { 
        amount: u64,
        unit: Unit
    },
    Unblock,
}

impl Cli {
    pub fn run(self) {
        match self.command {
            Commands::Add { domain } => crate::hosts::add_domain(&normalize(&domain)),
            Commands::Remove { domain } => crate::hosts::remove_domain(&normalize(&domain)),
            Commands::List => crate::hosts::list_domains(),
            Commands::Block { amount, unit } => set_block(amount, unit),
            Commands::Unblock => set_unblock(),
        }
    }
}

fn normalize(d: &str) -> String {
    d.trim().trim_start_matches("www.").to_string()
}