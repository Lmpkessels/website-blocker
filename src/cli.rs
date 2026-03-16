//TODO: Create a file to store the domains the user gives outside of /etc/hosts
use crate::{ 
    Unit, add_domain, block_domains, remove_domain, list_domains,
    unblock_domains 
};
use clap::{ Parser, Subcommand };

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
        match self {
            Commands::Add { domain } => add_domain(&domain),
            Commands::Block { time, unit } => block_domains(time, unit),
            Commands::Remove { domain } => remove_domain(&domain),
            Commands::List => list_domains(),
            Commands::Unblock => unblock_domains(),
        }
    }
}