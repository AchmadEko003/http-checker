mod welcome;

use inquire::Select;
use reqwest::blocking::Client;
use std::io::{self, Write};
use welcome::{display_info, display_welcome};

fn main() {
    display_welcome();

    let options = vec!["Check My Site", "Info", "Quit"];

    loop {
        let ans = Select::new("Pilih salah satu:", options.clone())
            .prompt()
            .unwrap();

        match ans.to_lowercase().as_str() {
            "check my site" => check_site(),
            "info" => display_info(),
            "quit" => {
                println!("\nThank you for using HTTP Checker. Goodbye! 👋\n");
                break;
            }
            _ => println!("\n❌ Invalid option.\n"),
        }
    }
}

fn check_site() {
    print!("\nEnter the URL to check (e.g., https://example.com): ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut url = String::new();
    io::stdin()
        .read_line(&mut url)
        .expect("Failed to read line");

    let url = url.trim();

    if url.is_empty() {
        println!("❌ URL cannot be empty.\n");
        return;
    }

    println!("\n🔍 Checking {}...", url);

    match Client::new().get(url).send() {
        Ok(response) => {
            let status = response.status();
            let status_code = status.as_u16();

            println!("\n┌────────────────────────────────────────┐");
            println!("│           CHECK RESULTS                │");
            println!("├────────────────────────────────────────┤");
            println!("│  URL: {:<32} │", truncate_string(url, 32));
            println!("│  Status Code: {:<24} │", status_code);
            println!("│  Status: {:<29} │", format!("{}", status));

            if status.is_success() {
                println!("│  Result: ✅ Site is UP                  │");
            } else if status.is_redirection() {
                println!("│  Result: ↪️  Redirection                │");
            } else if status.is_client_error() {
                println!("│  Result: ⚠️  Client Error               │");
            } else if status.is_server_error() {
                println!("│  Result: ❌ Server Error                │");
            } else {
                println!("│  Result: ℹ️  Unknown Status             │");
            }

            println!("└────────────────────────────────────────┘\n");
        }
        Err(e) => {
            println!("\n❌ Failed to reach the site.");
            println!("   Error: {}\n", e);
        }
    }
}

fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len - 3])
    }
}
