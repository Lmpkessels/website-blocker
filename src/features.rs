use std::fs::{ File, OpenOptions };
use std::io::{ Read, Write };
use std::str;

fn list_domains() {
    // Read a /etc/hosts from the local file system
    let mut blocked_domain = File::open("/etc/hosts").unwrap();
    
    // Create a empty mutable string to copy contents of /etc/hosts to
    let mut domain = String::new();
    
    // Copy contents of /etc/hosts to mutable string
    blocked_domain.read_to_string(&mut domain).unwrap();

    println!("{}", domain);
}

// TODO: Get root privaliges to write to /etc/hosts
fn add_domain(domain: &str) {
    // Give the privaliges to append data to the right file, and raise an
    // error if appending is not possible
    let mut blocked_domains = OpenOptions::new()
        .append(true)
        .open("data.txt")
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

fn main() {
    list_domains();
    add_domain("amazon");
}