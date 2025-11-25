use inquire::Confirm;

pub fn display_welcome() {
    println!("\n╔════════════════════════════════════════╗");
    println!("║     Welcome to HTTP Checker v0.1.0     ║");
    println!("╚════════════════════════════════════════╝\n");
}

pub fn display_info() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
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
        Ok(_) => print!("{esc}[2J{esc}[1;1H", esc = 27 as char),
        Err(_) => println!("Failed to read input."),
    }
}
