# Cancer CLI

A CLI tool for scaffolding Cancer web applications.

## Installation

```bash
cargo install cancer-cli
```

## Usage

### Create a new project

```bash
cancer new myapp
```

This will interactively prompt you for:
- Project name
- Description
- Author

### Non-interactive mode

```bash
cancer new myapp --no-interaction
```

### Skip git initialization

```bash
cancer new myapp --no-git
```

## Generated Project Structure

```
myapp/
├── Cargo.toml
├── .gitignore
└── src/
    ├── main.rs
    └── controllers/
        ├── mod.rs
        └── home.rs
```

## License

MIT
