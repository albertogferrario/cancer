//! Cancer Application Entry Point

use cancer::{Config, Server};
use clap::{Parser, Subcommand};
use sea_orm_migration::prelude::*;
use std::env;
use std::path::Path;

mod actions;
mod bootstrap;
mod config;
mod controllers;
mod events;
mod jobs;
mod listeners;
mod middleware;
mod migrations;
mod models;
mod notifications;
mod routes;
mod tasks;

use migrations::Migrator;

#[derive(Parser)]
#[command(name = "{package_name}")]
#[command(about = "Cancer application server and utilities")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Run the web server (default command)
    Serve {
        /// Skip running migrations on startup
        #[arg(long)]
        no_migrate: bool,
    },
    /// Run pending database migrations
    Migrate,
    /// Show migration status
    #[command(name = "migrate:status")]
    MigrateStatus,
    /// Rollback the last migration(s)
    #[command(name = "migrate:rollback")]
    MigrateRollback {
        /// Number of migrations to rollback
        #[arg(default_value = "1")]
        steps: u32,
    },
    /// Drop all tables and re-run all migrations
    #[command(name = "migrate:fresh")]
    MigrateFresh,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    // Initialize framework configuration (loads .env files)
    Config::init(Path::new("."));

    // Register application configs
    config::register_all();

    match cli.command {
        None | Some(Commands::Serve { no_migrate: false }) => {
            run_migrations_silent().await;
            run_server().await;
        }
        Some(Commands::Serve { no_migrate: true }) => {
            run_server().await;
        }
        Some(Commands::Migrate) => {
            run_migrations().await;
        }
        Some(Commands::MigrateStatus) => {
            show_migration_status().await;
        }
        Some(Commands::MigrateRollback { steps }) => {
            rollback_migrations(steps).await;
        }
        Some(Commands::MigrateFresh) => {
            fresh_migrations().await;
        }
    }
}

async fn run_server() {
    bootstrap::register().await;
    let router = routes::register();
    Server::from_config(router)
        .run()
        .await
        .expect("Failed to start server");
}

async fn get_database_connection() -> sea_orm::DatabaseConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let database_url = if database_url.starts_with("sqlite://") {
        let path = database_url.trim_start_matches("sqlite://");
        let path = path.trim_start_matches("./");

        if let Some(parent) = Path::new(path).parent() {
            if !parent.as_os_str().is_empty() {
                std::fs::create_dir_all(parent).ok();
            }
        }

        if !Path::new(path).exists() {
            std::fs::File::create(path).ok();
        }

        format!("sqlite:{}?mode=rwc", path)
    } else {
        database_url
    };

    sea_orm::Database::connect(&database_url)
        .await
        .expect("Failed to connect to database")
}

async fn run_migrations_silent() {
    let db = get_database_connection().await;
    if let Err(e) = Migrator::up(&db, None).await {
        eprintln!("Warning: Migration failed: {}", e);
    }
}

async fn run_migrations() {
    println!("Running migrations...");
    let db = get_database_connection().await;
    Migrator::up(&db, None)
        .await
        .expect("Failed to run migrations");
    println!("Migrations completed successfully!");
}

async fn show_migration_status() {
    println!("Migration status:");
    let db = get_database_connection().await;
    Migrator::status(&db)
        .await
        .expect("Failed to get migration status");
}

async fn rollback_migrations(steps: u32) {
    println!("Rolling back {} migration(s)...", steps);
    let db = get_database_connection().await;
    Migrator::down(&db, Some(steps))
        .await
        .expect("Failed to rollback migrations");
    println!("Rollback completed successfully!");
}

async fn fresh_migrations() {
    println!("WARNING: Dropping all tables and re-running migrations...");
    let db = get_database_connection().await;
    Migrator::fresh(&db)
        .await
        .expect("Failed to refresh database");
    println!("Database refreshed successfully!");
}
