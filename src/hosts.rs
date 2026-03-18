use std::fs;

const HOSTS: &str = "/etc/hosts";
const DOMAINS: &str = "/etc/blocker/domains.txt";

pub fn add_domain(domain: &str) {
    let mut list = load_domains();
    if !list.contains(&domain.to_string()) {
        list.push(domain.to_string());
        save_domains(&list);
    }
}

pub fn remove_domain(domain: &str) {
    let list: Vec<String> = load_domains()
        .into_iter()
        .filter(|d| d != domain)
        .collect();

    save_domains(&list);
}

pub fn list_domains() {
    for d in load_domains() {
        println!("{}", d);
    }
}

pub fn apply_block() {
    let domains = load_domains();
    let content = fs::read_to_string(HOSTS).unwrap();
    let mut base = strip_block(&content);

    base.push_str("# BLOCKER START\n");

    for d in domains {
        base.push_str(&format!("127.0.0.1 {}\n", d));
        base.push_str(&format!("127.0.0.1 www.{}\n", d));
    }

    base.push_str("# BLOCKER END\n");

    write_atomic(&base);
}

pub fn clean_block() {
    let content = fs::read_to_string(HOSTS).unwrap();
    let cleaned = strip_block(&content);
    write_atomic(&cleaned);
}

fn load_domains() -> Vec<String> {
    fs::read_to_string(DOMAINS)
        .unwrap_or_default()
        .lines()
        .map(|l| l.to_string())
        .collect()
}

fn save_domains(domains: &Vec<String>) {
    fs::create_dir_all("/etc/blocker").ok();
    let data = domains.join("\n");
    fs::write(DOMAINS, data).unwrap();
}

fn strip_block(content: &str) -> String {
    let mut out = String::new();
    let mut skip = false;

    for line in content.lines() {
        if line.trim() == "# BLOCKER START" {
            skip = true;
            continue;
        }
        if line.trim() == "# BLOCKER END" {
            skip = false;
            continue;
        }
        if !skip {
            out.push_str(line);
            out.push('\n');
        }
    }

    out
}

fn write_atomic(content: &str) {
    fs::write("/etc/hosts.tmp", content).unwrap();
    fs::rename("/etc/hosts.tmp", HOSTS).unwrap();
}