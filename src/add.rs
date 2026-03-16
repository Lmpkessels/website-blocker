use std::fs::OpenOptions;
use std::io::Write;
use crate::constants::STORED_DOMAINS;

// Add domain, receives the input of the user with the domain that he/she wants
// to be blocked, then the IPs will be placed in front of it and the domain-
// name will be written itself and one with .com appended to /etc/hosts
pub fn add_domain(domain: &str) {
    let mut blocked_domains = OpenOptions::new()
        .append(true)
        .create(true)
        .open(STORED_DOMAINS)
        .expect("Cannot open file");

    // Concatenate the string into the right string order
    writeln!(blocked_domains, "{}", domain)
        .expect("Writing domain to list failed");

    println!("{} is added to the list", domain);
}