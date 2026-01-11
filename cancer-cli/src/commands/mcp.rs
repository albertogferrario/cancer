//! MCP server command - start the Model Context Protocol server for AI-assisted development

use console::style;

pub fn run() {
    println!(
        "{} Starting Cancer MCP server...",
        style("[MCP]").cyan().bold()
    );

    let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");

    rt.block_on(async {
        let server = cancer_mcp::McpServer::new();

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
