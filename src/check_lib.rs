use regex::Regex;

pub fn check_version(version: &str) -> Option<String> {
    let regex = Regex::new(r"\b\d+(?:\.\d+){1,}(?:-[A-Za-z0-9]+)?\b").unwrap();

    if let Some(_) = regex.find(version) {
        Some(String::from(" ⚠️"))
    } else {
        None
    }
}

pub fn check_value(value: &str) -> Option<String> {
    let regex = Regex::new(r"[A-Za-z0-9\.\-_]+").unwrap();

    if let Some(_) = regex.find(value) {
        Some(String::from(" ⚠️"))
    } else {
        None
    }
}
