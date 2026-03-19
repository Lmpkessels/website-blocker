pub mod cli;
pub mod blocker;
pub mod hosts;
pub mod permissions;

pub use blocker::{Unit, set_block, set_unblock};
pub use hosts::{
    add_domain, remove_domain, list_domains, apply_block, clean_block
};
pub use permissions::{lock, unlock};