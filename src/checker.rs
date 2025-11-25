use inquire::Confirm;
use regex::Regex;
use reqwest::blocking::{Client, Response};
use std::{
    io::{self, Write},
    time::Instant,
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

        let url = url.trim();
        let regex = Regex::new(r"^(http|https)://").unwrap();

        if url.is_empty() {
            println!("❌ URL cannot be empty.\n");
            return;
        }

        if regex.is_match(&url) {
            url = format!("https://{}", &url.as_str);
        }

        println!("\n🔍 Checking {}...", url);

        let time = Instant::now();

        match Client::new().get(url).send() {
            Ok(response) => {
                let status = response.status();
                let status_code = status.as_u16();

                println!("\n┌────────────────────────────────────────┐");
                println!("│           CHECK RESULTS                │");
                println!("└────────────────────────────────────────┘");
                println!("URL: {}", url);
                println!("Status Code: {}", status_code);
                println!("Status: {}", status);

                let item: Vec<String> = response
                    .headers()
                    .iter()
                    .map(|(item, _)| item.to_string())
                    .collect();

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

                println!("Response Time: {} ms", time.elapsed().as_millis());
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

fn get_header_value(response: &Response, key: &str) -> Option<String> {
    let header = response.headers().get(key)?;

    Some(header.to_str().ok()?.to_string())
}
