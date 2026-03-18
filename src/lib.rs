pub mod cli;
pub mod blocker;
pub mod hosts;
pub mod permissions;
pub mod state;

pub use blocker::{set_block, set_unblock, daemon};
pub use hosts::{
    add_domain, remove_domain, list_domains, apply_block, clean_block
};
pub use permissions::{lock, unlock};
pub use state::{State, now, save, load};