use regex::Regex;

pub fn scan_version(version: &str) -> bool {
    let regex = Regex::new(r"\b\d+(?:\.\d+){1,}(?:-[A-Za-z0-9]+)?\b").unwrap();

    if let Some(_) = regex.find(version) {
        false
    } else {
        true
    }
}

pub fn scan_value(value: &str) -> bool {
    let regex = Regex::new(r"[A-Za-z0-9\.\-_]+").unwrap();

    if let Some(_) = regex.find(value) {
        false
    } else {
        true
    }
}

pub fn scan_web_server(server: &str) -> bool {
    let regex = Regex::new(r"(?i)\b(apache|nginx|iis|microsoft-iis|jetty|litespeed|openlitespeed|caddy|cherokee|gws|tornado|awselb|haproxy|traefik|envoy|
        nuxt|next|express|laravel|symfony|django|spring|rails|flask|fastapi|ubuntu|debian|centos|redhat|fedora|windows|freebsd|alpine|amazon linux|alma linux)\b").unwrap();

    if let Some(_) = regex.find(server) {
        false
    } else {
        true
    }
}
