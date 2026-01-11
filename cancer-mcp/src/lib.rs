//! Cancer MCP - Model Context Protocol server for AI-assisted Cancer Framework development

pub mod error;
pub mod introspection;
pub mod server;
pub mod service;
pub mod tools;

pub use server::McpServer;
