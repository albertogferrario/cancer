//! Get handler tool - returns the source code of a handler function

use crate::error::{McpError, Result};
use crate::tools::list_routes;
use serde::Serialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize)]
pub struct HandlerInfo {
    pub handler: String,
    pub path: String,
    pub method: String,
    pub file_path: String,
    pub source_code: String,
    pub line_start: usize,
    pub line_end: usize,
}

pub fn execute(project_root: &Path, route_path: &str) -> Result<HandlerInfo> {
    // First find the handler for this route
    let routes = list_routes::execute(project_root)?;

    let route = routes
        .routes
        .iter()
        .find(|r| r.path == route_path)
        .ok_or_else(|| McpError::NotFound(format!("Route not found: {}", route_path)))?;

    let handler = &route.handler;

    // Parse handler path like "controllers::shelter::animals::show"
    let parts: Vec<&str> = handler.split("::").collect();
    if parts.len() < 2 {
        return Err(McpError::ParseError(format!(
            "Invalid handler path: {}",
            handler
        )));
    }

    // Build the file path
    // controllers::shelter::animals::show -> src/controllers/shelter/animals.rs
    let file_parts: Vec<&str> = parts[..parts.len() - 1].to_vec();
    let function_name = parts[parts.len() - 1];

    let file_path = project_root
        .join("src")
        .join(file_parts.join("/"))
        .with_extension("rs");

    if !file_path.exists() {
        // Try as directory with mod.rs
        let mod_path = project_root
            .join("src")
            .join(file_parts.join("/"))
            .join("mod.rs");

        if mod_path.exists() {
            return extract_handler(&mod_path, function_name, handler, route, project_root);
        }

        return Err(McpError::FileNotFound(format!(
            "Handler file not found: {}",
            file_path.display()
        )));
    }

    extract_handler(&file_path, function_name, handler, route, project_root)
}

fn extract_handler(
    file_path: &Path,
    function_name: &str,
    handler: &str,
    route: &list_routes::RouteInfo,
    project_root: &Path,
) -> Result<HandlerInfo> {
    let content = fs::read_to_string(file_path)
        .map_err(|e| McpError::FileReadError(format!("{}: {}", file_path.display(), e)))?;

    let lines: Vec<&str> = content.lines().collect();

    // Find the function with #[handler] attribute
    let mut line_start = None;
    let mut in_handler = false;
    let mut brace_count = 0;

    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();

        // Look for #[handler] attribute followed by the function
        if trimmed.starts_with("#[handler") {
            // Check if next non-empty, non-attribute line is our function
            for j in (i + 1)..lines.len() {
                let next = lines[j].trim();
                if next.is_empty() || next.starts_with("#[") {
                    continue;
                }
                if next.contains(&format!("fn {}", function_name))
                    || next.contains(&format!("pub fn {}", function_name))
                    || next.contains(&format!("pub async fn {}", function_name))
                    || next.contains(&format!("async fn {}", function_name))
                {
                    line_start = Some(i);
                    in_handler = true;
                    brace_count = 0;
                }
                break;
            }
        }

        if in_handler {
            brace_count += line.chars().filter(|&c| c == '{').count();
            brace_count = brace_count.saturating_sub(line.chars().filter(|&c| c == '}').count());

            if brace_count == 0 && line.contains('}') {
                let start = line_start.unwrap();
                let end = i + 1;

                let source_code = lines[start..end].join("\n");
                let relative_path = file_path
                    .strip_prefix(project_root)
                    .unwrap_or(file_path)
                    .to_string_lossy()
                    .to_string();

                return Ok(HandlerInfo {
                    handler: handler.to_string(),
                    path: route.path.clone(),
                    method: route.method.clone(),
                    file_path: relative_path,
                    source_code,
                    line_start: start + 1, // 1-indexed
                    line_end: end,
                });
            }
        }
    }

    Err(McpError::NotFound(format!(
        "Handler function '{}' not found in {}",
        function_name,
        file_path.display()
    )))
}
