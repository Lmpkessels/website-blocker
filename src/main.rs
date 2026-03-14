pub use blocker::{ 
    add_domain, block_domains, list_domains, remove_domain, unblock_domains,
    Unit
};

fn main() {
    list_domains();
    block_domains(2, Unit::Minutes);
    unblock_domains();
    add_domain("test");
    remove_domain("test");
}