use std::{fs::{read_to_string, OpenOptions}, thread::{spawn, sleep}, sync::{Arc, RwLock}, time::Duration};

use lazy_static::lazy_static;
use ron::{from_str, to_string};

struct Command {
    regex: String,
    response: String
}

struct ServerCommands {
    server_id: u64,
    commands: Vec<Command>   
}

lazy_static! {
    pub static ref CUST_CMDS: Arc<RwLock<Vec<ServerCommands>>> = setup_cmds();
}

static CUST_CMDS_NAME: String = "cust_cmds.ron".to_string();

fn setup_cmds() -> Arc<RwLock<Vec<ServerCommands>>> {
    let cmds = load_cmds();
    spawn(||autosave_cmds());
    Arc::new(RwLock::new(cmds))
}

fn autosave_cmds() -> ! {
    loop {
        sleep(Duration::from_secs(60));
        if let Ok(cmds_lock) = CUST_CMDS.read() {
            if let Ok(file_out) = OpenOptions::new().truncate(true).open(CUST_CMDS_NAME) {
                if let Ok(write_str) = to_string(cmds_lock) {
                    write!(file_out, "{}", write_str);
                }   
            }
        }
    }
}

fn load_cmds() -> Vec<ServerCommands> {
    return from_str(read_to_string(CUST_CMDS_NAME).unwrap_or_default()).unwrap_or_default();
}