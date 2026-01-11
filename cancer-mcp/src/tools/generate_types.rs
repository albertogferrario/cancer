//! Generate types tool - trigger TypeScript type generation

use crate::error::{McpError, Result};
use serde::Serialize;
use std::path::Path;
use std::process::Command;

#[derive(Debug, Serialize)]
pub struct GenerateTypesResult {
    pub success: bool,
    pub output_path: Option<String>,
    pub message: String,
    pub types_generated: Vec<String>,
}

pub fn execute(project_root: &Path, output_path: Option<&str>) -> Result<GenerateTypesResult> {
    let output = output_path.unwrap_or("resources/js/types/models.d.ts");
    let full_output_path = project_root.join(output);

    // Ensure output directory exists
    if let Some(parent) = full_output_path.parent() {
        std::fs::create_dir_all(parent).map_err(McpError::IoError)?;
    }

    // Try to run the cancer generate:types command
    let result = Command::new("cargo")
        .args(["run", "-p", "cancer-cli", "--", "generate:types"])
        .current_dir(project_root)
        .output();

    match result {
        Ok(output_result) => {
            if output_result.status.success() {
                // Parse generated types from output
                let stdout = String::from_utf8_lossy(&output_result.stdout);
                let types_generated = extract_generated_types(&stdout);

                Ok(GenerateTypesResult {
                    success: true,
                    output_path: Some(output.to_string()),
                    message: "TypeScript types generated successfully".to_string(),
                    types_generated,
                })
            } else {
                let stderr = String::from_utf8_lossy(&output_result.stderr);
                Ok(GenerateTypesResult {
                    success: false,
                    output_path: None,
                    message: format!("Failed to generate types: {}", stderr),
                    types_generated: vec![],
                })
            }
        }
        Err(e) => {
            // Fallback: scan models and report what would be generated
            let models = scan_models_for_types(project_root);
            Ok(GenerateTypesResult {
                success: false,
                output_path: Some(output.to_string()),
                message: format!(
                    "Could not run cancer CLI ({}). Found {} models that would be converted.",
                    e,
                    models.len()
                ),
                types_generated: models,
            })
        }
    }
}

fn extract_generated_types(output: &str) -> Vec<String> {
    let mut types = Vec::new();
    for line in output.lines() {
        if line.contains("Generated") || line.contains("interface") || line.contains("type ") {
            types.push(line.trim().to_string());
        }
    }
    types
}

fn scan_models_for_types(project_root: &Path) -> Vec<String> {
    let models_dir = project_root.join("src/models");
    let mut models = Vec::new();

    if models_dir.exists() {
        if let Ok(entries) = std::fs::read_dir(&models_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().map(|e| e == "rs").unwrap_or(false) {
                    if let Some(stem) = path.file_stem() {
                        let name = stem.to_string_lossy();
                        if name != "mod" {
                            // Convert snake_case to PascalCase
                            let pascal_name: String = name
                                .split('_')
                                .map(|s| {
                                    let mut c = s.chars();
                                    match c.next() {
                                        None => String::new(),
                                        Some(f) => {
                                            f.to_uppercase().collect::<String>() + c.as_str()
                                        }
                                    }
                                })
                                .collect();
                            models.push(pascal_name);
                        }
                    }
                }
            }
        }
    }

    models
}
