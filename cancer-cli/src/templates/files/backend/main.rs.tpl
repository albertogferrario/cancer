//! Cancer Application Entry Point

use cancer::Application;
use sea_orm_migration::prelude::MigratorTrait;

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
mod schedule;
mod tasks;

#[tokio::main]
async fn main() {
    Application::new()
        .config(config::register_all)
        .bootstrap(bootstrap::register)
        .routes(routes::register)
        .schedule(schedule::register)
        .migrations::<migrations::Migrator>()
        .run()
        .await;
}
