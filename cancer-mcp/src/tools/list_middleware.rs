//! List middleware tool - scan for registered middleware

use crate::error::Result;
use crate::introspection::middleware;
use serde::Serialize;
use std::path::Path;

#[derive(Debug, Serialize)]
pub struct MiddlewareInfo {
    pub middleware: Vec<MiddlewareItem>,
}

#[derive(Debug, Serialize)]
pub struct MiddlewareItem {
    pub name: String,
    pub path: String,
    pub global: bool,
}

pub fn execute(project_root: &Path) -> Result<MiddlewareInfo> {
    let middleware_items = middleware::scan_middleware(project_root);
    Ok(MiddlewareInfo {
        middleware: middleware_items,
    })
}
