use std::fs;
use std::fs::{ OpenOptions, read_to_string };
use std::io::Write;
use std::time::Duration;
use std::thread::sleep;
use std::os::unix::fs::PermissionsExt;
use crate::constants::{ HOSTS_PATH, STORED_DOMAINS };
use clap::ValueEnum;

#[derive(Debug, Clone, ValueEnum)]
pub enum Unit {
    Min,
    Hour
}

// Block domains asks for a time in minutes or hours depending on user input,
// Then the file is made writable only for the given amount of time, when the
// timer has run out of time the file becomes writable again.
pub fn block_domains(time: u64, unit: Unit) {
    let time_to_block = match unit {
        Unit::Min => time * 60,
        Unit::Hour => time * 60 * 60,
    };

    // Check if a time to block is given by the user else exit function
    if time_to_block == 0 {
        return;
    }

    let content = read_to_string(HOSTS_PATH).expect("Cannot read /etc/hosts");
    let mut new_content = String::new();
    let mut skip_before = false;
    // Check all lines in /etc/hosts and overwrite the file in between the
    // comments # BLOCKER START, # BLOCKER END
    for line in content.lines() {
        if line.trim() == "# BLOCKER START" {
            skip_before = true;
            continue;
        }

        if line.trim() == "# BLOCKER END" {
            skip_before = false;
            continue;
        }

        if !skip_before {
            new_content.push_str(line);
            new_content.push('\n');
        }
    }

    new_content.push_str("# BLOCKER START\n");
    let domains = read_to_string(STORED_DOMAINS).expect("Cannot open /etc/hosts.stored");
    for domain in domains.lines() {
        new_content.push_str(
            &format!(
                "127.0.0.1 www.{}\n127.0.0.1 www.{}.com\n",
                domain,
                domain,
            )
        )
    }
    new_content.push_str("# BLOCKER END\n");

    let mut file_before = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(HOSTS_PATH)
        .expect("Cannot open /etc/hosts for writing");

    file_before.write_all(new_content.as_bytes())
        .expect("Failed to write /etc/hosts");

    // Make /etc/hosts unwritable during the block, and run the block,
    // then make it writable again
    let unwritable = fs::Permissions::from_mode(0o444);
    fs::set_permissions(HOSTS_PATH, unwritable).unwrap(); // Set permission to
                                                          // read write
    
    sleep(Duration::from_secs(time_to_block)); // Run the timer

    let writable = fs::Permissions::from_mode(0o644);   
    fs::set_permissions(HOSTS_PATH, writable).unwrap();   // Set permision to 
                                                          // write only
    
    let content_after = read_to_string(HOSTS_PATH).expect("Cannot read /etc/hosts");
    let mut cleaned_content = String::new();
    let mut skip_after = false;
    for line in content_after.lines() {
        if line.trim() == "# BLOCKER START" {
            skip_after = true;
            continue;
        }

        if line.trim() == "# BLOCKER END" {
            skip_after = false;
            continue;
        }

        if !skip_after {
            cleaned_content.push_str(line);
            cleaned_content.push('\n');
        }
    }

    let mut file_after = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(HOSTS_PATH)
        .unwrap();

    file_after.write_all(
        cleaned_content.as_bytes()
    ).expect("Failed to write /etc/hosts");
}