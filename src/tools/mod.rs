use lazy_static::lazy_static;
use regex::Regex;
use std::sync::{Arc, RwLock};

pub fn filter_pings(input: &str) -> String {
    lazy_static! {
        static ref PING_REGEX: Arc<RwLock<Option<Regex>>> = Arc::new(RwLock::new(
            Regex::new(r"<@(!|&)?\d+>|@everyone|@here").ok()
        ));
    };
    if let Ok(lock) = PING_REGEX.read() {
        if let Some(regex) = lock.clone() {
            if regex.is_match(input) {
                return "No pings allowed from this bot!".to_owned();
            }
        }
    }
    "Unable to verify if this pings another user or not.".to_owned()
}
