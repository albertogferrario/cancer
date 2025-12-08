[package]
name = "{package_name}"
version = "0.1.0"
edition = "2021"
description = "{description}"
{authors_line}
[[bin]]
name = "{package_name}"
path = "src/main.rs"

[[bin]]
name = "migrate"
path = "src/bin/migrate.rs"

[dependencies]
kit = {{ package = "kit-rs", version = "0.1" }}
tokio = {{ version = "1", features = ["full"] }}
sea-orm-migration = {{ version = "1.0", features = ["sqlx-sqlite", "sqlx-postgres", "runtime-tokio-native-tls"] }}
async-trait = "0.1"
dotenvy = "0.15"
