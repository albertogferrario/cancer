//! MCP server command - start the Model Context Protocol server for AI-assisted development

use console::style;
use std::path::PathBuf;

pub fn run(cwd: Option<String>) {
    eprintln!(
        "{} Starting Ferro MCP server...",
        style("[MCP]").cyan().bold()
    );

    let project_root = cwd
        .map(PathBuf::from)
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));

    eprintln!(
        "{} Project root: {}",
        style("[MCP]").cyan().bold(),
        project_root.display()
    );

    let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");

    rt.block_on(async {
        let server = cancer_mcp::McpServer::with_project_root(project_root);

        if let Err(e) = server.run().await {
            eprintln!(
                "{} Failed to run MCP server: {}",
                style("[ERROR]").red().bold(),
                e
            );
            std::process::exit(1);
        }
    });
}
