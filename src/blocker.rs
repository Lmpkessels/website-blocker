use std::{thread, time::Duration};
use crate::state::{save, load, now, State};
use crate::hosts::{apply_block, clean_block};
use crate::permissions::{lock, unlock};
use clap::ValueEnum;

#[derive(Debug, Clone, ValueEnum)]
pub enum Unit {
    Min,
    Hour,
}

pub fn set_block(amount: u64, unit: Unit) {
    let seconds = match unit {
        Unit::Min => amount * 60,
        Unit::Hour => amount * 60 * 60,
    };

    apply_block();
    lock();

    let state = State {
        blocked: true,
        end: now() + seconds,
    };
    save(&state);
}

pub fn set_unblock() {
    let state = State {
        blocked: false,
        end: 0,
    };
    save(&state);
}

pub fn daemon() {
    loop {
        enforce();
        thread::sleep(Duration::from_secs(2));
    }
}

fn enforce() {
    if let Some(state) = load() {
        if state.blocked && now() < state.end {
            apply_block();
            lock();
        } else {
            clean_block();
            unlock();

            save(&State {
                blocked: false,
                end: 0,
            });
        }
    }
}