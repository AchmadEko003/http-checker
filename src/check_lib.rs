use regex::Regex;

pub fn check_version(version: &str) -> Option<String> {
    let regex = Regex::new(r"\b\d+(?:\.\d+){1,}(?:-[A-Za-z0-9]+)?\b").unwrap();

    if let Some(_) = regex.find(version) {
        Some(String::from("⚠️"))
    } else {
        None
    }
}

pub fn check_value(value: &str) -> Option<String> {
    let regex = Regex::new(r"[A-Za-z0-9\.\-_]+").unwrap();

    if let Some(_) = regex.find(value) {
        Some(String::from("⚠️"))
    } else {
        None
    }
}

pub fn check_web_server(server: &str) -> Option<String> {
    let regex = Regex::new(r"(?i)\b(apache|nginx|iis|microsoft-iis|jetty|litespeed|openlitespeed|caddy|cherokee|gws|tornado|awselb|haproxy|traefik|envoy|
        nuxt|next|express|laravel|symfony|django|spring|rails|flask|fastapi|ubuntu|debian|centos|redhat|fedora|windows|freebsd|alpine|amazon linux|alma linux)\b").unwrap();

    if let Some(_) = regex.find(server) {
        Some(String::from("⚠️"))
    } else {
        Some(String::from("✅"))
    }
}
