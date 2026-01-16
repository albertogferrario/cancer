//! Generate types tool - generate TypeScript types from InertiaProps structs
//!
//! This provides direct introspection and type generation without running the CLI.

use crate::error::{McpError, Result};
use crate::tools::list_props::{self, PropsStruct};
use serde::Serialize;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize)]
pub struct GenerateTypesResult {
    pub success: bool,
    pub output_path: Option<String>,
    pub message: String,
    /// Number of types generated
    pub types_generated: usize,
    /// List of generated interface names
    pub interfaces: Vec<String>,
    /// Preview of generated TypeScript (first 2000 chars)
    pub preview: Option<String>,
    /// Differences from existing file (if any)
    pub diff: Option<TypesDiff>,
}

#[derive(Debug, Serialize)]
pub struct TypesDiff {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub modified: Vec<String>,
}

pub fn execute(
    project_root: &Path,
    output_path: Option<&str>,
    dry_run: bool,
) -> Result<GenerateTypesResult> {
    let output = output_path.unwrap_or("frontend/src/types/inertia-props.ts");
    let full_output_path = project_root.join(output);

    // Get all InertiaProps structs
    let props_result = list_props::execute(project_root, None)?;

    if props_result.props.is_empty() {
        return Ok(GenerateTypesResult {
            success: true,
            output_path: Some(output.to_string()),
            message: "No InertiaProps structs found in the project".to_string(),
            types_generated: 0,
            interfaces: vec![],
            preview: None,
            diff: None,
        });
    }

    // Sort topologically (dependencies first)
    let sorted = topological_sort(&props_result.props);

    // Generate TypeScript
    let typescript = generate_typescript(&sorted);

    // Check for differences with existing file
    let diff = if full_output_path.exists() {
        let existing = fs::read_to_string(&full_output_path).ok();
        existing.and_then(|e| compute_diff(&e, &typescript, &sorted))
    } else {
        None
    };

    let interfaces: Vec<String> = sorted.iter().map(|p| p.name.clone()).collect();
    let types_generated = interfaces.len();

    // Preview (truncate if too long)
    let preview = if typescript.len() > 2000 {
        Some(format!(
            "{}...\n\n(truncated, {} total characters)",
            &typescript[..2000],
            typescript.len()
        ))
    } else {
        Some(typescript.clone())
    };

    if dry_run {
        return Ok(GenerateTypesResult {
            success: true,
            output_path: Some(output.to_string()),
            message: format!("Dry run: would generate {} interface(s)", types_generated),
            types_generated,
            interfaces,
            preview,
            diff,
        });
    }

    // Ensure output directory exists
    if let Some(parent) = full_output_path.parent() {
        fs::create_dir_all(parent).map_err(McpError::IoError)?;
    }

    // Write the file
    fs::write(&full_output_path, &typescript).map_err(McpError::IoError)?;

    Ok(GenerateTypesResult {
        success: true,
        output_path: Some(output.to_string()),
        message: format!("Generated {} interface(s) to {}", types_generated, output),
        types_generated,
        interfaces,
        preview,
        diff,
    })
}

fn topological_sort(props: &[PropsStruct]) -> Vec<&PropsStruct> {
    let names: HashSet<String> = props.iter().map(|p| p.name.clone()).collect();

    // Build dependency graph using owned strings
    let mut deps: HashMap<String, HashSet<String>> = HashMap::new();
    for p in props {
        let mut p_deps = HashSet::new();
        for field in &p.fields {
            // Check if this field's type is another InertiaProps struct
            let type_name = extract_base_type(&field.typescript_type);
            if names.contains(&type_name) {
                p_deps.insert(type_name);
            }
        }
        deps.insert(p.name.clone(), p_deps);
    }

    // Kahn's algorithm
    let mut in_degree: HashMap<String, usize> = names.iter().map(|n| (n.clone(), 0)).collect();
    for p_deps in deps.values() {
        for dep in p_deps {
            if let Some(count) = in_degree.get_mut(dep) {
                *count += 1;
            }
        }
    }

    let mut queue: Vec<String> = in_degree
        .iter()
        .filter(|(_, &count)| count == 0)
        .map(|(name, _)| name.clone())
        .collect();

    let mut result = Vec::new();
    let props_map: HashMap<_, _> = props.iter().map(|p| (p.name.clone(), p)).collect();

    while let Some(name) = queue.pop() {
        if let Some(p) = props_map.get(&name) {
            result.push(*p);
        }
        if let Some(p_deps) = deps.get(&name) {
            for dep in p_deps {
                if let Some(count) = in_degree.get_mut(dep) {
                    *count = count.saturating_sub(1);
                    if *count == 0 {
                        queue.push(dep.clone());
                    }
                }
            }
        }
    }

    result
}

fn extract_base_type(ts_type: &str) -> String {
    // Remove array suffix, null union, etc
    ts_type
        .replace("[]", "")
        .replace(" | null", "")
        .replace(" | undefined", "")
        .trim()
        .to_string()
}

fn generate_typescript(props: &[&PropsStruct]) -> String {
    let mut output = String::new();
    output.push_str("// This file is auto-generated by Ferro. Do not edit manually.\n");
    output.push_str("// Run `ferro generate-types` to regenerate.\n\n");

    for p in props {
        output.push_str(&p.typescript);
        output.push_str("\n\n");
    }

    output.trim_end().to_string()
}

fn compute_diff(existing: &str, _new: &str, props: &[&PropsStruct]) -> Option<TypesDiff> {
    // Extract existing interface names
    let existing_interfaces: HashSet<_> = regex::Regex::new(r"(?:export\s+)?interface\s+(\w+)")
        .ok()?
        .captures_iter(existing)
        .filter_map(|c| c.get(1).map(|m| m.as_str().to_string()))
        .collect();

    let new_interfaces: HashSet<_> = props.iter().map(|p| p.name.clone()).collect();

    let added: Vec<_> = new_interfaces
        .difference(&existing_interfaces)
        .cloned()
        .collect();

    let removed: Vec<_> = existing_interfaces
        .difference(&new_interfaces)
        .cloned()
        .collect();

    // Check for modifications (same name but different content)
    let mut modified = Vec::new();
    for p in props {
        if existing_interfaces.contains(&p.name) {
            // Simple check: see if the interface body changed
            let existing_pattern = regex::Regex::new(&format!(
                r"(?:export\s+)?interface\s+{}\s*\{{\s*([^}}]+)\}}",
                regex::escape(&p.name)
            ))
            .ok()?;

            if let Some(cap) = existing_pattern.captures(existing) {
                let existing_body = cap.get(1)?.as_str().trim();
                // Normalize for comparison
                let existing_normalized = normalize_interface_body(existing_body);
                let new_normalized = normalize_interface_body(&p.typescript);

                if existing_normalized != new_normalized {
                    modified.push(p.name.clone());
                }
            }
        }
    }

    if added.is_empty() && removed.is_empty() && modified.is_empty() {
        return None;
    }

    Some(TypesDiff {
        added,
        removed,
        modified,
    })
}

fn normalize_interface_body(body: &str) -> String {
    body.lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}
