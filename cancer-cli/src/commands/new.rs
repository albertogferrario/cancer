use console::style;
use dialoguer::{theme::ColorfulTheme, Input};
use std::fs;
use std::path::Path;
use std::process::Command;

use crate::templates;

pub fn run(name: Option<String>, no_interaction: bool, no_git: bool) {
    println!();
    println!("{}", style("Welcome to Cancer!").cyan().bold());
    println!();

    let project_name = get_project_name(name, no_interaction);
    let description = get_description(no_interaction);
    let author = get_author(no_interaction);

    let package_name = to_snake_case(&project_name);

    println!();
    println!(
        "{}",
        style(format!("Creating project '{}'...", project_name)).dim()
    );

    if let Err(e) = create_project(&project_name, &package_name, &description, &author, no_git) {
        eprintln!("{} {}", style("Error:").red().bold(), e);
        std::process::exit(1);
    }

    println!("{} Generated project structure", style("✓").green());

    if !no_git {
        println!("{} Initialized git repository", style("✓").green());
    }

    println!("{} Ready to go!", style("✓").green());
    println!();
    println!("Next steps:");
    println!("  {} {}", style("cd").cyan(), project_name);
    println!("  {}", style("cancer serve").cyan());
    println!();
    println!(
        "Backend will be at {}",
        style("http://localhost:8000").underlined()
    );
    println!(
        "Frontend dev server at {}",
        style("http://localhost:5173").underlined()
    );
    println!();
}

fn get_project_name(name: Option<String>, no_interaction: bool) -> String {
    if let Some(n) = name {
        return n;
    }

    if no_interaction {
        return "my-cancer-app".to_string();
    }

    Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Project name")
        .default("my-cancer-app".to_string())
        .interact_text()
        .unwrap()
}

fn get_description(no_interaction: bool) -> String {
    if no_interaction {
        return "A web application built with Cancer".to_string();
    }

    Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Description")
        .default("A web application built with Cancer".to_string())
        .interact_text()
        .unwrap()
}

fn get_author(no_interaction: bool) -> String {
    if no_interaction {
        return String::new();
    }

    let default_author = get_git_author().unwrap_or_default();

    Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Author")
        .default(default_author)
        .allow_empty(true)
        .interact_text()
        .unwrap()
}

fn get_git_author() -> Option<String> {
    let name = Command::new("git")
        .args(["config", "user.name"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())?;

    let email = Command::new("git")
        .args(["config", "user.email"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())?;

    Some(format!("{} <{}>", name, email))
}

fn to_snake_case(s: &str) -> String {
    s.replace('-', "_").to_lowercase()
}

fn to_title_case(s: &str) -> String {
    s.replace(['-', '_'], " ")
        .split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn create_project(
    project_name: &str,
    package_name: &str,
    description: &str,
    author: &str,
    no_git: bool,
) -> Result<(), String> {
    let project_path = Path::new(project_name);

    if project_path.exists() {
        return Err(format!("Directory '{}' already exists", project_name));
    }

    // Create directory structure
    // Backend directories
    fs::create_dir_all(project_path.join("src/controllers"))
        .map_err(|e| format!("Failed to create directories: {}", e))?;
    fs::create_dir_all(project_path.join("src/config"))
        .map_err(|e| format!("Failed to create directories: {}", e))?;
    fs::create_dir_all(project_path.join("src/middleware"))
        .map_err(|e| format!("Failed to create directories: {}", e))?;
    fs::create_dir_all(project_path.join("src/actions"))
        .map_err(|e| format!("Failed to create directories: {}", e))?;
    fs::create_dir_all(project_path.join("src/models"))
        .map_err(|e| format!("Failed to create directories: {}", e))?;
    fs::create_dir_all(project_path.join("src/migrations"))
        .map_err(|e| format!("Failed to create directories: {}", e))?;
    fs::create_dir_all(project_path.join("src/events"))
        .map_err(|e| format!("Failed to create directories: {}", e))?;
    fs::create_dir_all(project_path.join("src/listeners"))
        .map_err(|e| format!("Failed to create directories: {}", e))?;
    fs::create_dir_all(project_path.join("src/jobs"))
        .map_err(|e| format!("Failed to create directories: {}", e))?;
    fs::create_dir_all(project_path.join("src/notifications"))
        .map_err(|e| format!("Failed to create directories: {}", e))?;
    fs::create_dir_all(project_path.join("src/tasks"))
        .map_err(|e| format!("Failed to create directories: {}", e))?;
    fs::create_dir_all(project_path.join("src/seeders"))
        .map_err(|e| format!("Failed to create directories: {}", e))?;
    fs::create_dir_all(project_path.join("src/factories"))
        .map_err(|e| format!("Failed to create directories: {}", e))?;

    // Storage directories
    fs::create_dir_all(project_path.join("storage/app/public"))
        .map_err(|e| format!("Failed to create directories: {}", e))?;
    fs::create_dir_all(project_path.join("storage/logs"))
        .map_err(|e| format!("Failed to create directories: {}", e))?;

    // Frontend directories
    fs::create_dir_all(project_path.join("frontend/src/pages"))
        .map_err(|e| format!("Failed to create directories: {}", e))?;
    fs::create_dir_all(project_path.join("frontend/src/pages/auth"))
        .map_err(|e| format!("Failed to create directories: {}", e))?;
    fs::create_dir_all(project_path.join("frontend/src/types"))
        .map_err(|e| format!("Failed to create directories: {}", e))?;
    fs::create_dir_all(project_path.join("frontend/src/layouts"))
        .map_err(|e| format!("Failed to create directories: {}", e))?;
    fs::create_dir_all(project_path.join("frontend/src/styles"))
        .map_err(|e| format!("Failed to create directories: {}", e))?;

    // Public assets directory (for production builds)
    fs::create_dir_all(project_path.join("public/assets"))
        .map_err(|e| format!("Failed to create directories: {}", e))?;

    // === Backend files ===

    // Write Cargo.toml
    let cargo_toml = templates::cargo_toml(package_name, description, author);
    fs::write(project_path.join("Cargo.toml"), cargo_toml)
        .map_err(|e| format!("Failed to write Cargo.toml: {}", e))?;

    // Write .gitignore
    fs::write(project_path.join(".gitignore"), templates::gitignore())
        .map_err(|e| format!("Failed to write .gitignore: {}", e))?;

    // Write .env
    fs::write(project_path.join(".env"), templates::env(project_name))
        .map_err(|e| format!("Failed to write .env: {}", e))?;

    // Write .env.example
    fs::write(project_path.join(".env.example"), templates::env_example())
        .map_err(|e| format!("Failed to write .env.example: {}", e))?;

    // Write src/main.rs
    fs::write(
        project_path.join("src/main.rs"),
        templates::main_rs(package_name),
    )
    .map_err(|e| format!("Failed to write src/main.rs: {}", e))?;

    // Write src/routes.rs
    fs::write(project_path.join("src/routes.rs"), templates::routes_rs())
        .map_err(|e| format!("Failed to write src/routes.rs: {}", e))?;

    // Write src/controllers/mod.rs
    fs::write(
        project_path.join("src/controllers/mod.rs"),
        templates::controllers_mod(),
    )
    .map_err(|e| format!("Failed to write src/controllers/mod.rs: {}", e))?;

    // Write src/controllers/home.rs
    fs::write(
        project_path.join("src/controllers/home.rs"),
        templates::home_controller(),
    )
    .map_err(|e| format!("Failed to write src/controllers/home.rs: {}", e))?;

    // Write src/controllers/auth.rs
    fs::write(
        project_path.join("src/controllers/auth.rs"),
        templates::auth_controller(),
    )
    .map_err(|e| format!("Failed to write src/controllers/auth.rs: {}", e))?;

    // Write src/controllers/dashboard.rs
    fs::write(
        project_path.join("src/controllers/dashboard.rs"),
        templates::dashboard_controller(),
    )
    .map_err(|e| format!("Failed to write src/controllers/dashboard.rs: {}", e))?;

    // Write src/controllers/profile.rs
    fs::write(
        project_path.join("src/controllers/profile.rs"),
        templates::profile_controller(),
    )
    .map_err(|e| format!("Failed to write src/controllers/profile.rs: {}", e))?;

    // Write src/controllers/settings.rs
    fs::write(
        project_path.join("src/controllers/settings.rs"),
        templates::settings_controller(),
    )
    .map_err(|e| format!("Failed to write src/controllers/settings.rs: {}", e))?;

    // Write src/config/mod.rs
    fs::write(
        project_path.join("src/config/mod.rs"),
        templates::config_mod(),
    )
    .map_err(|e| format!("Failed to write src/config/mod.rs: {}", e))?;

    // Write src/config/database.rs
    fs::write(
        project_path.join("src/config/database.rs"),
        templates::config_database(),
    )
    .map_err(|e| format!("Failed to write src/config/database.rs: {}", e))?;

    // Write src/config/mail.rs
    fs::write(
        project_path.join("src/config/mail.rs"),
        templates::config_mail(),
    )
    .map_err(|e| format!("Failed to write src/config/mail.rs: {}", e))?;

    // Write src/middleware/mod.rs
    fs::write(
        project_path.join("src/middleware/mod.rs"),
        templates::middleware_mod(),
    )
    .map_err(|e| format!("Failed to write src/middleware/mod.rs: {}", e))?;

    // Write src/middleware/logging.rs
    fs::write(
        project_path.join("src/middleware/logging.rs"),
        templates::middleware_logging(),
    )
    .map_err(|e| format!("Failed to write src/middleware/logging.rs: {}", e))?;

    // Write src/middleware/authenticate.rs
    fs::write(
        project_path.join("src/middleware/authenticate.rs"),
        templates::authenticate_middleware(),
    )
    .map_err(|e| format!("Failed to write src/middleware/authenticate.rs: {}", e))?;

    // Write src/bootstrap.rs
    fs::write(
        project_path.join("src/bootstrap.rs"),
        templates::bootstrap(),
    )
    .map_err(|e| format!("Failed to write src/bootstrap.rs: {}", e))?;

    // Write src/actions/mod.rs
    fs::write(
        project_path.join("src/actions/mod.rs"),
        templates::actions_mod(),
    )
    .map_err(|e| format!("Failed to write src/actions/mod.rs: {}", e))?;

    // Write src/actions/example_action.rs
    fs::write(
        project_path.join("src/actions/example_action.rs"),
        templates::example_action(),
    )
    .map_err(|e| format!("Failed to write src/actions/example_action.rs: {}", e))?;

    // Write src/models/mod.rs
    fs::write(
        project_path.join("src/models/mod.rs"),
        templates::models_mod(),
    )
    .map_err(|e| format!("Failed to write src/models/mod.rs: {}", e))?;

    // Write src/models/user.rs
    fs::write(
        project_path.join("src/models/user.rs"),
        templates::user_model(),
    )
    .map_err(|e| format!("Failed to write src/models/user.rs: {}", e))?;

    // Write src/models/password_reset_tokens.rs
    fs::write(
        project_path.join("src/models/password_reset_tokens.rs"),
        templates::password_reset_tokens_model(),
    )
    .map_err(|e| format!("Failed to write src/models/password_reset_tokens.rs: {}", e))?;

    // Write src/migrations/mod.rs
    fs::write(
        project_path.join("src/migrations/mod.rs"),
        templates::migrations_mod(),
    )
    .map_err(|e| format!("Failed to write src/migrations/mod.rs: {}", e))?;

    // Write auth migration files
    fs::write(
        project_path.join("src/migrations/m20240101_000001_create_users_table.rs"),
        templates::create_users_migration(),
    )
    .map_err(|e| format!("Failed to write create_users_table migration: {}", e))?;

    fs::write(
        project_path.join("src/migrations/m20240101_000002_create_sessions_table.rs"),
        templates::create_sessions_migration(),
    )
    .map_err(|e| format!("Failed to write create_sessions_table migration: {}", e))?;

    fs::write(
        project_path.join("src/migrations/m20240101_000003_create_password_reset_tokens_table.rs"),
        templates::create_password_reset_tokens_migration(),
    )
    .map_err(|e| format!("Failed to write create_password_reset_tokens_table migration: {}", e))?;

    // Note: migrations are now integrated into the main binary
    // Run with: ./app migrate

    // === Events, Listeners, Jobs, Notifications, Tasks ===

    // Write src/events/mod.rs
    fs::write(
        project_path.join("src/events/mod.rs"),
        templates::events_mod(),
    )
    .map_err(|e| format!("Failed to write src/events/mod.rs: {}", e))?;

    // Write src/listeners/mod.rs
    fs::write(
        project_path.join("src/listeners/mod.rs"),
        templates::listeners_mod(),
    )
    .map_err(|e| format!("Failed to write src/listeners/mod.rs: {}", e))?;

    // Write src/jobs/mod.rs
    fs::write(project_path.join("src/jobs/mod.rs"), templates::jobs_mod())
        .map_err(|e| format!("Failed to write src/jobs/mod.rs: {}", e))?;

    // Write src/notifications/mod.rs
    fs::write(
        project_path.join("src/notifications/mod.rs"),
        templates::notifications_mod(),
    )
    .map_err(|e| format!("Failed to write src/notifications/mod.rs: {}", e))?;

    // Write src/tasks/mod.rs
    fs::write(
        project_path.join("src/tasks/mod.rs"),
        templates::tasks_mod(),
    )
    .map_err(|e| format!("Failed to write src/tasks/mod.rs: {}", e))?;

    // Write src/seeders/mod.rs
    fs::write(
        project_path.join("src/seeders/mod.rs"),
        templates::seeders_mod(),
    )
    .map_err(|e| format!("Failed to write src/seeders/mod.rs: {}", e))?;

    // Write src/factories/mod.rs
    fs::write(
        project_path.join("src/factories/mod.rs"),
        templates::factories_mod(),
    )
    .map_err(|e| format!("Failed to write src/factories/mod.rs: {}", e))?;

    // Write src/schedule.rs
    fs::write(
        project_path.join("src/schedule.rs"),
        templates::schedule_rs(),
    )
    .map_err(|e| format!("Failed to write src/schedule.rs: {}", e))?;

    // Write storage/.gitkeep files
    fs::write(project_path.join("storage/app/.gitkeep"), "")
        .map_err(|e| format!("Failed to write storage/app/.gitkeep: {}", e))?;

    fs::write(project_path.join("storage/logs/.gitkeep"), "")
        .map_err(|e| format!("Failed to write storage/logs/.gitkeep: {}", e))?;

    // === Frontend files ===

    // Write frontend/package.json
    let package_json = templates::package_json(project_name);
    fs::write(project_path.join("frontend/package.json"), package_json)
        .map_err(|e| format!("Failed to write frontend/package.json: {}", e))?;

    // Write frontend/vite.config.ts
    fs::write(
        project_path.join("frontend/vite.config.ts"),
        templates::vite_config(),
    )
    .map_err(|e| format!("Failed to write frontend/vite.config.ts: {}", e))?;

    // Write frontend/tsconfig.json
    fs::write(
        project_path.join("frontend/tsconfig.json"),
        templates::tsconfig(),
    )
    .map_err(|e| format!("Failed to write frontend/tsconfig.json: {}", e))?;

    // Write frontend/index.html
    let title = to_title_case(project_name);
    let index_html = templates::index_html(&title);
    fs::write(project_path.join("frontend/index.html"), index_html)
        .map_err(|e| format!("Failed to write frontend/index.html: {}", e))?;

    // Write frontend/src/main.tsx
    fs::write(
        project_path.join("frontend/src/main.tsx"),
        templates::main_tsx(),
    )
    .map_err(|e| format!("Failed to write frontend/src/main.tsx: {}", e))?;

    // Write frontend/src/pages/Home.tsx
    fs::write(
        project_path.join("frontend/src/pages/Home.tsx"),
        templates::home_page(),
    )
    .map_err(|e| format!("Failed to write frontend/src/pages/Home.tsx: {}", e))?;

    // Write frontend/src/pages/auth/Login.tsx
    fs::write(
        project_path.join("frontend/src/pages/auth/Login.tsx"),
        templates::login_page(),
    )
    .map_err(|e| format!("Failed to write frontend/src/pages/auth/Login.tsx: {}", e))?;

    // Write frontend/src/pages/auth/Register.tsx
    fs::write(
        project_path.join("frontend/src/pages/auth/Register.tsx"),
        templates::register_page(),
    )
    .map_err(|e| {
        format!(
            "Failed to write frontend/src/pages/auth/Register.tsx: {}",
            e
        )
    })?;

    // Write frontend/src/pages/Dashboard.tsx
    fs::write(
        project_path.join("frontend/src/pages/Dashboard.tsx"),
        templates::dashboard_page(),
    )
    .map_err(|e| format!("Failed to write frontend/src/pages/Dashboard.tsx: {}", e))?;

    // Write frontend/src/types/inertia-props.ts
    fs::write(
        project_path.join("frontend/src/types/inertia-props.ts"),
        templates::inertia_props_types(),
    )
    .map_err(|e| format!("Failed to write frontend/src/types/inertia-props.ts: {}", e))?;

    // Write frontend/src/layouts/AppLayout.tsx
    fs::write(
        project_path.join("frontend/src/layouts/AppLayout.tsx"),
        templates::app_layout(),
    )
    .map_err(|e| format!("Failed to write frontend/src/layouts/AppLayout.tsx: {}", e))?;

    // Write frontend/src/layouts/AuthLayout.tsx
    fs::write(
        project_path.join("frontend/src/layouts/AuthLayout.tsx"),
        templates::auth_layout(),
    )
    .map_err(|e| format!("Failed to write frontend/src/layouts/AuthLayout.tsx: {}", e))?;

    // Write frontend/src/layouts/index.ts
    fs::write(
        project_path.join("frontend/src/layouts/index.ts"),
        templates::layouts_index(),
    )
    .map_err(|e| format!("Failed to write frontend/src/layouts/index.ts: {}", e))?;

    // Write frontend/src/styles/globals.css
    fs::write(
        project_path.join("frontend/src/styles/globals.css"),
        templates::globals_css(),
    )
    .map_err(|e| format!("Failed to write frontend/src/styles/globals.css: {}", e))?;

    // Write frontend/src/pages/auth/ForgotPassword.tsx
    fs::write(
        project_path.join("frontend/src/pages/auth/ForgotPassword.tsx"),
        templates::forgot_password_page(),
    )
    .map_err(|e| format!("Failed to write frontend/src/pages/auth/ForgotPassword.tsx: {}", e))?;

    // Write frontend/src/pages/auth/ResetPassword.tsx
    fs::write(
        project_path.join("frontend/src/pages/auth/ResetPassword.tsx"),
        templates::reset_password_page(),
    )
    .map_err(|e| format!("Failed to write frontend/src/pages/auth/ResetPassword.tsx: {}", e))?;

    // Write frontend/src/pages/Profile.tsx
    fs::write(
        project_path.join("frontend/src/pages/Profile.tsx"),
        templates::profile_page(),
    )
    .map_err(|e| format!("Failed to write frontend/src/pages/Profile.tsx: {}", e))?;

    // Write frontend/src/pages/Settings.tsx
    fs::write(
        project_path.join("frontend/src/pages/Settings.tsx"),
        templates::settings_page(),
    )
    .map_err(|e| format!("Failed to write frontend/src/pages/Settings.tsx: {}", e))?;

    // Initialize git repository
    if !no_git {
        Command::new("git")
            .args(["init"])
            .current_dir(project_path)
            .output()
            .map_err(|e| format!("Failed to initialize git repository: {}", e))?;
    }

    Ok(())
}
