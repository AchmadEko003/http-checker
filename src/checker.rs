use inquire::Confirm;
use regex::Regex;
use reqwest::blocking::{Client, Response};
use std::{
    io::{self, Write},
    time::Instant,
};

use crate::check_lib::{check_value, check_version, check_web_server};

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

                println!("\n┌────────────────────────────────────────┐");
                println!("│           CHECK RESULTS                │");
                println!("└────────────────────────────────────────┘");
                println!("URL: {}", url_trimmed);
                println!("Status Code: {}", status_code);
                println!("Status: {}", status);

                if status.is_success() {
                    println!("Result: ✅ Site is UP");
                } else if status.is_redirection() {
                    println!("Result: ↪️  Redirection");
                } else if status.is_client_error() {
                    println!("Result: ⚠️  Client Error");
                } else if status.is_server_error() {
                    println!("Result: ❌ Server Error");
                } else {
                    println!("Result: ℹ️  Unknown Status");
                }

                println!("Response Time: {} ms\n", time.elapsed().as_millis());

                println!("======HEADER======");
                get_header_value(&response, "Server");
                get_header_value(&response, "X-Powered-By");
                get_header_value(&response, "X-Content-Type-Options");
                get_header_value(&response, "X-Frame-Options");
                get_header_value(&response, "Strict-Transport-Security");
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

fn get_header_value(response: &Response, key: &str) {
    if let Some(header) = response.headers().get(key.to_lowercase()) {
        let mut value = header.to_str().unwrap_or("N/A").to_owned();

        match key.to_lowercase().as_str() {
            "server" => value = check_server_header(value.as_str()).unwrap_or("".to_string()),
            "x-powered-by" => value = check_server_header(value.as_str()).unwrap_or("".to_string()),
            "x-content-type-options" => {
                value = check_content_type_options(value.as_str()).unwrap_or("".to_string())
            }
            "x-frame-options" => {
                value = check_frame_options(value.as_str()).unwrap_or("".to_string())
            }
            "strict-transport-security" => value = value.to_string() + " ✅",
            _ => {
                value = value.to_string();
            }
        }

        println!("{}: {}", key, value);
    } else {
        println!("{}: N/A ⚠️", key);
    }
}

fn check_server_header(header_value: &str) -> Option<String> {
    let mut value = header_value.to_string();

    if let Some(val) = check_web_server(&value) {
        value = format!("{} {}", value, val);
    } else if let Some(version) = check_version(&value) {
        value = format!("{} {}", value, version);
    } else {
        return None;
    }

    Some(value)
}

fn check_content_type_options(content_type: &str) -> Option<String> {
    let mut value = content_type.to_string();

    if value.to_lowercase() != "nosniff" {
        let val = "⚠️";
        value = format!("{} {}", value, val);
    } else {
        let val = "✅";
        value = format!("{} {}", value, val);
    }

    Some(value)
}

fn check_frame_options(frame_options: &str) -> Option<String> {
    let mut value = frame_options.to_string();

    let val = match value.to_lowercase().as_str() {
        "deny" | "sameorigin" => "✅",
        _ => "⚠️",
    };

    value = format!("{} {}", value, val);

    Some(value)
}
