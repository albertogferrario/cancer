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

    // Generate TypeScript with imports from shared.ts
    let typescript = generate_typescript_with_imports(&sorted, Some(project_root));

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

/// Parse shared.ts to find exported type names
fn parse_shared_types(project_root: &Path) -> HashSet<String> {
    let shared_path = project_root.join("frontend/src/types/shared.ts");

    if !shared_path.exists() {
        return HashSet::new();
    }

    let content = match fs::read_to_string(&shared_path) {
        Ok(c) => c,
        Err(_) => return HashSet::new(),
    };

    let mut types = HashSet::new();

    // Match: export interface Name, export type Name, export enum Name
    let patterns = [
        r"export\s+interface\s+(\w+)",
        r"export\s+type\s+(\w+)",
        r"export\s+enum\s+(\w+)",
    ];

    for pattern in patterns {
        if let Ok(re) = regex::Regex::new(pattern) {
            for cap in re.captures_iter(&content) {
                if let Some(name) = cap.get(1) {
                    types.insert(name.as_str().to_string());
                }
            }
        }
    }

    types
}

/// Collect all custom types referenced in props fields
fn collect_referenced_types(props: &[&PropsStruct]) -> HashSet<String> {
    let mut types = HashSet::new();

    // Pattern to find type references (capitalized words that aren't keywords)
    let type_pattern = regex::Regex::new(r"\b([A-Z][a-zA-Z0-9]+)\b").unwrap();

    for p in props {
        for field in &p.fields {
            // Check the typescript_type for custom type references
            for cap in type_pattern.captures_iter(&field.typescript_type) {
                if let Some(name) = cap.get(1) {
                    let type_name = name.as_str();
                    // Filter out TypeScript built-in types
                    if !matches!(
                        type_name,
                        "Record" | "Array" | "Partial" | "Required" | "Readonly" | "Map" | "Set"
                    ) {
                        types.insert(type_name.to_string());
                    }
                }
            }
        }
    }

    types
}

fn generate_typescript_with_imports(props: &[&PropsStruct], project_root: Option<&Path>) -> String {
    // Collect struct names (types defined in this file)
    let defined_types: HashSet<String> = props.iter().map(|p| p.name.clone()).collect();

    // Parse shared.ts types
    let shared_types = project_root.map(parse_shared_types).unwrap_or_default();

    // Find types to import from shared.ts (only referenced ones)
    let mut imports_needed = Vec::new();
    if project_root.is_some() && !shared_types.is_empty() {
        let referenced_types = collect_referenced_types(props);

        // Types that are referenced but not defined in this file
        let mut to_import: Vec<_> = referenced_types
            .iter()
            .filter(|t| shared_types.contains(*t) && !defined_types.contains(*t))
            .cloned()
            .collect();
        to_import.sort();
        imports_needed = to_import;
    }

    // Collect all shared types for re-export (sorted for consistency)
    let mut reexport_types: Vec<_> = shared_types.iter().cloned().collect();
    reexport_types.sort();

    let mut output = String::new();
    output.push_str("// This file is auto-generated by Ferro. Do not edit manually.\n");
    output.push_str("// Run `ferro generate-types` to regenerate.\n\n");

    // Add import statement if we need types from shared.ts
    if !imports_needed.is_empty() {
        output.push_str(&format!(
            "import type {{ {} }} from './shared';\n\n",
            imports_needed.join(", ")
        ));
    }

    // Add re-export statement for all shared.ts types
    if !reexport_types.is_empty() {
        output.push_str("// Re-exports from shared.ts for convenience\n");
        output.push_str(&format!(
            "export type {{ {} }} from './shared';\n\n",
            reexport_types.join(", ")
        ));
    }

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
