use std::{
    collections::HashMap,
    fs::{read_to_string, OpenOptions},
    io::Write,
    sync::{Arc, RwLock},
    thread::{sleep, spawn},
    time::Duration,
};

use lazy_static::lazy_static;
use ron::{from_str, to_string};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Command {
    regex: String,
    response: String,
}
impl Command {
    pub fn new(regex: String, response: String) -> Self {
        Self { regex, response }
    }
}

lazy_static! {
    pub static ref CUST_CMDS: Arc<RwLock<HashMap<u64, Vec<Command>>>> = setup_cmds();
}

fn setup_cmds() -> Arc<RwLock<HashMap<u64, Vec<Command>>>> {
    let cmds = load_cmds();
    spawn(autosave_cmds);
    Arc::new(RwLock::new(cmds))
}

fn autosave_cmds() -> ! {
    loop {
        sleep(Duration::from_secs(60));
        if let Ok(cmds_lock) = CUST_CMDS.read() {
            if let Ok(mut file_out) = OpenOptions::new().truncate(true).open("cust_cmds.ron") {
                if let Ok(write_str) = to_string(&*cmds_lock) {
                    let _ = write!(file_out, "{}", write_str);
                }
            }
        }
    }
}

fn load_cmds() -> HashMap<u64, Vec<Command>> {
    from_str(&read_to_string("cust_cmds.ron").unwrap_or_default()).unwrap_or_default()
}