use std::collections::HashSet;

pub fn add_domain(domains: &mut HashSet<String>, domain: &str) {
    domains.insert(domain.to_string());

    println!("Added {} to blocked domains", domain);
}

pub fn remove_domain(domains: &mut HashSet<String>, domain: &str) {
    domains.remove(domain);

    println!("Removed {} from blocked domains", domain);
}