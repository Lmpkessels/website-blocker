use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

const PATH: &str = "/var/lib/blocker/state.json";

#[derive(serde::Serialize, serde::Deserialize)]
pub struct State {
    pub blocked: bool,
    pub end: u64,
}

pub fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

pub fn save(state: &State) {
    let data = serde_json::to_string(state).unwrap();
    fs::create_dir_all("/var/lib/blocker").ok();
    fs::write(PATH, data).unwrap();
}

pub fn load() -> Option<State> {
    let data = fs::read_to_string(PATH).ok()?;
    serde_json::from_str(&data).ok()
}