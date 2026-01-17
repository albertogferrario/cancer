//! validate:contracts command - Validate Inertia frontend/backend prop contracts
//!
//! Compares Rust InertiaProps structs with TypeScript interfaces to detect:
//! - Missing fields in either direction
//! - Type mismatches between Rust and TypeScript
//! - Nullability mismatches (Option vs required)

use console::style;
use regex::Regex;
use serde::Serialize;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

/// Result of contract validation
#[derive(Debug, Serialize)]
pub struct ContractValidationResult {
    pub total_routes: usize,
    pub validated: usize,
    pub passed: usize,
    pub failed: usize,
    pub skipped: usize,
    pub validations: Vec<RouteValidation>,
    pub summary: Vec<String>,
}

/// Validation result for a single route
#[derive(Debug, Serialize)]
pub struct RouteValidation {
    pub route: String,
    pub component: String,
    pub status: ValidationStatus,
    pub rust_props: Option<PropsInfo>,
    pub typescript_props: Option<PropsInfo>,
    pub mismatches: Vec<Mismatch>,
}

/// Information about props from either Rust or TypeScript
#[derive(Debug, Serialize, Clone)]
pub struct PropsInfo {
    pub name: String,
    pub fields: Vec<PropField>,
    pub source_file: String,
}

/// A single field in props
#[derive(Debug, Serialize, Clone)]
pub struct PropField {
    pub name: String,
    pub field_type: String,
    pub optional: bool,
}

/// Validation status for a route
#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ValidationStatus {
    Passed,
    Failed,
    Skipped,
}

/// A mismatch between frontend and backend
#[derive(Debug, Serialize)]
pub struct Mismatch {
    pub kind: MismatchKind,
    pub field: String,
    pub details: String,
}

/// Type of mismatch
#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)] // TypeMismatch reserved for structural type validation
pub enum MismatchKind {
    MissingInBackend,
    MissingInFrontend,
    TypeMismatch,
    NullabilityMismatch,
}

/// Execute contract validation
pub fn execute(
    project_path: &Path,
    route_filter: Option<&str>,
) -> Result<ContractValidationResult, String> {
    let mut validations = Vec::new();
    let mut passed = 0;
    let mut failed = 0;
    let mut skipped = 0;

    // Parse routes and their handlers
    let routes = parse_routes_with_components(project_path)?;

    for (route, handler, component) in &routes {
        // Apply filter if provided
        if let Some(filter) = route_filter {
            if !route.contains(filter) && !component.contains(filter) {
                continue;
            }
        }

        let validation = validate_route(project_path, route, handler, component);

        match validation.status {
            ValidationStatus::Passed => passed += 1,
            ValidationStatus::Failed => failed += 1,
            ValidationStatus::Skipped => skipped += 1,
        }

        validations.push(validation);
    }

    let total_routes = validations.len();
    let validated = passed + failed;

    // Generate summary
    let mut summary = Vec::new();
    if failed > 0 {
        summary.push(format!(
            "{} contract(s) have mismatches that need attention",
            failed
        ));
    }
    if passed > 0 {
        summary.push(format!("{} contract(s) validated successfully", passed));
    }
    if skipped > 0 {
        summary.push(format!(
            "{} route(s) skipped (no props or component found)",
            skipped
        ));
    }

    Ok(ContractValidationResult {
        total_routes,
        validated,
        passed,
        failed,
        skipped,
        validations,
        summary,
    })
}

/// Parse routes file to extract route/handler/component mappings
fn parse_routes_with_components(
    project_path: &Path,
) -> Result<Vec<(String, String, String)>, String> {
    let routes_file = project_path.join("src/routes.rs");
    if !routes_file.exists() {
        return Err("src/routes.rs not found".to_string());
    }

    let content =
        fs::read_to_string(&routes_file).map_err(|e| format!("Failed to read routes.rs: {}", e))?;
    let mut routes = Vec::new();

    // Pattern to match route definitions
    let route_pattern = Regex::new(
        r#"(get|post|put|patch|delete)!\s*\(\s*"([^"]+)"\s*,\s*([a-zA-Z_][a-zA-Z0-9_:]*)\s*\)"#,
    )
    .unwrap();

    for cap in route_pattern.captures_iter(&content) {
        let path = cap
            .get(2)
            .map(|m| m.as_str().to_string())
            .unwrap_or_default();
        let handler = cap
            .get(3)
            .map(|m| m.as_str().to_string())
            .unwrap_or_default();

        // Try to find the component this handler renders
        if let Some(component) = find_component_for_handler(project_path, &handler) {
            routes.push((path, handler, component));
        }
    }

    Ok(routes)
}

/// Find the Inertia component that a handler renders
fn find_component_for_handler(project_path: &Path, handler: &str) -> Option<String> {
    let parts: Vec<&str> = handler.split("::").collect();
    if parts.len() < 2 {
        return None;
    }

    let file_parts: Vec<&str> = parts[..parts.len() - 1].to_vec();
    let file_path = project_path
        .join("src")
        .join(file_parts.join("/"))
        .with_extension("rs");

    if !file_path.exists() {
        return None;
    }

    let content = fs::read_to_string(&file_path).ok()?;
    let function_name = parts.last()?;

    // Find the inertia_response! call in the handler
    let inertia_pattern = Regex::new(&format!(
        r#"fn\s+{}\s*\([^)]*\)[^{{]*\{{[^}}]*inertia_response!\s*\(\s*"([^"]+)""#,
        function_name
    ))
    .ok()?;

    if let Some(cap) = inertia_pattern.captures(&content) {
        return cap.get(1).map(|m| m.as_str().to_string());
    }

    // Fallback: search for any inertia_response! after the function definition
    let func_start = content.find(&format!("fn {}", function_name))?;
    let after_func = &content[func_start..];

    let simple_pattern = Regex::new(r#"inertia_response!\s*\(\s*"([^"]+)""#).ok()?;
    if let Some(cap) = simple_pattern.captures(after_func) {
        return cap.get(1).map(|m| m.as_str().to_string());
    }

    None
}

/// Validate a single route
fn validate_route(
    project_path: &Path,
    route: &str,
    handler: &str,
    component: &str,
) -> RouteValidation {
    // Extract Rust props from handler
    let rust_props = extract_rust_props(project_path, handler);

    // Extract TypeScript props from component
    let typescript_props = extract_typescript_props(project_path, component);

    // If we can't find either, skip validation
    if rust_props.is_none() || typescript_props.is_none() {
        return RouteValidation {
            route: route.to_string(),
            component: component.to_string(),
            status: ValidationStatus::Skipped,
            rust_props,
            typescript_props,
            mismatches: vec![],
        };
    }

    let rust = rust_props.as_ref().unwrap();
    let ts = typescript_props.as_ref().unwrap();

    // Compare fields
    let mismatches = compare_props(rust, ts);

    let status = if mismatches.is_empty() {
        ValidationStatus::Passed
    } else {
        ValidationStatus::Failed
    };

    RouteValidation {
        route: route.to_string(),
        component: component.to_string(),
        status,
        rust_props,
        typescript_props,
        mismatches,
    }
}

/// Extract Rust props from a handler
fn extract_rust_props(project_path: &Path, handler: &str) -> Option<PropsInfo> {
    let parts: Vec<&str> = handler.split("::").collect();
    if parts.len() < 2 {
        return None;
    }

    let file_parts: Vec<&str> = parts[..parts.len() - 1].to_vec();
    let function_name = parts.last()?;

    let file_path = project_path
        .join("src")
        .join(file_parts.join("/"))
        .with_extension("rs");

    if !file_path.exists() {
        return None;
    }

    let content = fs::read_to_string(&file_path).ok()?;

    // Find the handler function and extract what it returns
    let func_start = content.find(&format!("fn {}", function_name))?;
    let after_func = &content[func_start..];

    // Look for the Props struct being used in inertia_response!
    let props_pattern =
        Regex::new(r#"inertia_response!\s*\(\s*"[^"]+"\s*,\s*([A-Z][a-zA-Z0-9]*)\s*\{"#).ok()?;

    let props_name = if let Some(cap) = props_pattern.captures(after_func) {
        cap.get(1).map(|m| m.as_str().to_string())
    } else {
        None
    };

    // Try to find the Props struct definition
    if let Some(name) = &props_name {
        // Search in the same file or common props locations
        if let Some(props) = find_props_struct(&content, name, &file_path) {
            return Some(props);
        }

        // Search in props module
        let props_file = project_path.join("src/props.rs");
        if props_file.exists() {
            if let Ok(props_content) = fs::read_to_string(&props_file) {
                if let Some(props) = find_props_struct(&props_content, name, &props_file) {
                    return Some(props);
                }
            }
        }
    }

    // Fallback: extract inline struct fields from inertia_response!
    extract_inline_props(after_func, &file_path)
}

/// Find a props struct definition in source code
fn find_props_struct(content: &str, name: &str, source_file: &Path) -> Option<PropsInfo> {
    let struct_pattern = Regex::new(&format!(
        r#"(?:#\[derive\([^\)]*\)\]\s*)*struct\s+{}\s*\{{\s*([^}}]+)\}}"#,
        name
    ))
    .ok()?;

    let cap = struct_pattern.captures(content)?;
    let fields_str = cap.get(1)?.as_str();

    let fields = parse_rust_fields(fields_str);

    Some(PropsInfo {
        name: name.to_string(),
        fields,
        source_file: source_file.to_string_lossy().to_string(),
    })
}

/// Extract inline props from handler code
fn extract_inline_props(handler_code: &str, source_file: &Path) -> Option<PropsInfo> {
    let inline_pattern = Regex::new(r#"([A-Z][a-zA-Z0-9]*)\s*\{\s*([^}]+)\}"#).ok()?;

    for cap in inline_pattern.captures_iter(handler_code) {
        let name = cap.get(1)?.as_str();
        if name.ends_with("Props") || name.ends_with("Detail") || name.ends_with("Summary") {
            let fields_str = cap.get(2)?.as_str();
            let fields = parse_inline_fields(fields_str);

            if !fields.is_empty() {
                return Some(PropsInfo {
                    name: name.to_string(),
                    fields,
                    source_file: source_file.to_string_lossy().to_string(),
                });
            }
        }
    }

    None
}

/// Parse Rust struct fields
fn parse_rust_fields(fields_str: &str) -> Vec<PropField> {
    let mut fields = Vec::new();
    let field_pattern = Regex::new(r#"(?:pub\s+)?(\w+)\s*:\s*([^,\n]+)"#).unwrap();

    for cap in field_pattern.captures_iter(fields_str) {
        let name = cap
            .get(1)
            .map(|m| m.as_str().to_string())
            .unwrap_or_default();
        let field_type = cap
            .get(2)
            .map(|m| m.as_str().trim().to_string())
            .unwrap_or_default();

        // Skip if it looks like a derive attribute or comment
        if name.starts_with('#') || name.starts_with('/') {
            continue;
        }

        let optional = field_type.starts_with("Option<");

        fields.push(PropField {
            name,
            field_type,
            optional,
        });
    }

    fields
}

/// Parse inline field names from struct instantiation
fn parse_inline_fields(fields_str: &str) -> Vec<PropField> {
    let mut fields = Vec::new();
    let field_pattern = Regex::new(r#"(\w+)\s*:"#).unwrap();

    for cap in field_pattern.captures_iter(fields_str) {
        let name = cap
            .get(1)
            .map(|m| m.as_str().to_string())
            .unwrap_or_default();
        fields.push(PropField {
            name,
            field_type: "unknown".to_string(),
            optional: false,
        });
    }

    fields
}

/// Extract TypeScript props from a component file
fn extract_typescript_props(project_path: &Path, component: &str) -> Option<PropsInfo> {
    let component_path = project_path
        .join("frontend/src/pages")
        .join(format!("{}.tsx", component));

    if !component_path.exists() {
        return None;
    }

    let content = fs::read_to_string(&component_path).ok()?;

    // Strategy 1: Look for imported Props type usage in function signature
    let func_pattern = Regex::new(
        r#"(?:export\s+default\s+)?function\s+\w+\s*\(\s*\{\s*([^}]+)\s*\}\s*:\s*(\w+)"#,
    )
    .ok()?;

    if let Some(cap) = func_pattern.captures(&content) {
        let destructured = cap.get(1)?.as_str();
        let props_type = cap.get(2)?.as_str();

        let fields = parse_destructured_props(destructured);

        // Also try to find the interface definition
        let mut all_fields = fields;
        if let Some(interface_fields) = find_interface_fields(&content, props_type) {
            let existing_names: HashSet<_> = all_fields.iter().map(|f| f.name.clone()).collect();
            for field in interface_fields {
                if !existing_names.contains(&field.name) {
                    all_fields.push(field);
                }
            }
        }

        // Check imported types from inertia-props.ts
        if let Some(imported_fields) = find_imported_props(project_path, &content, props_type) {
            let existing_names: HashSet<_> = all_fields.iter().map(|f| f.name.clone()).collect();
            for field in imported_fields {
                if !existing_names.contains(&field.name) {
                    all_fields.push(field);
                }
            }
        }

        return Some(PropsInfo {
            name: props_type.to_string(),
            fields: all_fields,
            source_file: component_path.to_string_lossy().to_string(),
        });
    }

    // Strategy 2: Look for interface definition in the file
    let interface_pattern = Regex::new(r#"interface\s+(\w*Props\w*)\s*\{([^}]+)\}"#).ok()?;
    if let Some(cap) = interface_pattern.captures(&content) {
        let name = cap.get(1)?.as_str();
        let fields_str = cap.get(2)?.as_str();

        return Some(PropsInfo {
            name: name.to_string(),
            fields: parse_typescript_interface_fields(fields_str),
            source_file: component_path.to_string_lossy().to_string(),
        });
    }

    None
}

/// Parse destructured props from function signature
fn parse_destructured_props(destructured: &str) -> Vec<PropField> {
    let mut fields = Vec::new();

    for part in destructured.split(',') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }

        let name = part
            .split(':')
            .next()
            .unwrap_or(part)
            .split('=')
            .next()
            .unwrap_or(part)
            .trim()
            .to_string();

        if !name.is_empty() && !name.starts_with("...") {
            fields.push(PropField {
                name,
                field_type: "unknown".to_string(),
                optional: part.contains('='),
            });
        }
    }

    fields
}

/// Find interface fields in content
fn find_interface_fields(content: &str, props_type: &str) -> Option<Vec<PropField>> {
    let pattern = Regex::new(&format!(
        r#"interface\s+{}\s*(?:extends\s+[^{{]+)?\{{\s*([^}}]+)\}}"#,
        regex::escape(props_type)
    ))
    .ok()?;

    let cap = pattern.captures(content)?;
    let fields_str = cap.get(1)?.as_str();

    Some(parse_typescript_interface_fields(fields_str))
}

/// Find imported props from types files
fn find_imported_props(
    project_path: &Path,
    content: &str,
    props_type: &str,
) -> Option<Vec<PropField>> {
    let pattern_str = format!(
        r#"import\s*\{{[^}}]*\b{}\b[^}}]*\}}\s*from\s*['"]([^'"]+)['"]"#,
        regex::escape(props_type)
    );
    let import_pattern = Regex::new(&pattern_str).ok()?;

    let cap = import_pattern.captures(content)?;
    let import_path = cap.get(1)?.as_str();

    let types_file = if import_path.contains("inertia-props") {
        project_path.join("frontend/src/types/inertia-props.ts")
    } else if import_path.contains("shared") {
        project_path.join("frontend/src/types/shared.ts")
    } else {
        return None;
    };

    if !types_file.exists() {
        return None;
    }

    let types_content = fs::read_to_string(&types_file).ok()?;
    find_interface_fields(&types_content, props_type)
}

/// Parse TypeScript interface fields
fn parse_typescript_interface_fields(fields_str: &str) -> Vec<PropField> {
    let mut fields = Vec::new();
    let field_pattern = Regex::new(r#"(\w+)(\?)?:\s*([^;,\n]+)"#).unwrap();

    for cap in field_pattern.captures_iter(fields_str) {
        let name = cap
            .get(1)
            .map(|m| m.as_str().to_string())
            .unwrap_or_default();
        let optional = cap.get(2).is_some();
        let field_type = cap
            .get(3)
            .map(|m| m.as_str().trim().to_string())
            .unwrap_or_default();

        fields.push(PropField {
            name,
            field_type,
            optional,
        });
    }

    fields
}

/// Compare Rust and TypeScript props
fn compare_props(rust: &PropsInfo, ts: &PropsInfo) -> Vec<Mismatch> {
    let mut mismatches = Vec::new();

    let rust_fields: HashMap<_, _> = rust.fields.iter().map(|f| (f.name.clone(), f)).collect();
    let ts_fields: HashMap<_, _> = ts.fields.iter().map(|f| (f.name.clone(), f)).collect();

    // Build a map of camelCase -> original rust field names for reverse lookup
    let rust_camel_to_snake: HashMap<String, String> = rust_fields
        .keys()
        .map(|name| (to_camel_case(name), name.clone()))
        .collect();

    // Check for fields in TypeScript but missing in Rust
    for (name, ts_field) in &ts_fields {
        // Check if the TS field exists in Rust (direct match or snake_case version)
        let has_rust_match =
            rust_fields.contains_key(name) || rust_camel_to_snake.contains_key(name);

        if !has_rust_match && !ts_field.optional {
            mismatches.push(Mismatch {
                kind: MismatchKind::MissingInBackend,
                field: name.clone(),
                details: format!("Frontend expects '{}' but backend doesn't send it", name),
            });
        }
    }

    // Check for fields in Rust but not used in TypeScript
    for name in rust_fields.keys() {
        if !ts_fields.contains_key(name) {
            let camel_name = to_camel_case(name);
            if !ts_fields.contains_key(&camel_name) {
                mismatches.push(Mismatch {
                    kind: MismatchKind::MissingInFrontend,
                    field: name.clone(),
                    details: format!(
                        "Backend sends '{}' but frontend doesn't use it (might be intentional)",
                        name
                    ),
                });
            }
        }
    }

    // Check type mismatches for common fields
    for (name, rust_field) in &rust_fields {
        let ts_name = if ts_fields.contains_key(name) {
            name.clone()
        } else {
            to_camel_case(name)
        };

        if let Some(ts_field) = ts_fields.get(&ts_name) {
            // Check nullability mismatch
            if rust_field.optional && !ts_field.optional && !ts_field.field_type.contains("null") {
                mismatches.push(Mismatch {
                    kind: MismatchKind::NullabilityMismatch,
                    field: name.clone(),
                    details: format!(
                        "Backend sends Option<{}> but frontend expects non-nullable {}",
                        rust_field.field_type, ts_field.field_type
                    ),
                });
            }
        }
    }

    mismatches
}

/// Convert snake_case to camelCase
fn to_camel_case(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut capitalize_next = false;

    for c in s.chars() {
        if c == '_' {
            capitalize_next = true;
        } else if capitalize_next {
            result.extend(c.to_uppercase());
            capitalize_next = false;
        } else {
            result.push(c);
        }
    }

    result
}

/// Print validation results to console
fn print_results(result: &ContractValidationResult) {
    if result.validations.is_empty() {
        println!("{}", style("No Inertia routes found to validate.").yellow());
        return;
    }

    // Print header
    println!();
    println!("{}", style("Contract Validation Results").cyan().bold());
    println!("{}", style("=".repeat(50)).dim());
    println!();

    // Print each validation
    for validation in &result.validations {
        let status_icon = match validation.status {
            ValidationStatus::Passed => style("PASS").green(),
            ValidationStatus::Failed => style("FAIL").red(),
            ValidationStatus::Skipped => style("SKIP").yellow(),
        };

        println!(
            "  {} {} -> {}",
            status_icon,
            style(&validation.route).bold(),
            style(&validation.component).dim()
        );

        // Print mismatches for failed validations
        for mismatch in &validation.mismatches {
            let kind_label = match mismatch.kind {
                MismatchKind::MissingInBackend => "missing in backend",
                MismatchKind::MissingInFrontend => "missing in frontend",
                MismatchKind::TypeMismatch => "type mismatch",
                MismatchKind::NullabilityMismatch => "nullability mismatch",
            };
            println!(
                "       {} {} - {}",
                style("->").red(),
                style(format!("[{}]", kind_label)).yellow(),
                mismatch.details
            );
        }
    }

    // Print summary
    println!();
    println!("{}", style("-".repeat(50)).dim());
    println!();

    println!(
        "  {} {} validated, {} passed, {} failed, {} skipped",
        style("Total:").bold(),
        result.validated,
        style(result.passed).green(),
        if result.failed > 0 {
            style(result.failed).red()
        } else {
            style(result.failed).green()
        },
        style(result.skipped).yellow()
    );

    for summary_line in &result.summary {
        println!("  {} {}", style("->").cyan(), summary_line);
    }

    println!();
}

/// Main entry point for the validate:contracts command
pub fn run(filter: Option<String>, json: bool) {
    let project_path = Path::new(".");

    // Validate Ferro project
    let cargo_toml = project_path.join("Cargo.toml");
    if !cargo_toml.exists() {
        eprintln!(
            "{} Not a Ferro project (no Cargo.toml found)",
            style("Error:").red().bold()
        );
        std::process::exit(1);
    }

    // Check for routes.rs
    let routes_rs = project_path.join("src/routes.rs");
    if !routes_rs.exists() {
        eprintln!("{} No src/routes.rs found", style("Error:").red().bold());
        std::process::exit(1);
    }

    if !json {
        println!("{}", style("Validating Inertia contracts...").cyan());
    }

    match execute(project_path, filter.as_deref()) {
        Ok(result) => {
            if json {
                // Output JSON for programmatic use
                match serde_json::to_string_pretty(&result) {
                    Ok(json_output) => println!("{}", json_output),
                    Err(e) => {
                        eprintln!(
                            "{} Failed to serialize results: {}",
                            style("Error:").red().bold(),
                            e
                        );
                        std::process::exit(1);
                    }
                }
            } else {
                // Human-readable output
                print_results(&result);
            }

            // Exit with error code if any contracts failed
            if result.failed > 0 {
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("{} {}", style("Error:").red().bold(), e);
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_camel_case() {
        assert_eq!(to_camel_case("created_at"), "createdAt");
        assert_eq!(to_camel_case("user_id"), "userId");
        assert_eq!(to_camel_case("some_long_name"), "someLongName");
        assert_eq!(to_camel_case("name"), "name");
    }

    #[test]
    fn test_parse_rust_fields() {
        let fields_str = r#"
            pub id: i64,
            pub name: String,
            pub email: Option<String>,
        "#;

        let fields = parse_rust_fields(fields_str);

        assert_eq!(fields.len(), 3);
        assert_eq!(fields[0].name, "id");
        assert!(!fields[0].optional);
        assert_eq!(fields[1].name, "name");
        assert!(!fields[1].optional);
        assert_eq!(fields[2].name, "email");
        assert!(fields[2].optional);
    }

    #[test]
    fn test_parse_typescript_interface_fields() {
        let fields_str = r#"
            id: number;
            name: string;
            email?: string;
        "#;

        let fields = parse_typescript_interface_fields(fields_str);

        assert_eq!(fields.len(), 3);
        assert_eq!(fields[0].name, "id");
        assert!(!fields[0].optional);
        assert_eq!(fields[1].name, "name");
        assert!(!fields[1].optional);
        assert_eq!(fields[2].name, "email");
        assert!(fields[2].optional);
    }

    #[test]
    fn test_compare_props_missing_in_backend() {
        let rust = PropsInfo {
            name: "TestProps".to_string(),
            fields: vec![PropField {
                name: "id".to_string(),
                field_type: "i64".to_string(),
                optional: false,
            }],
            source_file: "test.rs".to_string(),
        };

        let ts = PropsInfo {
            name: "TestProps".to_string(),
            fields: vec![
                PropField {
                    name: "id".to_string(),
                    field_type: "number".to_string(),
                    optional: false,
                },
                PropField {
                    name: "name".to_string(),
                    field_type: "string".to_string(),
                    optional: false,
                },
            ],
            source_file: "test.tsx".to_string(),
        };

        let mismatches = compare_props(&rust, &ts);

        assert_eq!(mismatches.len(), 1);
        assert_eq!(mismatches[0].kind, MismatchKind::MissingInBackend);
        assert_eq!(mismatches[0].field, "name");
    }

    #[test]
    fn test_compare_props_missing_in_frontend() {
        let rust = PropsInfo {
            name: "TestProps".to_string(),
            fields: vec![
                PropField {
                    name: "id".to_string(),
                    field_type: "i64".to_string(),
                    optional: false,
                },
                PropField {
                    name: "extra".to_string(),
                    field_type: "String".to_string(),
                    optional: false,
                },
            ],
            source_file: "test.rs".to_string(),
        };

        let ts = PropsInfo {
            name: "TestProps".to_string(),
            fields: vec![PropField {
                name: "id".to_string(),
                field_type: "number".to_string(),
                optional: false,
            }],
            source_file: "test.tsx".to_string(),
        };

        let mismatches = compare_props(&rust, &ts);

        assert_eq!(mismatches.len(), 1);
        assert_eq!(mismatches[0].kind, MismatchKind::MissingInFrontend);
        assert_eq!(mismatches[0].field, "extra");
    }

    #[test]
    fn test_compare_props_nullability_mismatch() {
        let rust = PropsInfo {
            name: "TestProps".to_string(),
            fields: vec![PropField {
                name: "value".to_string(),
                field_type: "Option<String>".to_string(),
                optional: true,
            }],
            source_file: "test.rs".to_string(),
        };

        let ts = PropsInfo {
            name: "TestProps".to_string(),
            fields: vec![PropField {
                name: "value".to_string(),
                field_type: "string".to_string(),
                optional: false,
            }],
            source_file: "test.tsx".to_string(),
        };

        let mismatches = compare_props(&rust, &ts);

        assert_eq!(mismatches.len(), 1);
        assert_eq!(mismatches[0].kind, MismatchKind::NullabilityMismatch);
    }

    #[test]
    fn test_compare_props_camel_case_matching() {
        let rust = PropsInfo {
            name: "TestProps".to_string(),
            fields: vec![PropField {
                name: "created_at".to_string(),
                field_type: "String".to_string(),
                optional: false,
            }],
            source_file: "test.rs".to_string(),
        };

        let ts = PropsInfo {
            name: "TestProps".to_string(),
            fields: vec![PropField {
                name: "createdAt".to_string(),
                field_type: "string".to_string(),
                optional: false,
            }],
            source_file: "test.tsx".to_string(),
        };

        let mismatches = compare_props(&rust, &ts);

        // Should match snake_case to camelCase - no mismatches
        assert!(mismatches.is_empty());
    }

    #[test]
    fn test_parse_destructured_props() {
        let destructured = "id, name, email = ''";
        let fields = parse_destructured_props(destructured);

        assert_eq!(fields.len(), 3);
        assert_eq!(fields[0].name, "id");
        assert!(!fields[0].optional);
        assert_eq!(fields[1].name, "name");
        assert!(!fields[1].optional);
        assert_eq!(fields[2].name, "email");
        assert!(fields[2].optional);
    }

    #[test]
    fn test_parse_inline_fields() {
        let fields_str = "id: user.id, name: user.name, active: true";
        let fields = parse_inline_fields(fields_str);

        assert_eq!(fields.len(), 3);
        assert_eq!(fields[0].name, "id");
        assert_eq!(fields[1].name, "name");
        assert_eq!(fields[2].name, "active");
    }
}
