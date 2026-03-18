use std::fs;
use std::os::unix::fs::PermissionsExt;

const HOSTS_PATH: &str = "/etc/hosts";

pub fn lock() {
    let perms = fs::Permissions::from_mode(0o444);
    fs::set_permissions(HOSTS_PATH, perms).unwrap();
}

pub fn unlock() {
    let perms = fs::Permissions::from_mode(0o644);
    fs::set_permissions(HOSTS_PATH, perms).unwrap();
}