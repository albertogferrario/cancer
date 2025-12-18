//! schedule:list command - Display all registered scheduled tasks

use console::style;
use std::path::Path;
use std::process::Command;

pub fn run() {
    // Check we're in a Kit project
    if !Path::new("src/schedule.rs").exists() {
        eprintln!(
            "{} No schedule.rs found at src/schedule.rs",
            style("Error:").red().bold()
        );
        eprintln!(
            "{}",
            style("Run 'kit make:task <name>' to create your first scheduled task.").dim()
        );
        std::process::exit(1);
    }

    // Check if schedule binary exists
    if !Path::new("src/bin/schedule.rs").exists() {
        eprintln!(
            "{} Schedule binary not found at src/bin/schedule.rs",
            style("Error:").red().bold()
        );
        eprintln!(
            "{}",
            style("Run 'kit make:task <name>' to set up the scheduler.").dim()
        );
        std::process::exit(1);
    }

    // Run cargo run --bin schedule -- list
    let status = Command::new("cargo")
        .args(["run", "--bin", "schedule", "--quiet", "--", "list"])
        .status()
        .expect("Failed to execute cargo command");

    if !status.success() {
        eprintln!();
        eprintln!(
            "{} Failed to list scheduled tasks",
            style("Error:").red().bold()
        );
        std::process::exit(1);
    }
}
