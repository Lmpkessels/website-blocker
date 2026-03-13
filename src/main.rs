use std::fs::{ File, OpenOptions };
use std::fs;
use std::io::{ Read, Write };
use crossterm::event::{ read, Event, KeyCode };
use std::time::Duration;
use std::thread::sleep;
use std::os::unix::fs::PermissionsExt;

const HOSTS_PATH: &str = "data.txt"; // The path to which the domains will be 
                                     // written

// List domains displays a list of all domains in /etc/hosts so the ones which
// are blocked
fn list_domains() {
    // Read a /etc/hosts from the local file system
    let mut blocked_domain = File::open(HOSTS_PATH).unwrap();
    
    // Create a empty mutable string to copy contents of /etc/hosts to
    let mut domain = String::new();
    
    // Copy contents of /etc/hosts to mutable string
    blocked_domain.read_to_string(&mut domain).unwrap();

    println!("{}", domain);
}

// Add domain, receives the input of the user with the domain that he/she wants
// to be blocked, then the IPs will be placed in front of it and the domain-
// name will be written itself and one with .com appended to /etc/hosts
fn add_domain(domain: &str) {
    // TODO: Get root privaliges to write to /etc/hosts
    // Give the privaliges to append data to the right file, and raise an
    // error if appending is not possible
    let mut blocked_domains = OpenOptions::new()
        .append(true)
        .open(HOSTS_PATH)
        .expect("Cannot open file");

    // Concatenate the string into the right string order
    let formated_domain = format!(
        "\n127.0.0.1 {}\n127.0.0.1 {}.com",
        domain,
        domain
    );

    // Write the domain to the file
    blocked_domains
        .write_all(formated_domain.as_bytes())
        .expect("Writing to /etc/hosts failed");    

    println!("Appended content to a file");
}

// Unblock domains through typing 500 keystrokes
//
// The keystrokes will be received after pressing enter, then a count takes
// place if the number is >= 500 /etc/hosts is cleared, else the remaining
// strokes need to be typed.
fn unblock_domains() {
    println!("To unblock the domains type 500 keystrokes");

    const PASSPHRASE_LEN: u16 = 500;
    let mut keystrokes_recieved = 0;

    loop {
        // Count keystrokes
        if let Event::Key(event) = read().unwrap() {
            match event.code {
                KeyCode::Enter => {}, // 'Enter' is not counted as a keystroke
                _ => keystrokes_recieved += 1
            }
        }

        // Clear /etc/hosts when at least 500 keystrokes are received
        // to unblock domains
        if keystrokes_recieved >= PASSPHRASE_LEN {
            let blocked_domains = OpenOptions::new()
                .write(true)
                .truncate(true)
                .open("data.txt")
                .unwrap();

            println!("Domains unblocked");
            break;
        }
    }
}

enum Unit {
    Minutes,
    Hours
}

// Block domains asks for a time in minutes or hours depending on user input,
// Then the file is made writable only for the given amount of time, when the
// timer has run out of time the file becomes writable again.
fn block_domains(time: u64, unit: Unit) {
    let time_to_block = match unit {
        Unit::Minutes => time * 60,
        Unit::Hours => time * 60 * 60,
    };

    if time_to_block == 0 {
        return;
    }

    let unwritable = fs::Permissions::from_mode(0o444);
    let writable = fs::Permissions::from_mode(0o644);   

    fs::set_permissions(HOSTS_PATH, unwritable).unwrap(); // Set permission to
                                                          // read write

    // Run the timer
    sleep(Duration::from_secs(time_to_block));

    fs::set_permissions(HOSTS_PATH, writable).unwrap();   // Set permision to 
                                                          // write only
}

fn main() {
    // list_domains();
    // add_domain("test");
    // unblock_domains();
    block_domains(1, Unit::Minutes);
}