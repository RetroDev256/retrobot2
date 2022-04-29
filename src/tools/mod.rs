use regex::Regex;

thread_local! {
    static PING_REGEX: Option<Regex> = Regex::new(r"<@(!|&)?\d+>|@everyone|@here").ok();
}

pub fn filter_pings(input: &str) -> String {
    PING_REGEX
        .with(|regex_opt| match regex_opt {
            Some(regex) => match regex.is_match(input) {
                true => "No pings allowed from this bot!",
                _ => input,
            },
            _ => "Unable to verify if this pings another user or not.",
        })
        .to_owned()
}
