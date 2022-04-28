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

type ServerCmds = HashMap<String, String>;
type Servers = HashMap<u64, ServerCmds>;

lazy_static! {
    pub static ref CUST_CMDS: Arc<RwLock<Servers>> = setup_cmds();
}

fn setup_cmds() -> Arc<RwLock<Servers>> {
    println!("Loaded: {:?}", load_cmds());
    let cmds = load_cmds();
    let _thread = spawn(autosave_cmds);
    Arc::new(RwLock::new(cmds))
}

fn autosave_cmds() -> ! {
    loop {
        sleep(Duration::from_secs(60));
        if let Ok(cmds_lock) = CUST_CMDS.read() {
            if let Ok(mut file_out) = OpenOptions::new().truncate(true).open("cust_cmds.ron") {
                if let Ok(write_str) = to_string(&*cmds_lock) {
                    let _ = write!(file_out, "{}", write_str);
                    println!("Saved commands.");
                }
                println!("Attempted to save commands.");
            }
            println!("Attempted to attempt.");
        }
        println!("Attempted that.");
    }
}

fn load_cmds() -> Servers {
    from_str(&read_to_string("cust_cmds.ron").unwrap_or_default()).unwrap_or_default()
}
