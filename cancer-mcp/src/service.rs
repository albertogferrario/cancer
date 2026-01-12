//! MCP Service implementation with tool handlers

use crate::tools;
use rmcp::{
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::ServerInfo,
    tool, tool_handler, tool_router, ServerHandler,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Cancer MCP Service that handles tool requests
#[derive(Clone)]
pub struct CancerMcpService {
    project_root: PathBuf,
    tool_router: ToolRouter<Self>,
}

impl CancerMcpService {
    pub fn new(project_root: PathBuf) -> Self {
        Self {
            project_root,
            tool_router: Self::tool_router(),
        }
    }
}

// Tool request types

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct DbQueryParams {
    /// SQL query to execute (SELECT only for safety)
    pub query: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct DbSchemaParams {
    /// Optional table name to filter (returns all if omitted)
    pub table: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct ReadLogsParams {
    /// Number of lines to read (default: 50)
    pub lines: Option<usize>,
    /// Log level filter: debug, info, warn, error
    pub level: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct GetConfigParams {
    /// Configuration key filter
    pub key: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct GenerateTypesParams {
    /// Output file path (optional)
    pub output: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct SearchDocsParams {
    /// Search query
    pub query: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct GetHandlerParams {
    /// Route path to get handler for (e.g., "/animali/{id}")
    pub route: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct TinkerParams {
    /// Rust code to execute
    pub code: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct BrowserLogsParams {
    /// Number of lines to read (default: 50)
    pub lines: Option<usize>,
    /// Log level filter: error, warn, info
    pub level: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct SessionInspectParams {
    /// Optional session ID to inspect (shows all recent sessions if omitted)
    pub session_id: Option<String>,
}

#[tool_router(router = tool_router)]
impl CancerMcpService {
    /// Get application information including framework version, Rust version, models, and installed crates
    #[tool(
        name = "application_info",
        description = "Get application information including framework version, Rust version, models, and installed crates"
    )]
    pub async fn application_info(&self) -> String {
        match tools::application_info::execute(&self.project_root) {
            Ok(info) => serde_json::to_string_pretty(&info).unwrap_or_else(|_| "{}".to_string()),
            Err(e) => format!("{{\"error\": \"{}\"}}", e),
        }
    }

    /// Execute a read-only SQL query against the database
    #[tool(
        name = "db_query",
        description = "Execute a read-only SQL query against the database"
    )]
    pub async fn db_query(&self, params: Parameters<DbQueryParams>) -> String {
        match tools::database_query::execute(&self.project_root, &params.0.query).await {
            Ok(data) => serde_json::to_string_pretty(&data).unwrap_or_else(|_| "{}".to_string()),
            Err(e) => format!("{{\"error\": \"{}\"}}", e),
        }
    }

    /// Get database schema information (tables, columns, types)
    #[tool(
        name = "db_schema",
        description = "Get database schema information (tables, columns, types)"
    )]
    pub async fn db_schema(&self, params: Parameters<DbSchemaParams>) -> String {
        match tools::database_schema::execute(&self.project_root, params.0.table.as_deref()).await {
            Ok(schema) => {
                serde_json::to_string_pretty(&schema).unwrap_or_else(|_| "{}".to_string())
            }
            Err(e) => format!("{{\"error\": \"{}\"}}", e),
        }
    }

    /// List all routes defined in the application
    #[tool(
        name = "list_routes",
        description = "List all routes defined in the application"
    )]
    pub async fn list_routes(&self) -> String {
        match tools::list_routes::execute(&self.project_root) {
            Ok(routes) => {
                serde_json::to_string_pretty(&routes).unwrap_or_else(|_| "[]".to_string())
            }
            Err(e) => format!("{{\"error\": \"{}\"}}", e),
        }
    }

    /// List all available CLI commands
    #[tool(
        name = "list_commands",
        description = "List all available CLI commands"
    )]
    pub async fn list_commands(&self) -> String {
        let result = tools::list_commands::execute();
        serde_json::to_string_pretty(&result).unwrap_or_else(|_| "[]".to_string())
    }

    /// Show migration status (applied and pending migrations)
    #[tool(
        name = "list_migrations",
        description = "Show migration status (applied and pending migrations)"
    )]
    pub async fn list_migrations(&self) -> String {
        match tools::list_migrations::execute(&self.project_root).await {
            Ok(migrations) => {
                serde_json::to_string_pretty(&migrations).unwrap_or_else(|_| "[]".to_string())
            }
            Err(e) => format!("{{\"error\": \"{}\"}}", e),
        }
    }

    /// List all registered events and their listeners
    #[tool(
        name = "list_events",
        description = "List all registered events and their listeners"
    )]
    pub async fn list_events(&self) -> String {
        match tools::list_events::execute(&self.project_root) {
            Ok(events) => {
                serde_json::to_string_pretty(&events).unwrap_or_else(|_| "[]".to_string())
            }
            Err(e) => format!("{{\"error\": \"{}\"}}", e),
        }
    }

    /// List all defined background jobs
    #[tool(name = "list_jobs", description = "List all defined background jobs")]
    pub async fn list_jobs(&self) -> String {
        match tools::list_jobs::execute(&self.project_root) {
            Ok(jobs) => serde_json::to_string_pretty(&jobs).unwrap_or_else(|_| "[]".to_string()),
            Err(e) => format!("{{\"error\": \"{}\"}}", e),
        }
    }

    /// List all registered middleware
    #[tool(
        name = "list_middleware",
        description = "List all registered middleware"
    )]
    pub async fn list_middleware(&self) -> String {
        match tools::list_middleware::execute(&self.project_root) {
            Ok(middleware) => {
                serde_json::to_string_pretty(&middleware).unwrap_or_else(|_| "[]".to_string())
            }
            Err(e) => format!("{{\"error\": \"{}\"}}", e),
        }
    }

    /// List all database models with their fields and types
    #[tool(
        name = "list_models",
        description = "List all database models with their fields, types, and relationships"
    )]
    pub async fn list_models(&self) -> String {
        match tools::list_models::execute(&self.project_root) {
            Ok(models) => {
                serde_json::to_string_pretty(&models).unwrap_or_else(|_| "[]".to_string())
            }
            Err(e) => format!("{{\"error\": \"{}\"}}", e),
        }
    }

    /// Get handler source code for a specific route
    #[tool(
        name = "get_handler",
        description = "Get the source code of a handler function for a given route path"
    )]
    pub async fn get_handler(&self, params: Parameters<GetHandlerParams>) -> String {
        match tools::get_handler::execute(&self.project_root, &params.0.route) {
            Ok(handler) => {
                serde_json::to_string_pretty(&handler).unwrap_or_else(|_| "{}".to_string())
            }
            Err(e) => format!("{{\"error\": \"{}\"}}", e),
        }
    }

    /// Read recent log entries
    #[tool(name = "read_logs", description = "Read recent log entries")]
    pub async fn read_logs(&self, params: Parameters<ReadLogsParams>) -> String {
        match tools::read_logs::execute(
            &self.project_root,
            params.0.lines.unwrap_or(50),
            params.0.level.as_deref(),
        ) {
            Ok(logs) => serde_json::to_string_pretty(&logs).unwrap_or_else(|_| "[]".to_string()),
            Err(e) => format!("{{\"error\": \"{}\"}}", e),
        }
    }

    /// Get the most recent error from logs
    #[tool(
        name = "last_error",
        description = "Get the most recent error from logs"
    )]
    pub async fn last_error(&self) -> String {
        match tools::last_error::execute(&self.project_root) {
            Ok(error) => {
                serde_json::to_string_pretty(&error).unwrap_or_else(|_| "null".to_string())
            }
            Err(e) => format!("{{\"error\": \"{}\"}}", e),
        }
    }

    /// Read configuration values
    #[tool(name = "get_config", description = "Read configuration values")]
    pub async fn get_config(&self, params: Parameters<GetConfigParams>) -> String {
        match tools::get_config::execute(&self.project_root, params.0.key.as_deref()) {
            Ok(config) => {
                serde_json::to_string_pretty(&config).unwrap_or_else(|_| "{}".to_string())
            }
            Err(e) => format!("{{\"error\": \"{}\"}}", e),
        }
    }

    /// Trigger TypeScript type generation
    #[tool(
        name = "generate_types",
        description = "Trigger TypeScript type generation"
    )]
    pub async fn generate_types(&self, params: Parameters<GenerateTypesParams>) -> String {
        match tools::generate_types::execute(&self.project_root, params.0.output.as_deref()) {
            Ok(info) => serde_json::to_string_pretty(&info).unwrap_or_else(|_| "{}".to_string()),
            Err(e) => format!("{{\"error\": \"{}\"}}", e),
        }
    }

    /// Search framework documentation
    #[tool(name = "search_docs", description = "Search framework documentation")]
    pub async fn search_docs(&self, params: Parameters<SearchDocsParams>) -> String {
        match tools::search_docs::execute(&self.project_root, &params.0.query) {
            Ok(results) => {
                serde_json::to_string_pretty(&results).unwrap_or_else(|_| "[]".to_string())
            }
            Err(e) => format!("{{\"error\": \"{}\"}}", e),
        }
    }

    /// Execute Rust code within the application context (like Laravel Tinker)
    #[tool(
        name = "tinker",
        description = "Execute Rust code within the application context (like Laravel Tinker)"
    )]
    pub async fn tinker(&self, params: Parameters<TinkerParams>) -> String {
        match tools::tinker::execute(&self.project_root, &params.0.code) {
            Ok(result) => {
                serde_json::to_string_pretty(&result).unwrap_or_else(|_| "{}".to_string())
            }
            Err(e) => format!("{{\"error\": \"{}\"}}", e),
        }
    }

    /// Read browser/frontend error logs
    #[tool(
        name = "browser_logs",
        description = "Read browser/frontend error logs from the application"
    )]
    pub async fn browser_logs(&self, params: Parameters<BrowserLogsParams>) -> String {
        match tools::browser_logs::execute(
            &self.project_root,
            params.0.lines.unwrap_or(50),
            params.0.level.as_deref(),
        ) {
            Ok(logs) => serde_json::to_string_pretty(&logs).unwrap_or_else(|_| "[]".to_string()),
            Err(e) => format!("{{\"error\": \"{}\"}}", e),
        }
    }

    /// Inspect active sessions in the database for debugging authentication issues
    #[tool(
        name = "session_inspect",
        description = "Inspect active sessions in the database. Shows session IDs, user IDs, and payload data. Useful for debugging authentication issues like session not persisting after login."
    )]
    pub async fn session_inspect(&self, params: Parameters<SessionInspectParams>) -> String {
        match tools::session_inspect::execute(&self.project_root, params.0.session_id.as_deref())
            .await
        {
            Ok(sessions) => {
                serde_json::to_string_pretty(&sessions).unwrap_or_else(|_| "{}".to_string())
            }
            Err(e) => format!("{{\"error\": \"{}\"}}", e),
        }
    }
}

#[tool_handler(router = self.tool_router)]
impl ServerHandler for CancerMcpService {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some(CANCER_MCP_INSTRUCTIONS.to_string()),
            ..Default::default()
        }
    }
}

/// Detailed instructions for AI agents on how to use Cancer MCP tools effectively.
const CANCER_MCP_INSTRUCTIONS: &str = r#"
Cancer Framework MCP server. Provides introspection tools for AI-assisted development of Cancer Rust web applications.

## About Cancer Framework

Cancer is a Laravel-inspired web framework for Rust featuring:
- SeaORM for database (models in src/models/, migrations in src/migrations/)
- Inertia.js for full-stack React/TypeScript (frontend in frontend/src/pages/)
- Background jobs via cancer-queue (jobs in src/jobs/)
- Event system via cancer-events (events in src/events/, listeners in src/listeners/)
- Multi-channel notifications (src/notifications/)
- WebSocket broadcasting (cancer-broadcast)
- File storage abstraction (cancer-storage)
- Caching with tags (cancer-cache)

## When to Use These Tools (PROACTIVELY)

**USE application_info FIRST** when starting work on a Cancer project to understand:
- Framework version and installed crates
- Available models and their structure
- Project configuration

**USE list_routes** when:
- User asks about API endpoints or URLs
- You need to understand the application structure
- Debugging routing issues
- Before adding new routes (to check for conflicts)

**USE list_models** when:
- Working with database operations
- User mentions a model name
- Creating migrations or seeders
- Understanding data relationships

**USE db_schema** when:
- You need exact column names and types
- Debugging database queries
- Planning migrations

**USE db_query** when:
- User asks to check data in the database
- Debugging data issues
- Verifying migration results

**USE get_handler** when:
- User asks how a specific endpoint works
- Debugging controller logic
- Understanding request/response flow

**USE list_migrations** when:
- User asks about database state
- Before creating new migrations
- Debugging migration order issues

**USE list_events and list_jobs** when:
- Understanding async workflows
- Debugging event/listener connections
- Planning new features

**USE read_logs and last_error** when:
- User reports an error
- Debugging runtime issues
- ALWAYS check logs when something isn't working

**USE session_inspect** when:
- Debugging authentication issues
- Session not persisting after login
- User reports being logged out unexpectedly

**USE get_config** when:
- Understanding environment configuration
- Debugging connection issues (database, cache, queue)

**USE search_docs** when:
- User asks "how do I..." questions
- Looking up Cancer-specific APIs
- Finding examples

## Tool Categories

### Introspection (understand the app)
- application_info: Start here - get overview
- list_routes: All HTTP endpoints
- list_models: Database models and fields
- list_middleware: Request pipeline
- list_events: Event/listener mappings
- list_jobs: Background job definitions
- list_migrations: Database migration status

### Database (query and inspect)
- db_schema: Table structures
- db_query: Execute SELECT queries
- session_inspect: Debug sessions

### Debugging (find problems)
- read_logs: Application logs
- last_error: Most recent error
- browser_logs: Frontend errors
- get_handler: Handler source code

### Development (generate and configure)
- generate_types: Create TypeScript types
- get_config: Read configuration
- search_docs: Find documentation
- list_commands: Available CLI commands
"#;
