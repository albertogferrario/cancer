# ferro-mcp

MCP (Model Context Protocol) server for AI-assisted Ferro framework development.

Similar to Laravel Boost, this crate provides introspection tools that AI agents can use to understand and work with Ferro applications.

## Features

- Application introspection
- Route listing
- Database schema inspection
- Log reading
- Interactive REPL (tinker)

## Available Tools

| Tool | Description |
|------|-------------|
| `application_info` | Framework version, environment, installed crates, models |
| `list_routes` | All registered routes with methods, paths, handlers |
| `list_middleware` | Global and route middleware chain |
| `list_migrations` | Database migrations with status |
| `list_jobs` | Background jobs defined in the app |
| `list_events` | Events and their listeners |
| `list_commands` | Available CLI commands |
| `database_schema` | Table schemas and relationships |
| `database_query` | Execute SQL queries |
| `read_logs` | Application log output |
| `last_error` | Most recent error with stack trace |
| `get_config` | Configuration values |
| `search_docs` | Search framework documentation |
| `tinker` | Interactive REPL for testing |
| `generate_types` | Generate TypeScript types from Rust |

## Usage

### Start MCP Server

```bash
ferro mcp
```

### Install for AI Editor

```bash
ferro boost:install
```

This configures the MCP server for your editor (Cursor, VS Code, Claude).

### Manual Configuration

Add to your MCP configuration:

```json
{
  "mcpServers": {
    "ferro": {
      "command": "cargo",
      "args": ["run", "--package", "ferro-mcp"],
      "cwd": "/path/to/your/project"
    }
  }
}
```

## Example Tool Output

### `application_info`

```json
{
  "framework_version": "0.1.0",
  "rust_version": "rustc 1.75.0",
  "database_engine": "sqlite",
  "environment": "local",
  "installed_crates": [
    {"name": "ferro-rs", "version": "0.1.0"},
    {"name": "ferro-events", "version": "0.1.0"}
  ],
  "models": [
    {"name": "User", "table": "users", "path": "src/models/users.rs"}
  ]
}
```

### `list_routes`

```json
{
  "routes": [
    {"method": "GET", "path": "/users", "handler": "users_controller::index"},
    {"method": "POST", "path": "/users", "handler": "users_controller::store"}
  ]
}
```

## License

MIT
