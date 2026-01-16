//! Ferro MCP - Model Context Protocol server for AI-assisted Ferro Framework development

pub mod error;
pub mod introspection;
pub mod resources;
pub mod server;
pub mod service;
pub mod tools;

pub use server::McpServer;
