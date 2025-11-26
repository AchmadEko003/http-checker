use inquire::Confirm;
use regex::Regex;
use reqwest::blocking::{Client, Response};
use std::{
    io::{self, Write},
    time::Instant,
};

use crate::{
    states::{HeaderField, ScanStatus},
    utils::{scan_value, scan_version, scan_web_server},
};

pub fn check_site() {
    loop {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        print!("\nEnter the URL to check (e.g., https://example.com): ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut url = String::new();
        io::stdin()
            .read_line(&mut url)
            .expect("Failed to read line");

        let mut url_trimmed = url.trim().to_string();
        let regex = Regex::new(r"^(http|https)://").unwrap();

        if url_trimmed.is_empty() {
            println!("❌ URL cannot be empty.\n");
            return;
        }

        if !regex.is_match(&url_trimmed) {
            url_trimmed = String::from("https://") + &url_trimmed;
        }

        println!("\n🔍 Checking {}...", &url_trimmed);

        let time = Instant::now();

        match Client::new().get(&url_trimmed).send() {
            Ok(response) => {
                let status = response.status();
                let status_code = status.as_u16();

                let scheme = response.url().scheme().to_string();

                let mut result = ScanStatus::default();
                result.url = response.url().to_string();
                result.scheme = match scheme.as_str() {
                    "https" => scheme + " ✅",
                    "http" => scheme + " ⚠️",
                    _ => "N/A ⚠️".to_string(),
                };
                result.status_code = status_code;
                result.status_text = status.to_string();
                result.response_time_ms = time.elapsed().as_millis();

                result.header_info.server = check_server_header(&response);
                result.header_info.x_powered_by = check_powered_by(&response);
                result.header_info.x_content_type_options = check_content_type_options(&response);
                result.header_info.x_frame_options = check_frame_options(&response);
                result.header_info.strict_transport_security =
                    check_strict_transport_security(&response);
                // result.referrer_policy = get_header_value(&response, "Refferer-Policy");

                println!("\n┌────────────────────────────────────────┐");
                println!("│           CHECK RESULTS                │");
                println!("└────────────────────────────────────────┘");
                println!("URL: {}", &result.url);
                println!("Scheme: {}", &result.scheme);
                println!("Status: {}", &result.status_text);

                if status.is_success() {
                    println!("Result: Site is UP ✅");
                } else if status.is_redirection() {
                    println!("Result: Redirection 🔄");
                } else if status.is_client_error() {
                    println!("Result: Client Error ⚠️");
                } else if status.is_server_error() {
                    println!("Result: Server Error ❌");
                } else {
                    println!("Result: Unknown Status ℹ️");
                }

                println!("Response Time: {} ms\n", &result.response_time_ms);

                println!("======HEADER======\n");
                server_header_print(&result.header_info.server);
                powered_by_print(&result.header_info.x_powered_by);
                content_type_options_print(&result.header_info.x_content_type_options);
                frame_options_print(&result.header_info.x_frame_options);
                strict_transport_security_print(&result.header_info.strict_transport_security);
                println!("\n");
            }
            Err(e) => {
                println!("\n❌ Failed to reach the site.");
                println!("   Error: {}\n", e);
            }
        }

        let ans = Confirm::new("Do you want to check another site?")
            .with_default(true)
            .prompt();

        match ans {
            Ok(true) => continue,
            Ok(false) => break,
            Err(_) => {
                println!("Failed to read input. Returning to main menu.");
                break;
            }
        }
    }
}

fn get_header_value(response: &Response, key: &str) -> String {
    if let Some(header) = response.headers().get(key.to_lowercase()) {
        let mut value = header.to_str().unwrap_or("N/A").to_owned();

        match key.to_lowercase().as_str() {
            "strict-transport-security" => value = value.to_string() + " ✅",
            _ => {
                value = value.to_string();
            }
        }

        return format!("{}: {}", key, value);
    } else {
        return format!("{}: N/A ⚠️", key);
    }
}

// SEVER HEADER
fn check_server_header(response: &Response) -> HeaderField {
    if let Some(header) = response.headers().get("server") {
        let value = header.to_str().unwrap_or("N/A").to_string();

        let valid = if !scan_web_server(&value) {
            false
        } else if !scan_version(&value) {
            false
        } else {
            true
        };

        let note = if valid {
            None
        } else {
            Some("Recommended: remove or change server name and version".into())
        };

        HeaderField::Present {
            raw: value,
            valid,
            note: note,
        }
    } else {
        HeaderField::Present {
            raw: "n/a".to_string(),
            valid: true,
            note: None,
        }
    }
}

fn server_header_print(header: &HeaderField) {
    match &header {
        HeaderField::Present { raw, valid, note } => {
            let status_icon = if valid.to_owned() { "✅" } else { "⚠️" };
            let note_result = note
                .as_ref()
                .map(|item| format!(" ({})", item))
                .unwrap_or("".to_string());
            println!("Server: {} {}{}", raw, status_icon, note_result);
        }
        HeaderField::Missing => {
            println!("Server: N/A ✅");
        }
    }
}

// X-POWERED-BY HEADER
fn check_powered_by(response: &Response) -> HeaderField {
    if let Some(header) = response.headers().get("x-powered-by") {
        let value = header.to_str().unwrap_or("N/A").to_string();

        let valid = if !scan_value(&value) {
            false
        } else if !scan_version(&value) {
            false
        } else {
            true
        };

        let note = if valid {
            None
        } else {
            Some("Recommended: remove X-Powered-By header or version".into())
        };

        HeaderField::Present {
            raw: value,
            valid,
            note: note,
        }
    } else {
        HeaderField::Present {
            raw: "n/a".to_string(),
            valid: true,
            note: None,
        }
    }
}

fn powered_by_print(header: &HeaderField) {
    match &header {
        HeaderField::Present { raw, valid, note } => {
            let status_icon = if valid.to_owned() { "✅" } else { "⚠️" };
            let note_result = note
                .as_ref()
                .map(|item| format!(" ({})", item))
                .unwrap_or("".to_string());
            println!("X-Powered-By: {} {}{}", raw, status_icon, note_result);
        }
        HeaderField::Missing => {
            println!("X-Powered-By: N/A ✅");
        }
    }
}

// X-CONTENT-TYPE-OPTIONS HEADER
fn check_content_type_options(response: &Response) -> HeaderField {
    if let Some(header) = response.headers().get("x-content-type-options") {
        let value = header.to_str().unwrap_or("N/A").to_string();

        let valid = value.to_lowercase() == "nosniff".to_string();

        let note = if valid {
            None
        } else {
            Some("Recommended: set X-Content-Type-Options to 'nosniff'".into())
        };

        HeaderField::Present {
            raw: value,
            valid,
            note: note,
        }
    } else {
        HeaderField::Missing {}
    }
}

fn content_type_options_print(header: &HeaderField) {
    match &header {
        HeaderField::Present { raw, valid, note } => {
            let status_icon = if valid.to_owned() { "✅" } else { "⚠️" };
            let note_result = note
                .as_ref()
                .map(|item| format!(" ({})", item))
                .unwrap_or("".to_string());
            println!(
                "X-Content-Type-Options: {} {}{}",
                raw, status_icon, note_result
            );
        }
        HeaderField::Missing => {
            println!(
                "X-Content-Type-Options: N/A ⚠️ (Recommended: set X-Content-Type-Options header)"
            );
        }
    }
}

// X-FRAME-OPTIONS HEADER
fn check_frame_options(response: &Response) -> HeaderField {
    if let Some(header) = response.headers().get("x-frame-options") {
        let value = header.to_str().unwrap_or("N/A").to_string();

        let valid = matches!(value.to_lowercase().as_str(), "deny" | "sameorigin");

        let notes = if valid {
            None
        } else {
            Some("Recommended: set X-Frame-Options to 'DENY' or 'SAMEORIGIN'".into())
        };

        HeaderField::Present {
            raw: value,
            valid,
            note: notes,
        }
    } else {
        HeaderField::Missing {}
    }
}

fn frame_options_print(header: &HeaderField) {
    match &header {
        HeaderField::Present { raw, valid, note } => {
            let status_icon = if valid.to_owned() { "✅" } else { "⚠️" };
            let note_result = note
                .as_ref()
                .map(|item| format!(" ({})", item))
                .unwrap_or("".to_string());
            println!("X-Frame-Options: {} {}{}", raw, status_icon, note_result);
        }
        HeaderField::Missing => {
            println!("X-Frame-Options: N/A ⚠️ (Recommended: set X-Frame-Options header)");
        }
    }
}

// STRICT-TRANSPORT-SECURITY HEADER
fn check_strict_transport_security(response: &Response) -> HeaderField {
    if let Some(header) = response.headers().get("strict-transport-security") {
        let value = header.to_str().unwrap_or("N/A").to_string();

        let valid = if value != "N/A" { true } else { false };

        let note = if valid {
            None
        } else {
            Some("Recommended: enable Strict-Transport-Security header".into())
        };

        HeaderField::Present {
            raw: value,
            valid,
            note,
        }
    } else {
        HeaderField::Missing {}
    }
}

fn strict_transport_security_print(header: &HeaderField) {
    match &header {
        HeaderField::Present { raw, valid, note } => {
            let status_icon = if valid.to_owned() { "✅" } else { "⚠️" };
            let note_result = note
                .as_ref()
                .map(|item| format!(" ({})", item))
                .unwrap_or("".to_string());
            println!(
                "Strict-Transport-Security: {} {}{}",
                raw, status_icon, note_result
            );
        }
        HeaderField::Missing => {
            println!(
                "Strict-Transport-Security: N/A ⚠️ (Recommended: enable Strict-Transport-Security header)"
            );
        }
    }
}
