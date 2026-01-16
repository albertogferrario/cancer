mod database;
mod mail;

pub use database::DatabaseConfig;
pub use mail::MailConfig;

use ferro::{Config, DatabaseConfig as FerroDatabaseConfig};

/// Register all application configs
pub fn register_all() {
    // Use Cancer's built-in DatabaseConfig
    Config::register(CancerDatabaseConfig::from_env());
    Config::register(MailConfig::from_env());
}
