//TODO: Create a file to store the domains the user gives outside of /etc/hosts
use crate::{ 
    Unit, add_domain, block_domains, remove_domain, list_domains,
    unblock_domains 
};
use clap::{ Parser, Subcommand, CommandFactory };
use std::collections::HashSet;

#[derive(Parser)]
#[command(name = "blocker")]
#[command(about = "Block distracting domains by editing /etc/hosts")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Add {
        domain: String,
    },
    Block {
        time: u64,
        unit: Unit,
    },
    Remove {
        domain: String,
    },
    List,
    Unblock
}

impl Commands {
    pub fn run(self) {
        let mut domains: HashSet<String> = HashSet::new();

        // Stored data to test if the program works
        domains.insert("amazon".to_string());

        match self {
            Commands::Add { domain } => add_domain(&mut domains, &domain),
            Commands::Block { time, unit } => block_domains(&domains, time, unit),
            Commands::Remove { domain } => remove_domain(&mut domains, &domain),
            Commands::List => list_domains(),
            Commands::Unblock => unblock_domains(),
        }
    }
}