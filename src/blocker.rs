use std::{thread, time::Duration};
use crate::state::{save, load, now, State};
use crate::hosts::{apply_block, clean_block};
use std::fs;
use std::os::unix::fs::PermissionsExt;

const HOSTS: &str = "/etc/hosts";

pub fn set_block(seconds: u64) {
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

fn lock() {
    let p = fs::Permissions::from_mode(0o444);
    fs::set_permissions(HOSTS, p).ok();
}

fn unlock() {
    let p = fs::Permissions::from_mode(0o644);
    fs::set_permissions(HOSTS, p).ok();
}