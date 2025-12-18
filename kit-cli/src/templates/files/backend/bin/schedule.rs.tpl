//! Schedule runner binary
//!
//! This binary handles running scheduled tasks.
//!
//! # Usage
//!
//! ```bash
//! # Run all due tasks once (default)
//! cargo run --bin schedule
//! cargo run --bin schedule run
//!
//! # Run as daemon (checks every minute)
//! cargo run --bin schedule work
//!
//! # List all registered tasks
//! cargo run --bin schedule list
//!
//! # Run a specific task by name
//! cargo run --bin schedule run-task <task-name>
//! ```
//!
//! For production, use the Kit CLI commands instead:
//! ```bash
//! kit schedule:run
//! kit schedule:work
//! kit schedule:list
//! ```

mod bootstrap;
mod config;
mod schedule;
mod tasks;

// Import your models and actions as needed:
// mod actions;
// mod models;

use chrono::Local;
use kit::{Config, Schedule};
use std::env;
use std::path::Path;
use std::time::Duration;

#[tokio::main]
async fn main() {
    // Initialize framework configuration (loads .env files)
    Config::init(Path::new("."));

    // Register application configs
    config::register_all();

    // Bootstrap the application (database, services, etc.)
    // This gives tasks access to the same context as handlers
    bootstrap::register().await;

    let args: Vec<String> = env::args().collect();
    let command = args.get(1).map(|s| s.as_str()).unwrap_or("run");

    // Initialize the schedule
    let mut sched = Schedule::new();
    schedule::register(&mut sched);

    match command {
        "run" => {
            run_due_tasks(&sched).await;
        }
        "work" => {
            run_daemon(&sched).await;
        }
        "list" => {
            list_tasks(&sched);
        }
        "run-task" => {
            if let Some(task_name) = args.get(2) {
                run_specific_task(&sched, task_name).await;
            } else {
                eprintln!("Error: Please specify a task name");
                eprintln!("Usage: schedule run-task <task-name>");
                std::process::exit(1);
            }
        }
        _ => {
            eprintln!("Unknown command: {}", command);
            eprintln!();
            eprintln!("Usage: schedule <command>");
            eprintln!();
            eprintln!("Commands:");
            eprintln!("  run              Run all due tasks once");
            eprintln!("  work             Run as daemon (checks every minute)");
            eprintln!("  list             List all registered tasks");
            eprintln!("  run-task <name>  Run a specific task by name");
            std::process::exit(1);
        }
    }
}

async fn run_due_tasks(schedule: &Schedule) {
    let due_tasks = schedule.due_tasks();

    if due_tasks.is_empty() {
        println!("No tasks are due to run.");
        return;
    }

    println!("Running {} due task(s)...", due_tasks.len());
    println!();

    for task in due_tasks {
        print!("  [{}] Running '{}'... ", timestamp(), task.name);

        match task.run().await {
            Ok(()) => println!("OK"),
            Err(e) => println!("FAILED: {}", e),
        }
    }

    println!();
    println!("Done.");
}

async fn run_daemon(schedule: &Schedule) {
    println!("==============================================");
    println!("  Kit Scheduler Daemon");
    println!("==============================================");
    println!();
    println!("  Registered tasks: {}", schedule.len());
    println!("  Press Ctrl+C to stop");
    println!();
    println!("==============================================");
    println!();

    loop {
        let due_tasks = schedule.due_tasks();

        for task in due_tasks {
            println!("[{}] Running '{}'", timestamp(), task.name);

            if task.run_in_background {
                // Spawn task in background
                let task_name = task.name.clone();
                let task_ref = task.task.clone();
                tokio::spawn(async move {
                    if let Err(e) = task_ref.handle().await {
                        eprintln!("[{}] Task '{}' failed: {}", timestamp(), task_name, e);
                    } else {
                        println!("[{}] Task '{}' completed", timestamp(), task_name);
                    }
                });
            } else {
                // Run synchronously
                match task.run().await {
                    Ok(()) => println!("[{}] Task '{}' completed", timestamp(), task.name),
                    Err(e) => eprintln!("[{}] Task '{}' failed: {}", timestamp(), task.name, e),
                }
            }
        }

        // Sleep until the next minute
        let now = Local::now();
        let seconds_until_next_minute = 60 - now.format("%S").to_string().parse::<u64>().unwrap_or(0);
        tokio::time::sleep(Duration::from_secs(seconds_until_next_minute)).await;
    }
}

fn list_tasks(schedule: &Schedule) {
    let tasks = schedule.tasks();

    if tasks.is_empty() {
        println!("No scheduled tasks registered.");
        println!();
        println!("Create a task with: kit make:task <name>");
        return;
    }

    println!();
    println!("Scheduled Tasks:");
    println!("{}", "=".repeat(90));
    println!(
        "{:<30} {:<30} {}",
        "Name", "Schedule", "Description"
    );
    println!("{}", "-".repeat(90));

    for task in tasks {
        let desc = task.description.as_deref().unwrap_or("-");
        let schedule_str = task.expression.expression();

        // Truncate description if too long
        let desc_display = if desc.len() > 28 {
            format!("{}...", &desc[..25])
        } else {
            desc.to_string()
        };

        println!(
            "{:<30} {:<30} {}",
            task.name, schedule_str, desc_display
        );
    }

    println!("{}", "=".repeat(90));
    println!("Total: {} task(s)", tasks.len());
    println!();
}

async fn run_specific_task(schedule: &Schedule, task_name: &str) {
    match schedule.run_task(task_name).await {
        Some(Ok(())) => {
            println!("Task '{}' completed successfully.", task_name);
        }
        Some(Err(e)) => {
            eprintln!("Task '{}' failed: {}", task_name, e);
            std::process::exit(1);
        }
        None => {
            eprintln!("Task '{}' not found.", task_name);
            eprintln!();
            eprintln!("Available tasks:");
            for task in schedule.tasks() {
                eprintln!("  - {}", task.name);
            }
            std::process::exit(1);
        }
    }
}

fn timestamp() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}
