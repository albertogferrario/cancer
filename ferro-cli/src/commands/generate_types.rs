use console::style;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use syn::visit::Visit;
use syn::{Attribute, Fields, GenericArgument, ItemStruct, Meta, PathArguments, Type};
use walkdir::WalkDir;

/// Serde rename_all case transformation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SerdeCase {
    #[default]
    None,
    CamelCase,
    SnakeCase,
    PascalCase,
    ScreamingSnakeCase,
    KebabCase,
}

impl SerdeCase {
    /// Parse from serde attribute value
    fn from_str(s: &str) -> Self {
        match s {
            "camelCase" => Self::CamelCase,
            "snake_case" => Self::SnakeCase,
            "PascalCase" => Self::PascalCase,
            "SCREAMING_SNAKE_CASE" => Self::ScreamingSnakeCase,
            "kebab-case" => Self::KebabCase,
            _ => Self::None,
        }
    }

    /// Apply case transformation to a field name
    fn apply(&self, name: &str) -> String {
        match self {
            Self::None | Self::SnakeCase => name.to_string(),
            Self::CamelCase => snake_to_camel(name),
            Self::PascalCase => snake_to_pascal(name),
            Self::ScreamingSnakeCase => name.to_uppercase(),
            Self::KebabCase => name.replace('_', "-"),
        }
    }
}

/// Convert snake_case to camelCase
fn snake_to_camel(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = false;

    for c in s.chars() {
        if c == '_' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(c.to_ascii_uppercase());
            capitalize_next = false;
        } else {
            result.push(c);
        }
    }

    result
}

/// Convert snake_case to PascalCase
fn snake_to_pascal(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = true;

    for c in s.chars() {
        if c == '_' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(c.to_ascii_uppercase());
            capitalize_next = false;
        } else {
            result.push(c);
        }
    }

    result
}

/// Represents a parsed InertiaProps struct
#[derive(Debug, Clone)]
pub struct InertiaPropsStruct {
    pub name: String,
    pub fields: Vec<StructField>,
    /// Serde rename_all attribute on the struct
    pub rename_all: SerdeCase,
}

#[derive(Debug, Clone)]
pub struct StructField {
    pub name: String,
    pub ty: RustType,
    /// Per-field serde rename override
    pub serde_rename: Option<String>,
}

#[derive(Debug, Clone)]
pub enum RustType {
    String,
    Number,
    Bool,
    Option(Box<RustType>),
    Vec(Box<RustType>),
    HashMap(Box<RustType>, Box<RustType>),
    Custom(String),
}

/// Visitor that collects structs with #[derive(InertiaProps)]
struct InertiaPropsVisitor {
    structs: Vec<InertiaPropsStruct>,
}

impl InertiaPropsVisitor {
    fn new() -> Self {
        Self {
            structs: Vec::new(),
        }
    }

    fn has_inertia_props_derive(&self, attrs: &[Attribute]) -> bool {
        for attr in attrs {
            if attr.path().is_ident("derive") {
                if let Ok(nested) = attr.parse_args_with(
                    syn::punctuated::Punctuated::<syn::Path, syn::Token![,]>::parse_terminated,
                ) {
                    for path in nested {
                        if path.is_ident("InertiaProps") {
                            return true;
                        }
                        // Also check for ferro::InertiaProps
                        if path.segments.len() == 2 {
                            let first = &path.segments[0].ident;
                            let second = &path.segments[1].ident;
                            if first == "ferro" && second == "InertiaProps" {
                                return true;
                            }
                        }
                    }
                }
            }
        }
        false
    }

    /// Parse #[serde(rename_all = "...")] from struct attributes
    fn parse_serde_rename_all(attrs: &[Attribute]) -> SerdeCase {
        for attr in attrs {
            if attr.path().is_ident("serde") {
                if let Meta::List(meta_list) = &attr.meta {
                    // Parse the token stream to find rename_all = "value"
                    let tokens_str = meta_list.tokens.to_string();
                    if let Some(rename_all) = parse_serde_rename_all_value(&tokens_str) {
                        return SerdeCase::from_str(&rename_all);
                    }
                }
            }
        }
        SerdeCase::None
    }

    /// Parse #[serde(rename = "...")] from field attributes
    fn parse_serde_field_rename(attrs: &[Attribute]) -> Option<String> {
        for attr in attrs {
            if attr.path().is_ident("serde") {
                if let Meta::List(meta_list) = &attr.meta {
                    let tokens_str = meta_list.tokens.to_string();
                    if let Some(rename) = parse_serde_rename_value(&tokens_str) {
                        return Some(rename);
                    }
                }
            }
        }
        None
    }
}

/// Parse rename_all = "value" from serde attribute tokens
fn parse_serde_rename_all_value(tokens: &str) -> Option<String> {
    // Look for rename_all = "..."
    if let Some(start) = tokens.find("rename_all") {
        let rest = &tokens[start..];
        // Find the value between quotes
        if let Some(quote_start) = rest.find('"') {
            let after_quote = &rest[quote_start + 1..];
            if let Some(quote_end) = after_quote.find('"') {
                return Some(after_quote[..quote_end].to_string());
            }
        }
    }
    None
}

/// Parse rename = "value" from serde attribute tokens (but not rename_all)
fn parse_serde_rename_value(tokens: &str) -> Option<String> {
    // Look for "rename" followed by "=" but not "rename_all"
    let mut search_from = 0;
    while let Some(pos) = tokens[search_from..].find("rename") {
        let actual_pos = search_from + pos;
        let rest = &tokens[actual_pos..];
        // Check if it's "rename_all"
        if rest.starts_with("rename_all") {
            search_from = actual_pos + 10;
            continue;
        }
        // Find the value between quotes after =
        if let Some(eq_pos) = rest.find('=') {
            let after_eq = &rest[eq_pos..];
            if let Some(quote_start) = after_eq.find('"') {
                let after_quote = &after_eq[quote_start + 1..];
                if let Some(quote_end) = after_quote.find('"') {
                    return Some(after_quote[..quote_end].to_string());
                }
            }
        }
        break;
    }
    None
}

impl InertiaPropsVisitor {
    fn parse_type(ty: &Type) -> RustType {
        match ty {
            Type::Path(type_path) => {
                let segment = type_path.path.segments.last().unwrap();
                let ident = segment.ident.to_string();

                match ident.as_str() {
                    "String" | "str" => RustType::String,
                    "i8" | "i16" | "i32" | "i64" | "i128" | "isize" | "u8" | "u16" | "u32"
                    | "u64" | "u128" | "usize" | "f32" | "f64" => RustType::Number,
                    "bool" => RustType::Bool,
                    "Option" => {
                        if let PathArguments::AngleBracketed(args) = &segment.arguments {
                            if let Some(GenericArgument::Type(inner_ty)) = args.args.first() {
                                return RustType::Option(Box::new(Self::parse_type(inner_ty)));
                            }
                        }
                        RustType::Option(Box::new(RustType::Custom("unknown".to_string())))
                    }
                    "Vec" => {
                        if let PathArguments::AngleBracketed(args) = &segment.arguments {
                            if let Some(GenericArgument::Type(inner_ty)) = args.args.first() {
                                return RustType::Vec(Box::new(Self::parse_type(inner_ty)));
                            }
                        }
                        RustType::Vec(Box::new(RustType::Custom("unknown".to_string())))
                    }
                    "HashMap" | "BTreeMap" => {
                        if let PathArguments::AngleBracketed(args) = &segment.arguments {
                            let mut iter = args.args.iter();
                            if let (
                                Some(GenericArgument::Type(key_ty)),
                                Some(GenericArgument::Type(val_ty)),
                            ) = (iter.next(), iter.next())
                            {
                                return RustType::HashMap(
                                    Box::new(Self::parse_type(key_ty)),
                                    Box::new(Self::parse_type(val_ty)),
                                );
                            }
                        }
                        RustType::HashMap(
                            Box::new(RustType::String),
                            Box::new(RustType::Custom("unknown".to_string())),
                        )
                    }
                    other => RustType::Custom(other.to_string()),
                }
            }
            Type::Reference(type_ref) => {
                // Handle &str as String
                if let Type::Path(inner) = &*type_ref.elem {
                    if inner
                        .path
                        .segments
                        .last()
                        .map(|s| s.ident == "str")
                        .unwrap_or(false)
                    {
                        return RustType::String;
                    }
                }
                Self::parse_type(&type_ref.elem)
            }
            _ => RustType::Custom("unknown".to_string()),
        }
    }
}

impl<'ast> Visit<'ast> for InertiaPropsVisitor {
    fn visit_item_struct(&mut self, node: &'ast ItemStruct) {
        if self.has_inertia_props_derive(&node.attrs) {
            let name = node.ident.to_string();
            let rename_all = Self::parse_serde_rename_all(&node.attrs);

            let fields = match &node.fields {
                Fields::Named(named) => named
                    .named
                    .iter()
                    .filter_map(|f| {
                        f.ident.as_ref().map(|ident| StructField {
                            name: ident.to_string(),
                            ty: Self::parse_type(&f.ty),
                            serde_rename: Self::parse_serde_field_rename(&f.attrs),
                        })
                    })
                    .collect(),
                _ => Vec::new(),
            };

            self.structs.push(InertiaPropsStruct {
                name,
                fields,
                rename_all,
            });
        }

        // Continue visiting nested items
        syn::visit::visit_item_struct(self, node);
    }
}

/// Scan all Rust files in the src directory for InertiaProps structs
pub fn scan_inertia_props(project_path: &Path) -> Vec<InertiaPropsStruct> {
    let src_path = project_path.join("src");
    let mut all_structs = Vec::new();

    for entry in WalkDir::new(&src_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map(|ext| ext == "rs").unwrap_or(false))
    {
        if let Ok(content) = fs::read_to_string(entry.path()) {
            if let Ok(syntax) = syn::parse_file(&content) {
                let mut visitor = InertiaPropsVisitor::new();
                visitor.visit_file(&syntax);
                all_structs.extend(visitor.structs);
            }
        }
    }

    all_structs
}

/// Convert a RustType to TypeScript type string
fn rust_type_to_ts(ty: &RustType) -> String {
    match ty {
        RustType::String => "string".to_string(),
        RustType::Number => "number".to_string(),
        RustType::Bool => "boolean".to_string(),
        RustType::Option(inner) => format!("{} | null", rust_type_to_ts(inner)),
        RustType::Vec(inner) => format!("{}[]", rust_type_to_ts(inner)),
        RustType::HashMap(key, val) => {
            format!("Record<{}, {}>", rust_type_to_ts(key), rust_type_to_ts(val))
        }
        RustType::Custom(name) => name.clone(),
    }
}

/// Sort structs topologically so dependencies come first
fn topological_sort(structs: &[InertiaPropsStruct]) -> Vec<&InertiaPropsStruct> {
    let struct_map: HashMap<_, _> = structs.iter().map(|s| (s.name.clone(), s)).collect();
    let struct_names: HashSet<_> = structs.iter().map(|s| s.name.clone()).collect();

    // Build dependency graph
    let mut deps: HashMap<String, HashSet<String>> = HashMap::new();
    for s in structs {
        let mut s_deps = HashSet::new();
        for field in &s.fields {
            collect_type_deps(&field.ty, &mut s_deps, &struct_names);
        }
        deps.insert(s.name.clone(), s_deps);
    }

    // Kahn's algorithm for topological sort
    let mut in_degree: HashMap<String, usize> =
        struct_names.iter().map(|n| (n.clone(), 0)).collect();
    for s_deps in deps.values() {
        for dep in s_deps {
            if let Some(count) = in_degree.get_mut(dep) {
                *count += 1;
            }
        }
    }

    let mut queue: Vec<_> = in_degree
        .iter()
        .filter(|(_, &count)| count == 0)
        .map(|(name, _)| name.clone())
        .collect();
    let mut result = Vec::new();

    while let Some(name) = queue.pop() {
        if let Some(s) = struct_map.get(&name) {
            result.push(*s);
        }
        if let Some(s_deps) = deps.get(&name) {
            for dep in s_deps {
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

fn collect_type_deps(ty: &RustType, deps: &mut HashSet<String>, known: &HashSet<String>) {
    match ty {
        RustType::Custom(name) if known.contains(name) => {
            deps.insert(name.clone());
        }
        RustType::Option(inner) | RustType::Vec(inner) => {
            collect_type_deps(inner, deps, known);
        }
        RustType::HashMap(key, val) => {
            collect_type_deps(key, deps, known);
            collect_type_deps(val, deps, known);
        }
        _ => {}
    }
}

/// Parse shared.ts to find exported type names
/// Returns set of exported type/interface/enum names
pub fn parse_shared_types(project_path: &Path) -> HashSet<String> {
    let shared_path = project_path.join("frontend/src/types/shared.ts");

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

/// Collect all custom types referenced in InertiaProps structs
fn collect_referenced_types(structs: &[InertiaPropsStruct]) -> HashSet<String> {
    let mut types = HashSet::new();

    for s in structs {
        for field in &s.fields {
            collect_custom_types(&field.ty, &mut types);
        }
    }

    types
}

/// Recursively collect custom type names from a RustType
fn collect_custom_types(ty: &RustType, types: &mut HashSet<String>) {
    match ty {
        RustType::Custom(name) => {
            types.insert(name.clone());
        }
        RustType::Option(inner) | RustType::Vec(inner) => {
            collect_custom_types(inner, types);
        }
        RustType::HashMap(key, val) => {
            collect_custom_types(key, types);
            collect_custom_types(val, types);
        }
        _ => {}
    }
}

/// Generate TypeScript interfaces from the structs
/// Apply serde renaming to get the final TypeScript field name
fn apply_field_rename(field: &StructField, rename_all: SerdeCase) -> String {
    // Per-field rename takes precedence over rename_all
    if let Some(ref rename) = field.serde_rename {
        return rename.clone();
    }
    // Apply struct-level rename_all
    rename_all.apply(&field.name)
}

/// Generate TypeScript interfaces from the structs
/// Optionally imports types from shared.ts if project_path is provided
#[allow(dead_code)]
pub fn generate_typescript(structs: &[InertiaPropsStruct]) -> String {
    generate_typescript_with_imports(structs, None)
}

/// Generate TypeScript interfaces with imports from shared.ts
pub fn generate_typescript_with_imports(
    structs: &[InertiaPropsStruct],
    project_path: Option<&Path>,
) -> String {
    generate_typescript_with_options(structs, project_path, true)
}

/// Generate TypeScript interfaces with full control over imports and re-exports
pub fn generate_typescript_with_options(
    structs: &[InertiaPropsStruct],
    project_path: Option<&Path>,
    include_reexports: bool,
) -> String {
    let sorted = topological_sort(structs);

    // Collect struct names (types defined in this file)
    let defined_types: HashSet<String> = structs.iter().map(|s| s.name.clone()).collect();

    // Parse shared.ts types
    let shared_types = project_path.map(parse_shared_types).unwrap_or_default();

    // Find types to import from shared.ts (only referenced ones)
    let mut imports_needed = Vec::new();
    if project_path.is_some() && !shared_types.is_empty() {
        let referenced_types = collect_referenced_types(structs);

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
    if include_reexports && !reexport_types.is_empty() {
        output.push_str("// Re-exports from shared.ts for convenience\n");
        output.push_str(&format!(
            "export type {{ {} }} from './shared';\n\n",
            reexport_types.join(", ")
        ));
    }

    for s in sorted {
        output.push_str(&format!("export interface {} {{\n", s.name));
        for field in &s.fields {
            let ts_type = rust_type_to_ts(&field.ty);
            let ts_name = apply_field_rename(field, s.rename_all);
            output.push_str(&format!("  {}: {};\n", ts_name, ts_type));
        }
        output.push_str("}\n\n");
    }

    output
}

/// Generate types and write to the output file
pub fn generate_types_to_file(project_path: &Path, output_path: &Path) -> Result<usize, String> {
    generate_types_to_file_with_options(project_path, output_path, true)
}

/// Generate types and write to the output file with options
pub fn generate_types_to_file_with_options(
    project_path: &Path,
    output_path: &Path,
    include_reexports: bool,
) -> Result<usize, String> {
    let structs = scan_inertia_props(project_path);

    if structs.is_empty() {
        return Ok(0);
    }

    // Ensure output directory exists
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create output directory: {}", e))?;
    }

    // Use the version with imports and re-exports
    let typescript =
        generate_typescript_with_options(&structs, Some(project_path), include_reexports);
    fs::write(output_path, typescript)
        .map_err(|e| format!("Failed to write TypeScript file: {}", e))?;

    Ok(structs.len())
}

/// Main entry point for the generate-types command
pub fn run(output: Option<String>, watch: bool, no_reexports: bool) {
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

    let output_path = output
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| project_path.join("frontend/src/types/inertia-props.ts"));

    println!("{}", style("Scanning for InertiaProps structs...").cyan());

    // include_reexports is the opposite of no_reexports
    let include_reexports = !no_reexports;
    match generate_types_to_file_with_options(project_path, &output_path, include_reexports) {
        Ok(0) => {
            println!("{}", style("No InertiaProps structs found.").yellow());
        }
        Ok(count) => {
            println!(
                "{} Found {} InertiaProps struct(s)",
                style("->").green(),
                count
            );
            println!("{} Generated {}", style("✓").green(), output_path.display());
        }
        Err(e) => {
            eprintln!("{} {}", style("Error:").red().bold(), e);
            std::process::exit(1);
        }
    }

    // Also generate route types
    generate_route_types(project_path);

    if watch {
        println!("{}", style("Watching for changes...").dim());
        if let Err(e) = start_watcher(project_path, &output_path) {
            eprintln!(
                "{} Failed to start watcher: {}",
                style("Error:").red().bold(),
                e
            );
            std::process::exit(1);
        }
    }
}

/// Generate route types
fn generate_route_types(project_path: &Path) {
    let routes_output = project_path.join("frontend/src/types/routes.ts");

    println!(
        "{}",
        style("Scanning routes for type-safe generation...").cyan()
    );

    match super::generate_routes::generate_routes_to_file(project_path, &routes_output) {
        Ok(0) => {
            println!("{}", style("No routes found in src/routes.rs").yellow());
        }
        Ok(count) => {
            println!("{} Found {} route(s)", style("->").green(), count);
            println!(
                "{} Generated {}",
                style("✓").green(),
                routes_output.display()
            );
        }
        Err(e) => {
            eprintln!(
                "{} Route generation error: {}",
                style("Warning:").yellow(),
                e
            );
        }
    }
}

/// Start file watcher for automatic type regeneration
fn start_watcher(project_path: &Path, output_path: &Path) -> Result<(), String> {
    use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
    use std::sync::mpsc::channel;
    use std::time::Duration;

    let (tx, rx) = channel();
    let src_path = project_path.join("src");

    let mut watcher = RecommendedWatcher::new(
        move |res| {
            if let Ok(event) = res {
                let _ = tx.send(event);
            }
        },
        Config::default().with_poll_interval(Duration::from_secs(1)),
    )
    .map_err(|e| format!("Failed to create watcher: {}", e))?;

    watcher
        .watch(&src_path, RecursiveMode::Recursive)
        .map_err(|e| format!("Failed to watch directory: {}", e))?;

    println!(
        "{} Watching {} for changes",
        style("->").cyan(),
        src_path.display()
    );

    let output_path = output_path.to_path_buf();
    let project_path = project_path.to_path_buf();

    loop {
        match rx.recv() {
            Ok(event) => {
                // Check if it's a Rust file change
                let is_rust_change = event
                    .paths
                    .iter()
                    .any(|p| p.extension().map(|e| e == "rs").unwrap_or(false));

                if is_rust_change {
                    println!("{}", style("Detected changes, regenerating types...").dim());
                    match generate_types_to_file(&project_path, &output_path) {
                        Ok(count) => {
                            println!("{} Regenerated {} type(s)", style("✓").green(), count);
                        }
                        Err(e) => {
                            eprintln!("{} Failed to regenerate: {}", style("Error:").red(), e);
                        }
                    }
                }
            }
            Err(e) => {
                return Err(format!("Watch error: {}", e));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snake_to_camel() {
        assert_eq!(snake_to_camel("created_at"), "createdAt");
        assert_eq!(snake_to_camel("user_id"), "userId");
        assert_eq!(snake_to_camel("some_long_name"), "someLongName");
        assert_eq!(snake_to_camel("name"), "name");
    }

    #[test]
    fn test_snake_to_pascal() {
        assert_eq!(snake_to_pascal("created_at"), "CreatedAt");
        assert_eq!(snake_to_pascal("user_id"), "UserId");
        assert_eq!(snake_to_pascal("some_long_name"), "SomeLongName");
        assert_eq!(snake_to_pascal("name"), "Name");
    }

    #[test]
    fn test_serde_case_apply() {
        assert_eq!(SerdeCase::CamelCase.apply("created_at"), "createdAt");
        assert_eq!(SerdeCase::PascalCase.apply("created_at"), "CreatedAt");
        assert_eq!(
            SerdeCase::ScreamingSnakeCase.apply("created_at"),
            "CREATED_AT"
        );
        assert_eq!(SerdeCase::KebabCase.apply("created_at"), "created-at");
        assert_eq!(SerdeCase::None.apply("created_at"), "created_at");
        assert_eq!(SerdeCase::SnakeCase.apply("created_at"), "created_at");
    }

    #[test]
    fn test_serde_case_from_str() {
        assert_eq!(SerdeCase::from_str("camelCase"), SerdeCase::CamelCase);
        assert_eq!(SerdeCase::from_str("PascalCase"), SerdeCase::PascalCase);
        assert_eq!(
            SerdeCase::from_str("SCREAMING_SNAKE_CASE"),
            SerdeCase::ScreamingSnakeCase
        );
        assert_eq!(SerdeCase::from_str("kebab-case"), SerdeCase::KebabCase);
        assert_eq!(SerdeCase::from_str("snake_case"), SerdeCase::SnakeCase);
        assert_eq!(SerdeCase::from_str("unknown"), SerdeCase::None);
    }

    #[test]
    fn test_parse_serde_rename_all_value() {
        let tokens = r#"rename_all = "camelCase""#;
        assert_eq!(
            parse_serde_rename_all_value(tokens),
            Some("camelCase".to_string())
        );

        let tokens = r#"derive(Serialize), rename_all = "PascalCase""#;
        assert_eq!(
            parse_serde_rename_all_value(tokens),
            Some("PascalCase".to_string())
        );

        let tokens = r#"skip_serializing"#;
        assert_eq!(parse_serde_rename_all_value(tokens), None);
    }

    #[test]
    fn test_parse_serde_rename_value() {
        let tokens = r#"rename = "customName""#;
        assert_eq!(
            parse_serde_rename_value(tokens),
            Some("customName".to_string())
        );

        // Should not match rename_all
        let tokens = r#"rename_all = "camelCase""#;
        assert_eq!(parse_serde_rename_value(tokens), None);

        // Should find rename after rename_all
        let tokens = r#"rename_all = "camelCase", rename = "custom""#;
        assert_eq!(parse_serde_rename_value(tokens), Some("custom".to_string()));
    }

    #[test]
    fn test_apply_field_rename() {
        let field = StructField {
            name: "created_at".to_string(),
            ty: RustType::String,
            serde_rename: None,
        };

        // With camelCase rename_all
        assert_eq!(
            apply_field_rename(&field, SerdeCase::CamelCase),
            "createdAt"
        );

        // With explicit rename override
        let field_with_rename = StructField {
            name: "created_at".to_string(),
            ty: RustType::String,
            serde_rename: Some("customField".to_string()),
        };
        assert_eq!(
            apply_field_rename(&field_with_rename, SerdeCase::CamelCase),
            "customField"
        );
    }

    #[test]
    fn test_generate_typescript_with_serde() {
        let structs = vec![InertiaPropsStruct {
            name: "TestProps".to_string(),
            fields: vec![
                StructField {
                    name: "user_id".to_string(),
                    ty: RustType::Number,
                    serde_rename: None,
                },
                StructField {
                    name: "created_at".to_string(),
                    ty: RustType::String,
                    serde_rename: None,
                },
                StructField {
                    name: "special_field".to_string(),
                    ty: RustType::String,
                    serde_rename: Some("overridden".to_string()),
                },
            ],
            rename_all: SerdeCase::CamelCase,
        }];

        let typescript = generate_typescript(&structs);

        assert!(typescript.contains("userId: number;"));
        assert!(typescript.contains("createdAt: string;"));
        assert!(typescript.contains("overridden: string;"));
        // Should NOT contain snake_case versions
        assert!(!typescript.contains("user_id:"));
        assert!(!typescript.contains("created_at:"));
        assert!(!typescript.contains("special_field:"));
    }

    #[test]
    fn test_collect_referenced_types() {
        let structs = vec![InertiaPropsStruct {
            name: "TestProps".to_string(),
            fields: vec![
                StructField {
                    name: "animal".to_string(),
                    ty: RustType::Custom("Animal".to_string()),
                    serde_rename: None,
                },
                StructField {
                    name: "user".to_string(),
                    ty: RustType::Option(Box::new(RustType::Custom("UserProfile".to_string()))),
                    serde_rename: None,
                },
                StructField {
                    name: "items".to_string(),
                    ty: RustType::Vec(Box::new(RustType::Custom("DiscoverAnimal".to_string()))),
                    serde_rename: None,
                },
                StructField {
                    name: "name".to_string(),
                    ty: RustType::String,
                    serde_rename: None,
                },
            ],
            rename_all: SerdeCase::None,
        }];

        let types = collect_referenced_types(&structs);

        assert!(types.contains("Animal"));
        assert!(types.contains("UserProfile"));
        assert!(types.contains("DiscoverAnimal"));
        // Should not contain primitive type indicators
        assert!(!types.contains("String"));
    }

    #[test]
    fn test_generate_typescript_with_imports() {
        // Create structs that reference external types
        let structs = vec![InertiaPropsStruct {
            name: "DiscoverProps".to_string(),
            fields: vec![
                StructField {
                    name: "animals".to_string(),
                    ty: RustType::Vec(Box::new(RustType::Custom("DiscoverAnimal".to_string()))),
                    serde_rename: None,
                },
                StructField {
                    name: "user".to_string(),
                    ty: RustType::Option(Box::new(RustType::Custom("UserProfile".to_string()))),
                    serde_rename: None,
                },
            ],
            rename_all: SerdeCase::None,
        }];

        // Test without project path (no imports)
        let typescript = generate_typescript(&structs);
        assert!(!typescript.contains("import type"));

        // Test with project path but no shared.ts file (tempdir scenario)
        let typescript =
            generate_typescript_with_imports(&structs, Some(std::path::Path::new("/nonexistent")));
        assert!(!typescript.contains("import type"));
    }
}
