use std::io::{self, Write};

use inquire::Confirm;

pub fn display_welcome() {
    println!("\n╔════════════════════════════════════════╗");
    println!("║     Welcome to HTTP Checker v0.1.0     ║");
    println!("╚════════════════════════════════════════╝\n");
}

pub fn get_user_choice() -> String {
    print!("\nEnter your choice: ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().to_string()
}

pub fn display_info() {
    println!("\n╔════════════════════════════════════════╗");
    println!("║              INFORMATION               ║");
    println!("╠════════════════════════════════════════╣");
    println!("║  HTTP Checker - v0.1.0                 ║");
    println!("║                                        ║");
    println!("║  A simple HTTP status checker tool     ║");
    println!("║  that verifies website availability    ║");
    println!("║  and returns status information.       ║");
    println!("║                                        ║");
    println!("║  Built with Rust 🦀                    ║");
    println!("╚════════════════════════════════════════╝\n");

    let ans = Confirm::new("Press Enter to return to the main menu...")
        .with_default(true)
        .prompt();

    match ans {
        Ok(_) => print!("\x1B[2J"),
        Err(_) => println!("Failed to read input."),
    }
}
