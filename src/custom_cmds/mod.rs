use std::{
    collections::HashMap,
    fs::{read_to_string, OpenOptions},
    io::Write,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, RwLock,
    },
    thread::{sleep, spawn},
    time::Duration,
};

use lazy_static::lazy_static;
use regex::{Regex, RegexBuilder};
use ron::{from_str, to_string};

type ServerCmds = Vec<(Regex, String, String)>;
type Servers = HashMap<u64, ServerCmds>;
type Sync<T> = Arc<RwLock<T>>;

lazy_static! {
    static ref CHANGED: AtomicBool = AtomicBool::new(false);
    static ref CUST_CMDS: Sync<Servers> = setup_cmds();
}

pub fn get_commands(server_id: &u64) -> ServerCmds {
    if let Ok(lock) = CUST_CMDS.read() {
        if let Some(cmds) = lock.get(server_id) {
            return cmds.clone();
        }
    }
    vec![]
}

pub fn add_command(server_id: &u64, regex_str: String, reply: String) -> String {
    match RegexBuilder::new(&regex_str).size_limit(1 << 20).build() {
        Ok(regex) => {
            let entry = (regex, regex_str, reply);
            match CUST_CMDS.write() {
                Ok(mut lock) => {
                    CHANGED.store(true, Ordering::Relaxed);
                    match lock.get_mut(server_id) {
                        Some(server_cmds) => {
                            server_cmds.push(entry);
                            "Added new command to server's command list."
                        }
                        None => {
                            let _ = lock.insert(*server_id, vec![entry]);
                            "Added first command to server's command list."
                        }
                    }
                }
                _ => "Failed to acquire write lock on the command list.",
            }
        }
        .to_owned(),
        Err(regex_err) => match regex_err {
            regex::Error::Syntax(syntax) => match syntax.len() > 1972 {
                true => format!("Invalid regex syntax: ```{}...```", &syntax[..1969]),
                false => format!("Invalid regex syntax: ```{}```", syntax),
            },
            regex::Error::CompiledTooBig(too_big) => format!(
                "Compiled regex exceeded allowed 1 MiB: {} bytes used.",
                too_big
            ),
            _ => "Failed to compile regex.".to_owned(),
        },
    }
}

pub fn remove_command(server_id: &u64, index: usize) -> String {
    match CUST_CMDS.write() {
        Ok(mut lock) => match lock.get_mut(server_id) {
            Some(server_cmds) => match server_cmds.len() <= index {
                true => {
                    CHANGED.store(true, Ordering::Relaxed);
                    server_cmds.remove(index);
                    "Successfully removed command."
                }
                _ => "Server doesn't have that command.",
            },
            _ => "Server has no commands.",
        },
        _ => "Failed to acquire write lock on the command list.",
    }
    .to_owned()
}

fn setup_cmds() -> Sync<Servers> {
    let cmds = load_cmds();
    let _thread = spawn(autosave_cmds);
    Arc::new(RwLock::new(cmds))
}

fn autosave_cmds() -> ! {
    loop {
        sleep(Duration::from_secs(60));
        if CHANGED.load(Ordering::Relaxed) {
            if let Ok(cmds_lock) = CUST_CMDS.read() {
                if let Ok(mut file_out) = OpenOptions::new()
                    .create(true)
                    .write(true)
                    .truncate(true)
                    .open("cust_cmds.ron")
                {
                    let compacted: HashMap<u64, Vec<(String, String)>> = cmds_lock
                        .iter()
                        .map(|(id, list)| {
                            (
                                *id,
                                list.iter()
                                    .map(|(_reg, reg_str, reply)| (reg_str.clone(), reply.clone()))
                                    .collect(),
                            )
                        })
                        .collect();
                    if let Ok(write_str) = to_string(&compacted) {
                        if write!(file_out, "{}", write_str).is_ok() {
                            CHANGED.store(false, Ordering::Relaxed);
                        }
                    }
                }
            }
        }
    }
}

fn load_cmds() -> Servers {
    let compacted: HashMap<u64, Vec<(String, String)>> =
        from_str(&read_to_string("cust_cmds.ron").unwrap_or_default()).unwrap_or_default();
    let mut changed = false;
    let loaded = compacted
        .into_iter()
        .map(|(id, list)| {
            (
                id,
                list.into_iter()
                    .filter_map(|(reg_str, reply)| {
                        match RegexBuilder::new(&reg_str).size_limit(1 << 20).build() {
                            Ok(regex) => Some((regex, reg_str, reply)),
                            _ => {
                                changed = true;
                                None
                            }
                        }
                    })
                    .collect(),
            )
        })
        .collect();
    CHANGED.store(changed, Ordering::Relaxed);
    loaded
}
