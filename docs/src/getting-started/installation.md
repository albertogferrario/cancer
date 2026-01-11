# Installation

## Requirements

- Rust 1.75+ (with Cargo)
- Node.js 18+ (for frontend)
- PostgreSQL, SQLite, or MySQL

## Installing the CLI

Install the Cancer CLI globally:

```bash
cargo install cancer-cli
```

Or build from source:

```bash
git clone https://github.com/albertogferrario/cancer.git
cd cancer
cargo install --path cancer-cli
```

## Creating a New Project

```bash
cancer new my-app
```

This will:
1. Create a new directory `my-app`
2. Initialize a Rust workspace
3. Set up the frontend with React and TypeScript
4. Configure the database
5. Initialize git repository

### Options

```bash
# Skip interactive prompts
cancer new my-app --no-interaction

# Skip git initialization
cancer new my-app --no-git
```

## Starting Development

```bash
cd my-app
cancer serve
```

This starts both the backend (port 8000) and frontend (port 5173) servers.

### Server Options

```bash
# Custom ports
cancer serve --port 3000 --frontend-port 3001

# Backend only
cancer serve --backend-only

# Frontend only
cancer serve --frontend-only

# Skip TypeScript generation
cancer serve --skip-types
```

## AI Development Setup

For AI-assisted development with Claude, Cursor, or VS Code:

```bash
cancer boost:install
```

This configures the MCP server and adds project guidelines for your editor.

## Next Steps

- [Quick Start](quickstart.md) - Build your first feature
- [Directory Structure](directory-structure.md) - Understand the project layout
