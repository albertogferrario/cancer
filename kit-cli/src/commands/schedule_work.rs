//! schedule:work command - Run the scheduler daemon

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

    println!("{} Starting scheduler daemon...", style("->").cyan());
    println!(
        "{}",
        style("Press Ctrl+C to stop").dim()
    );
    println!();

    // Run cargo run --bin schedule -- work
    let status = Command::new("cargo")
        .args(["run", "--bin", "schedule", "--quiet", "--", "work"])
        .status()
        .expect("Failed to execute cargo command");

    if !status.success() {
        // Exit code might be from Ctrl+C, which is expected
        // Only print error if it wasn't interrupted
        if let Some(code) = status.code() {
            if code != 130 {
                // 130 = interrupted by Ctrl+C
                eprintln!();
                eprintln!(
                    "{} Scheduler daemon exited with error (code: {})",
                    style("Error:").red().bold(),
                    code
                );
                std::process::exit(1);
            }
        }
    }

    println!();
    println!("{} Scheduler daemon stopped.", style("->").cyan());
}
