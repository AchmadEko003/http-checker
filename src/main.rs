mod checker;
mod welcome;

use inquire::Select;
use welcome::{display_info, display_welcome};

use crate::checker::check_site;

fn main() {
    let options = vec!["Check Site", "Info", "Quit"];

    loop {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        display_welcome();

        let ans = Select::new("Pilih salah satu:", options.clone())
            .prompt()
            .unwrap();

        match ans.to_lowercase().as_str() {
            "check site" => check_site(),
            "info" => display_info(),
            "quit" => {
                println!("\nThank you for using HTTP Checker. Goodbye! 👋\n");
                break;
            }
            _ => println!("\n❌ Invalid option.\n"),
        }
    }
}
