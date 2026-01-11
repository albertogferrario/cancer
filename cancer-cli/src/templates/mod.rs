// Types for entity generation templates

/// Column information from database schema
pub struct ColumnInfo {
    pub name: String,
    pub col_type: String,
    pub is_nullable: bool,
    pub is_primary_key: bool,
}

/// Table information from database schema
pub struct TableInfo {
    pub name: String,
    pub columns: Vec<ColumnInfo>,
}

// Backend templates

pub fn cargo_toml(package_name: &str, description: &str, author: &str) -> String {
    let authors_line = if author.is_empty() {
        String::new()
    } else {
        format!("authors = [\"{}\"]\n", author)
    };

    format!(
        include_str!("files/backend/Cargo.toml.tpl"),
        package_name = package_name,
        description = description,
        authors_line = authors_line
    )
}

pub fn main_rs(package_name: &str) -> String {
    include_str!("files/backend/main.rs.tpl").replace("{package_name}", package_name)
}

pub fn routes_rs() -> &'static str {
    include_str!("files/backend/routes.rs.tpl")
}

pub fn controllers_mod() -> &'static str {
    include_str!("files/backend/controllers/mod.rs.tpl")
}

pub fn home_controller() -> &'static str {
    include_str!("files/backend/controllers/home.rs.tpl")
}

// Middleware templates

pub fn middleware_mod() -> &'static str {
    include_str!("files/backend/middleware/mod.rs.tpl")
}

pub fn middleware_logging() -> &'static str {
    include_str!("files/backend/middleware/logging.rs.tpl")
}

/// Template for generating new middleware with make:middleware command
pub fn middleware_template(name: &str, struct_name: &str) -> String {
    format!(
        r#"//! {name} middleware

use cancer::{{async_trait, Middleware, Next, Request, Response}};

/// {name} middleware
pub struct {struct_name};

#[async_trait]
impl Middleware for {struct_name} {{
    async fn handle(&self, request: Request, next: Next) -> Response {{
        // TODO: Implement middleware logic
        next(request).await
    }}
}}
"#,
        name = name,
        struct_name = struct_name
    )
}

/// Template for generating new controller with make:controller command
pub fn controller_template(name: &str) -> String {
    format!(
        r#"//! {name} controller

use cancer::{{handler, json_response, Request, Response}};

#[handler]
pub async fn invoke(_req: Request) -> Response {{
    json_response!({{
        "controller": "{name}"
    }})
}}
"#,
        name = name
    )
}

/// Template for generating new action with make:action command
pub fn action_template(name: &str, struct_name: &str) -> String {
    format!(
        r#"//! {name} action

use cancer::injectable;

#[injectable]
pub struct {struct_name} {{
    // Dependencies injected via container
}}

impl {struct_name} {{
    pub fn execute(&self) {{
        // TODO: Implement action logic
    }}
}}
"#,
        name = name,
        struct_name = struct_name
    )
}

/// Template for generating new Inertia page with make:inertia command
pub fn inertia_page_template(component_name: &str) -> String {
    format!(
        r#"export default function {component_name}() {{
  return (
    <div className="font-sans p-8 max-w-xl mx-auto">
      <h1 className="text-3xl font-bold">{component_name}</h1>
      <p className="mt-2">
        Edit <code className="bg-gray-100 px-1 rounded">frontend/src/pages/{component_name}.tsx</code> to get started.
      </p>
    </div>
  )
}}
"#,
        component_name = component_name
    )
}

/// Template for generating new error with make:error command
pub fn error_template(struct_name: &str) -> String {
    // Convert PascalCase to human readable message
    let mut message = String::new();
    for (i, c) in struct_name.chars().enumerate() {
        if c.is_uppercase() && i > 0 {
            message.push(' ');
            message.push(c.to_lowercase().next().unwrap());
        } else {
            message.push(c);
        }
    }

    format!(
        r#"//! {struct_name} error

use cancer::domain_error;

#[domain_error(status = 500, message = "{message}")]
pub struct {struct_name};
"#,
        struct_name = struct_name,
        message = message
    )
}

/// Template for models/mod.rs
pub fn models_mod() -> &'static str {
    include_str!("files/backend/models/mod.rs.tpl")
}

// Actions templates

pub fn actions_mod() -> &'static str {
    include_str!("files/backend/actions/mod.rs.tpl")
}

pub fn example_action() -> &'static str {
    include_str!("files/backend/actions/example_action.rs.tpl")
}

// Config templates

pub fn config_mod() -> &'static str {
    include_str!("files/backend/config/mod.rs.tpl")
}

pub fn config_database() -> &'static str {
    include_str!("files/backend/config/database.rs.tpl")
}

pub fn config_mail() -> &'static str {
    include_str!("files/backend/config/mail.rs.tpl")
}

pub fn bootstrap() -> &'static str {
    include_str!("files/backend/bootstrap.rs.tpl")
}

// Migrations templates

pub fn migrations_mod() -> &'static str {
    include_str!("files/backend/migrations/mod.rs.tpl")
}

// migrate_bin removed - migrations now integrated into main binary

// Frontend templates

pub fn package_json(project_name: &str) -> String {
    include_str!("files/frontend/package.json.tpl").replace("{project_name}", project_name)
}

pub fn vite_config() -> &'static str {
    include_str!("files/frontend/vite.config.ts.tpl")
}

pub fn tsconfig() -> &'static str {
    include_str!("files/frontend/tsconfig.json.tpl")
}

pub fn index_html(project_title: &str) -> String {
    include_str!("files/frontend/index.html.tpl").replace("{project_title}", project_title)
}

pub fn main_tsx() -> &'static str {
    include_str!("files/frontend/src/main.tsx.tpl")
}

pub fn home_page() -> &'static str {
    include_str!("files/frontend/src/pages/Home.tsx.tpl")
}

pub fn inertia_props_types() -> &'static str {
    include_str!("files/frontend/src/types/inertia-props.ts.tpl")
}

// Auth frontend templates

pub fn login_page() -> &'static str {
    include_str!("files/frontend/src/pages/auth/Login.tsx.tpl")
}

pub fn register_page() -> &'static str {
    include_str!("files/frontend/src/pages/auth/Register.tsx.tpl")
}

pub fn dashboard_page() -> &'static str {
    include_str!("files/frontend/src/pages/Dashboard.tsx.tpl")
}

// Auth backend templates

pub fn auth_controller() -> &'static str {
    include_str!("files/backend/controllers/auth.rs.tpl")
}

pub fn dashboard_controller() -> &'static str {
    include_str!("files/backend/controllers/dashboard.rs.tpl")
}

pub fn authenticate_middleware() -> &'static str {
    include_str!("files/backend/middleware/authenticate.rs.tpl")
}

pub fn user_model() -> &'static str {
    include_str!("files/backend/models/user.rs.tpl")
}

// Auth migration templates

pub fn create_users_migration() -> &'static str {
    include_str!("files/backend/migrations/create_users_table.rs.tpl")
}

pub fn create_sessions_migration() -> &'static str {
    include_str!("files/backend/migrations/create_sessions_table.rs.tpl")
}

// Root templates

pub fn gitignore() -> &'static str {
    include_str!("files/root/gitignore.tpl")
}

pub fn env(project_name: &str) -> String {
    include_str!("files/root/env.tpl").replace("{project_name}", project_name)
}

pub fn env_example() -> &'static str {
    include_str!("files/root/env.example.tpl")
}

// Entity generation templates for db:sync command

/// Rust reserved keywords that need escaping with r# prefix
const RUST_RESERVED_KEYWORDS: &[&str] = &[
    "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn", "for",
    "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return",
    "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use", "where",
    "while", "async", "await", "dyn", "abstract", "become", "box", "do", "final", "macro",
    "override", "priv", "typeof", "unsized", "virtual", "yield", "try",
];

/// Check if a name is a Rust reserved keyword
fn is_reserved_keyword(name: &str) -> bool {
    RUST_RESERVED_KEYWORDS.contains(&name)
}

/// Escape a column name if it's a reserved keyword
fn escape_column_name(name: &str) -> String {
    if is_reserved_keyword(name) {
        format!("r#{}", name)
    } else {
        name.to_string()
    }
}

/// Generate auto-generated entity file (regenerated on every sync)
pub fn entity_template(table_name: &str, columns: &[ColumnInfo]) -> String {
    let _struct_name = to_pascal_case(&singularize(table_name));

    // Generate column fields
    let column_fields: Vec<String> = columns
        .iter()
        .map(|col| {
            let rust_type = sql_type_to_rust_type(col);
            let mut attrs = Vec::new();

            if col.is_primary_key {
                attrs.push("    #[sea_orm(primary_key)]".to_string());
            }

            // Handle reserved keywords
            let field_name = escape_column_name(&col.name);
            if is_reserved_keyword(&col.name) {
                attrs.push(format!("    #[sea_orm(column_name = \"{}\")]", col.name));
            }

            let field = format!("    pub {}: {},", field_name, rust_type);
            if attrs.is_empty() {
                field
            } else {
                format!("{}\n{}", attrs.join("\n"), field)
            }
        })
        .collect();

    // Find primary key columns (reserved for future use)
    let _pk_columns: Vec<&ColumnInfo> = columns.iter().filter(|c| c.is_primary_key).collect();

    format!(
        r#"// AUTO-GENERATED FILE - DO NOT EDIT
// Generated by `cancer db:sync` - Changes will be overwritten
// Add custom code to src/models/{table_name}.rs instead

use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "{table_name}")]
pub struct Model {{
{columns}
}}

// Note: Relation enum is required here for DeriveEntityModel macro.
// Define your actual relations in src/models/{table_name}.rs using the Related trait.
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {{}}
"#,
        table_name = table_name,
        columns = column_fields.join("\n"),
    )
}

/// Generate user model file with Eloquent-like API (created only once, never overwritten)
pub fn user_model_template(table_name: &str, struct_name: &str, columns: &[ColumnInfo]) -> String {
    let model_setters = generate_model_setters(columns);
    let builder_fields = generate_builder_fields(columns);
    let builder_setters = generate_builder_setters(columns);
    let builder_to_active = generate_builder_to_active(columns);
    let model_to_active = generate_model_to_active(columns);
    let pk_field = columns
        .iter()
        .find(|c| c.is_primary_key)
        .map(|c| c.name.as_str())
        .unwrap_or("id");

    // Auto-implement Authenticatable for users table
    let authenticatable_impl = if table_name == "users" {
        format!(
            r#"
// ============================================================================
// AUTHENTICATION
// Auto-implemented Authenticatable trait for users table
// ============================================================================

impl cancer::auth::Authenticatable for Model {{
    fn auth_identifier(&self) -> i64 {{
        self.{pk_field} as i64
    }}

    fn as_any(&self) -> &dyn std::any::Any {{
        self
    }}
}}
"#,
            pk_field = pk_field
        )
    } else {
        String::new()
    };

    format!(
        r#"//! {struct_name} model
//!
//! This file contains custom implementations for the {struct_name} model.
//! The base entity is auto-generated in src/models/entities/{table_name}.rs
//!
//! This file is NEVER overwritten by `cancer db:sync` - your custom code is safe here.

// Re-export the auto-generated entity
pub use super::entities::{table_name}::*;

use cancer::database::{{ModelMut, QueryBuilder}};
use sea_orm::{{entity::prelude::*, Set}};

/// Type alias for convenient access
pub type {struct_name} = Model;

// ============================================================================
// ENTITY CONFIGURATION
// ============================================================================

impl ActiveModelBehavior for ActiveModel {{}}

impl cancer::database::Model for Entity {{}}
impl cancer::database::ModelMut for Entity {{}}

// ============================================================================
// ELOQUENT-LIKE API
// Fluent query builder and setter methods for {struct_name}
// ============================================================================

impl Model {{
    /// Start a new query builder
    ///
    /// # Example
    /// ```rust,ignore
    /// let records = {struct_name}::query().all().await?;
    /// let record = {struct_name}::query().filter(Column::Id.eq(1)).first().await?;
    /// ```
    pub fn query() -> QueryBuilder<Entity> {{
        QueryBuilder::new()
    }}

    /// Create a new record builder
    ///
    /// # Example
    /// ```rust,ignore
    /// let record = {struct_name}::create()
    ///     .set_field("value")
    ///     .insert()
    ///     .await?;
    /// ```
    pub fn create() -> {struct_name}Builder {{
        {struct_name}Builder::default()
    }}

{model_setters}
    /// Save changes to the database
    ///
    /// # Example
    /// ```rust,ignore
    /// let updated = record.set_field("new_value").update().await?;
    /// ```
    pub async fn update(self) -> Result<Self, cancer::FrameworkError> {{
        let active = self.to_active_model();
        Entity::update_one(active).await
    }}

    /// Delete this record from the database
    ///
    /// # Example
    /// ```rust,ignore
    /// record.delete().await?;
    /// ```
    pub async fn delete(self) -> Result<u64, cancer::FrameworkError> {{
        Entity::delete_by_pk(self.{pk_field}).await
    }}

    fn to_active_model(&self) -> ActiveModel {{
{model_to_active}
    }}
}}

// ============================================================================
// BUILDER
// For creating new records with fluent setter pattern
// ============================================================================

/// Builder for creating new {struct_name} records
#[derive(Default)]
pub struct {struct_name}Builder {{
{builder_fields}
}}

impl {struct_name}Builder {{
{builder_setters}
    /// Insert the record into the database
    ///
    /// # Example
    /// ```rust,ignore
    /// let record = {struct_name}::create()
    ///     .set_field("value")
    ///     .insert()
    ///     .await?;
    /// ```
    pub async fn insert(self) -> Result<Model, cancer::FrameworkError> {{
        let active = self.build();
        Entity::insert_one(active).await
    }}

    fn build(self) -> ActiveModel {{
{builder_to_active}
    }}
}}

// ============================================================================
// CUSTOM METHODS
// Add your custom query and mutation methods below
// ============================================================================

// Example custom finder:
// impl Model {{
//     pub async fn find_by_email(email: &str) -> Result<Option<Self>, cancer::FrameworkError> {{
//         Self::query().filter(Column::Email.eq(email)).first().await
//     }}
// }}

// ============================================================================
// RELATIONS
// Define relationships to other entities here
// ============================================================================

// Example: One-to-Many relation
// impl Entity {{
//     pub fn has_many_posts() -> RelationDef {{
//         Entity::has_many(super::posts::Entity).into()
//     }}
// }}

// Example: Belongs-To relation
// impl Entity {{
//     pub fn belongs_to_user() -> RelationDef {{
//         Entity::belongs_to(super::users::Entity)
//             .from(Column::UserId)
//             .to(super::users::Column::Id)
//             .into()
//     }}
// }}
{authenticatable_impl}"#,
        struct_name = struct_name,
        table_name = table_name,
        model_setters = model_setters,
        builder_fields = builder_fields,
        builder_setters = builder_setters,
        builder_to_active = builder_to_active,
        model_to_active = model_to_active,
        pk_field = pk_field,
        authenticatable_impl = authenticatable_impl,
    )
}

/// Generate entities/mod.rs (regenerated on every sync)
pub fn entities_mod_template(tables: &[TableInfo]) -> String {
    let mut content =
        String::from("// AUTO-GENERATED FILE - DO NOT EDIT\n// Generated by `cancer db:sync`\n\n");

    for table in tables {
        content.push_str(&format!("pub mod {};\n", table.name));
    }

    content
}

// Helper functions for entity generation

fn sql_type_to_rust_type(col: &ColumnInfo) -> String {
    let col_type_upper = col.col_type.to_uppercase();
    let base_type = if col_type_upper.contains("INT") {
        if col_type_upper.contains("BIGINT") || col_type_upper.contains("INT8") {
            "i64"
        } else if col_type_upper.contains("SMALLINT") || col_type_upper.contains("INT2") {
            "i16"
        } else {
            "i32"
        }
    } else if col_type_upper.contains("TEXT")
        || col_type_upper.contains("VARCHAR")
        || col_type_upper.contains("CHAR")
        || col_type_upper.contains("CHARACTER")
    {
        "String"
    } else if col_type_upper.contains("BOOL") {
        "bool"
    } else if col_type_upper.contains("REAL") || col_type_upper.contains("FLOAT4") {
        "f32"
    } else if col_type_upper.contains("DOUBLE") || col_type_upper.contains("FLOAT8") {
        "f64"
    } else if col_type_upper.contains("TIMESTAMP") || col_type_upper.contains("DATETIME") {
        "DateTimeUtc"
    } else if col_type_upper.contains("DATE") {
        "Date"
    } else if col_type_upper.contains("TIME") {
        "Time"
    } else if col_type_upper.contains("UUID") {
        "Uuid"
    } else if col_type_upper.contains("JSON") {
        "Json"
    } else if col_type_upper.contains("BYTEA") || col_type_upper.contains("BLOB") {
        "Vec<u8>"
    } else if col_type_upper.contains("DECIMAL") || col_type_upper.contains("NUMERIC") {
        "Decimal"
    } else {
        "String" // fallback
    };

    if col.is_nullable {
        format!("Option<{}>", base_type)
    } else {
        base_type.to_string()
    }
}

fn to_pascal_case(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = true;

    for c in s.chars() {
        if c == '_' || c == '-' || c == ' ' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(c.to_uppercase().next().unwrap());
            capitalize_next = false;
        } else {
            result.push(c);
        }
    }
    result
}

fn singularize(word: &str) -> String {
    if let Some(stem) = word.strip_suffix("ies") {
        format!("{}y", stem)
    } else if let Some(stem) = word.strip_suffix("es") {
        if word.ends_with("ses") || word.ends_with("xes") {
            word.to_string()
        } else {
            stem.to_string()
        }
    } else if let Some(stem) = word.strip_suffix('s') {
        if word.ends_with("ss") || word.ends_with("us") {
            word.to_string()
        } else {
            stem.to_string()
        }
    } else {
        word.to_string()
    }
}

// ============================================================================
// Eloquent-like API Code Generation Helpers
// ============================================================================

/// Generate setter methods for the Model (used for updates)
fn generate_model_setters(columns: &[ColumnInfo]) -> String {
    columns
        .iter()
        .filter(|c| !c.is_primary_key && !is_timestamp_field(&c.name))
        .map(|col| {
            let rust_type = sql_type_to_rust_type(col);
            let setter_input_type = get_setter_input_type(&rust_type, col.is_nullable);
            let field_name = escape_column_name(&col.name);

            format!(
                r#"    /// Set the {} field
    pub fn set_{setter_name}(mut self, value: {input_type}) -> Self {{
        self.{field} = {assignment};
        self
    }}

"#,
                col.name,
                setter_name = col.name, // Setter uses original name (set_type not set_r#type)
                field = field_name,     // Field access uses escaped name
                input_type = setter_input_type,
                assignment = get_setter_assignment(&rust_type, col.is_nullable),
            )
        })
        .collect::<Vec<_>>()
        .join("")
}

/// Generate builder struct fields
fn generate_builder_fields(columns: &[ColumnInfo]) -> String {
    columns
        .iter()
        .filter(|c| !c.is_primary_key)
        .map(|col| {
            let rust_type = sql_type_to_rust_type(col);
            let field_name = escape_column_name(&col.name);
            format!("    {}: Option<{}>,", field_name, rust_type)
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Generate setter methods for the Builder (used for creates)
fn generate_builder_setters(columns: &[ColumnInfo]) -> String {
    columns
        .iter()
        .filter(|c| !c.is_primary_key && !is_timestamp_field(&c.name))
        .map(|col| {
            let rust_type = sql_type_to_rust_type(col);
            let setter_input_type = get_builder_setter_input_type(&rust_type, col.is_nullable);
            let field_name = escape_column_name(&col.name);

            format!(
                r#"    /// Set the {} field
    pub fn set_{setter_name}(mut self, value: {input_type}) -> Self {{
        self.{field} = Some({builder_assignment});
        self
    }}

"#,
                col.name,
                setter_name = col.name, // Setter uses original name
                field = field_name,     // Field access uses escaped name
                input_type = setter_input_type,
                builder_assignment = get_builder_setter_assignment(&rust_type, col.is_nullable),
            )
        })
        .collect::<Vec<_>>()
        .join("")
}

/// Generate code to convert Builder to ActiveModel
fn generate_builder_to_active(columns: &[ColumnInfo]) -> String {
    let mut lines = vec!["        ActiveModel {".to_string()];

    for col in columns {
        let field_name = escape_column_name(&col.name);
        if col.is_primary_key {
            lines.push(format!(
                "            {}: sea_orm::ActiveValue::NotSet,",
                field_name
            ));
        } else {
            lines.push(format!(
                "            {field}: self.{field}.map(Set).unwrap_or(sea_orm::ActiveValue::NotSet),",
                field = field_name
            ));
        }
    }

    lines.push("        }".to_string());
    lines.join("\n")
}

/// Generate code to convert Model to ActiveModel for updates
fn generate_model_to_active(columns: &[ColumnInfo]) -> String {
    let mut lines = vec!["        ActiveModel {".to_string()];

    for col in columns {
        let rust_type = sql_type_to_rust_type(col);
        let needs_clone = needs_clone_for_type(&rust_type);
        let field_name = escape_column_name(&col.name);

        if needs_clone {
            lines.push(format!(
                "            {field}: Set(self.{field}.clone()),",
                field = field_name
            ));
        } else {
            lines.push(format!(
                "            {field}: Set(self.{field}),",
                field = field_name
            ));
        }
    }

    lines.push("        }".to_string());
    lines.join("\n")
}

/// Check if field is a timestamp field (auto-managed)
fn is_timestamp_field(name: &str) -> bool {
    matches!(name, "created_at" | "updated_at" | "deleted_at")
}

/// Get the input type for a setter method on Model
fn get_setter_input_type(rust_type: &str, is_nullable: bool) -> String {
    if is_nullable {
        // For Option<String>, accept Option<impl Into<String>>
        if rust_type == "Option<String>" {
            "Option<impl Into<String>>".to_string()
        } else {
            rust_type.to_string()
        }
    } else if rust_type == "String" {
        "impl Into<String>".to_string()
    } else {
        rust_type.to_string()
    }
}

/// Get the assignment expression for a setter on Model
fn get_setter_assignment(rust_type: &str, is_nullable: bool) -> String {
    if is_nullable {
        if rust_type == "Option<String>" {
            "value.map(|v| v.into())".to_string()
        } else {
            "value".to_string()
        }
    } else if rust_type == "String" {
        "value.into()".to_string()
    } else {
        "value".to_string()
    }
}

/// Get the input type for a builder setter method
fn get_builder_setter_input_type(rust_type: &str, is_nullable: bool) -> String {
    if is_nullable {
        // For nullable fields in builder, accept the inner type (not Option)
        if rust_type == "Option<String>" {
            "impl Into<String>".to_string()
        } else if rust_type.starts_with("Option<") && rust_type.ends_with(">") {
            // Extract inner type from Option<T>
            rust_type[7..rust_type.len() - 1].to_string()
        } else {
            rust_type.to_string()
        }
    } else if rust_type == "String" {
        "impl Into<String>".to_string()
    } else {
        rust_type.to_string()
    }
}

/// Get the assignment expression for a builder setter
fn get_builder_setter_assignment(rust_type: &str, is_nullable: bool) -> String {
    if is_nullable {
        // Wrap in Some for nullable fields
        if rust_type == "Option<String>" {
            "Some(value.into())".to_string()
        } else {
            "Some(value)".to_string()
        }
    } else if rust_type == "String" {
        "value.into()".to_string()
    } else {
        "value".to_string()
    }
}

/// Check if a type needs .clone() when converting
fn needs_clone_for_type(rust_type: &str) -> bool {
    // Types that implement Copy don't need clone
    let copy_types = [
        "i8", "i16", "i32", "i64", "i128", "u8", "u16", "u32", "u64", "u128", "f32", "f64", "bool",
    ];

    // Check if it's a Copy type
    if copy_types.contains(&rust_type) {
        return false;
    }

    // Option<Copy> types also don't need clone
    for copy_type in &copy_types {
        if rust_type == format!("Option<{}>", copy_type) {
            return false;
        }
    }

    // Everything else needs clone (String, Option<String>, DateTimeUtc, etc.)
    true
}

// ============================================================================
// Docker Templates
// ============================================================================

/// Generate Dockerfile for production deployment
pub fn dockerfile_template(package_name: &str) -> String {
    include_str!("files/docker/Dockerfile.tpl").replace("{package_name}", package_name)
}

/// Generate .dockerignore file
pub fn dockerignore_template() -> &'static str {
    include_str!("files/docker/dockerignore.tpl")
}

/// Generate docker-compose.yml for local development
pub fn docker_compose_template(
    project_name: &str,
    include_mailpit: bool,
    include_minio: bool,
) -> String {
    let mailpit_service = if include_mailpit {
        include_str!("files/docker/mailpit.service.tpl").replace("{project_name}", project_name)
    } else {
        String::new()
    };

    let minio_service = if include_minio {
        include_str!("files/docker/minio.service.tpl").replace("{project_name}", project_name)
    } else {
        String::new()
    };

    let additional_volumes = if include_minio {
        "\n  minio_data:".to_string()
    } else {
        String::new()
    };

    include_str!("files/docker/docker-compose.yml.tpl")
        .replace("{project_name}", project_name)
        .replace("{mailpit_service}", &mailpit_service)
        .replace("{minio_service}", &minio_service)
        .replace("{additional_volumes}", &additional_volumes)
}

// ============================================================================
// Schedule Templates
// ============================================================================

/// Template for schedule.rs registration file
pub fn schedule_rs() -> &'static str {
    include_str!("files/backend/schedule.rs.tpl")
}

/// Template for tasks/mod.rs
pub fn tasks_mod() -> &'static str {
    include_str!("files/backend/tasks/mod.rs.tpl")
}

// schedule_bin removed - scheduler now integrated into main binary

/// Template for generating new scheduled task with make:task command
pub fn task_template(file_name: &str, struct_name: &str) -> String {
    format!(
        r#"//! {struct_name} scheduled task
//!
//! Created with `cancer make:task {file_name}`

use async_trait::async_trait;
use cancer::{{Task, TaskResult}};

/// {struct_name} - A scheduled task
///
/// Implement your task logic in the `handle()` method.
/// Register this task in `src/schedule.rs` with the fluent API.
///
/// # Example Registration
///
/// ```rust,ignore
/// // In src/schedule.rs
/// use crate::tasks::{file_name};
///
/// schedule.add(
///     schedule.task({struct_name}::new())
///         .daily()
///         .at("03:00")
///         .name("{file_name}")
///         .description("TODO: Add task description")
/// );
/// ```
pub struct {struct_name};

impl {struct_name} {{
    /// Create a new instance of this task
    pub fn new() -> Self {{
        Self
    }}
}}

impl Default for {struct_name} {{
    fn default() -> Self {{
        Self::new()
    }}
}}

#[async_trait]
impl Task for {struct_name} {{
    async fn handle(&self) -> TaskResult {{
        // TODO: Implement your task logic here
        println!("Running {struct_name}...");
        Ok(())
    }}
}}
"#,
        file_name = file_name,
        struct_name = struct_name
    )
}

// Event templates

/// Template for generating new events with make:event command
pub fn event_template(file_name: &str, struct_name: &str) -> String {
    format!(
        r#"//! {struct_name} event
//!
//! Created with `cancer make:event {file_name}`

use cancer_events::Event;
use serde::{{Deserialize, Serialize}};

/// {struct_name} - A domain event
///
/// Events represent something that has happened in your application.
/// Listeners can react to these events asynchronously.
///
/// # Example
///
/// ```rust,ignore
/// use crate::events::{file_name}::{struct_name};
///
/// // Dispatch the event
/// {struct_name} {{ /* fields */ }}.dispatch().await?;
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct {struct_name} {{
    // TODO: Add event data fields
    // pub user_id: i64,
    // pub created_at: chrono::DateTime<chrono::Utc>,
}}

impl Event for {struct_name} {{}}
"#,
        file_name = file_name,
        struct_name = struct_name
    )
}

/// Template for events/mod.rs
pub fn events_mod() -> &'static str {
    r#"//! Application events
//!
//! This module contains domain events that can be dispatched
//! and handled by listeners.

"#
}

// Listener templates

/// Template for generating new listeners with make:listener command
pub fn listener_template(file_name: &str, struct_name: &str, event_type: &str) -> String {
    format!(
        r#"//! {struct_name} listener
//!
//! Created with `cancer make:listener {file_name}`

use cancer_events::{{async_trait, Error, Listener}};
// TODO: Import the event type
// use crate::events::your_event::YourEvent;

/// {struct_name} - An event listener
///
/// Listeners react to events and perform side effects.
/// They can be synchronous or queued for background processing.
///
/// # Example Registration
///
/// ```rust,ignore
/// // In your app initialization
/// use cancer_events::EventDispatcher;
/// use crate::listeners::{file_name}::{struct_name};
///
/// let mut dispatcher = EventDispatcher::new();
/// dispatcher.listen::<{event_type}, _>({struct_name});
/// ```
pub struct {struct_name};

#[async_trait]
impl Listener<{event_type}> for {struct_name} {{
    async fn handle(&self, event: &{event_type}) -> Result<(), Error> {{
        // TODO: Implement listener logic
        tracing::info!("{struct_name} handling event: {{:?}}", event);
        Ok(())
    }}
}}
"#,
        file_name = file_name,
        struct_name = struct_name,
        event_type = event_type
    )
}

/// Template for listeners/mod.rs
pub fn listeners_mod() -> &'static str {
    r#"//! Application event listeners
//!
//! This module contains listeners that react to domain events.

"#
}

// Job templates

/// Template for generating new jobs with make:job command
pub fn job_template(file_name: &str, struct_name: &str) -> String {
    format!(
        r#"//! {struct_name} background job
//!
//! Created with `cancer make:job {file_name}`

use cancer_queue::{{async_trait, Error, Job, Queueable}};
use serde::{{Deserialize, Serialize}};

/// {struct_name} - A background job
///
/// Jobs are queued for background processing by workers.
/// They support retries, delays, and queue prioritization.
///
/// # Example
///
/// ```rust,ignore
/// use crate::jobs::{file_name}::{struct_name};
///
/// // Dispatch immediately
/// {struct_name} {{ /* fields */ }}.dispatch().await?;
///
/// // Dispatch with delay
/// {struct_name} {{ /* fields */ }}
///     .delay(std::time::Duration::from_secs(60))
///     .dispatch()
///     .await?;
///
/// // Dispatch to specific queue
/// {struct_name} {{ /* fields */ }}
///     .on_queue("high-priority")
///     .dispatch()
///     .await?;
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct {struct_name} {{
    // TODO: Add job data fields
    // pub user_id: i64,
    // pub payload: String,
}}

#[async_trait]
impl Job for {struct_name} {{
    async fn handle(&self) -> Result<(), Error> {{
        // TODO: Implement job logic
        tracing::info!("Processing {struct_name}: {{:?}}", self);
        Ok(())
    }}

    fn max_retries(&self) -> u32 {{
        3
    }}

    fn retry_delay(&self, attempt: u32) -> std::time::Duration {{
        // Exponential backoff
        std::time::Duration::from_secs(2u64.pow(attempt))
    }}
}}
"#,
        file_name = file_name,
        struct_name = struct_name
    )
}

/// Template for jobs/mod.rs
pub fn jobs_mod() -> &'static str {
    r#"//! Application background jobs
//!
//! This module contains jobs that are processed asynchronously
//! by queue workers.

"#
}

// Notification templates

/// Template for generating new notifications with make:notification command
pub fn notification_template(file_name: &str, struct_name: &str) -> String {
    format!(
        r#"//! {struct_name} notification
//!
//! Created with `cancer make:notification {file_name}`

use cancer_notifications::{{Channel, DatabaseMessage, MailMessage, Notification}};

/// {struct_name} - A multi-channel notification
///
/// Notifications can be sent through multiple channels:
/// - Mail: Email via SMTP
/// - Database: In-app notifications
/// - Slack: Webhook messages
///
/// # Example
///
/// ```rust,ignore
/// use crate::notifications::{file_name}::{struct_name};
///
/// // Send notification to a user
/// user.notify({struct_name} {{ /* fields */ }}).await?;
/// ```
pub struct {struct_name} {{
    // TODO: Add notification data fields
    // pub order_id: i64,
    // pub tracking_number: String,
}}

impl Notification for {struct_name} {{
    fn via(&self) -> Vec<Channel> {{
        // TODO: Choose notification channels
        vec![Channel::Mail, Channel::Database]
    }}

    fn to_mail(&self) -> Option<MailMessage> {{
        Some(MailMessage::new()
            .subject("{struct_name}")
            .body("TODO: Add notification message"))
    }}

    fn to_database(&self) -> Option<DatabaseMessage> {{
        Some(DatabaseMessage::new("{file_name}")
            // TODO: Add notification data
            // .data("order_id", self.order_id)
        )
    }}
}}
"#,
        file_name = file_name,
        struct_name = struct_name
    )
}

/// Template for notifications/mod.rs
pub fn notifications_mod() -> &'static str {
    r#"//! Application notifications
//!
//! This module contains notifications that can be sent
//! through multiple channels (mail, database, slack, etc.).

"#
}

// ============================================================================
// AI Development Boost Templates
// ============================================================================

/// Cancer framework guidelines for AI assistants
pub fn cancer_guidelines_template() -> &'static str {
    r#"# Cancer Framework Guidelines

Cancer is a Rust web framework inspired by Laravel, providing a familiar developer experience with Rust's performance and safety.

## Project Structure

```
app/
├── src/
│   ├── main.rs           # Application entry point
│   ├── routes.rs         # Route definitions
│   ├── bootstrap.rs      # Application bootstrap
│   ├── controllers/      # Request handlers
│   ├── middleware/       # HTTP middleware
│   ├── models/           # Database models (SeaORM entities)
│   │   └── entities/     # Auto-generated entities (do not edit)
│   ├── actions/          # Business logic actions
│   ├── events/           # Domain events
│   ├── listeners/        # Event listeners
│   ├── jobs/             # Background jobs
│   ├── notifications/    # Multi-channel notifications
│   ├── tasks/            # Scheduled tasks
│   ├── config/           # Configuration modules
│   └── migrations/       # Database migrations
└── Cargo.toml
frontend/
├── src/
│   ├── main.tsx          # Frontend entry
│   └── pages/            # Inertia.js pages (React/TypeScript)
└── package.json
```

## Key Conventions

### Controllers
- Use the `#[handler]` macro for route handlers
- Return `Response` type using helper macros

```rust
use cancer::{handler, json_response, Request, Response};

#[handler]
pub async fn index(_req: Request) -> Response {
    json_response!({ "message": "Hello" })
}
```

### Middleware
- Implement the `Middleware` trait
- Use `#[async_trait]` for async methods

```rust
use cancer::{async_trait, Middleware, Next, Request, Response};

pub struct MyMiddleware;

#[async_trait]
impl Middleware for MyMiddleware {
    async fn handle(&self, request: Request, next: Next) -> Response {
        // Before request
        let response = next(request).await;
        // After request
        response
    }
}
```

### Models (SeaORM)
- Models use SeaORM with an Eloquent-like API
- Entity files in `models/entities/` are auto-generated
- Custom logic goes in `models/{table_name}.rs`

```rust
// Query builder pattern
let users = User::query()
    .filter(Column::Active.eq(true))
    .all()
    .await?;

// Fluent create
let user = User::create()
    .set_email("user@example.com")
    .set_name("John")
    .insert()
    .await?;

// Fluent update
let updated = user
    .set_name("Jane")
    .update()
    .await?;
```

### Inertia.js Integration
- Backend sends data via `inertia_response!` macro
- Frontend receives as props in React components
- TypeScript types auto-generated from Rust structs

```rust
// Backend
#[handler]
pub async fn show(req: Request) -> Response {
    inertia_response!("Users/Show", {
        "user": user,
        "posts": posts
    })
}
```

### Database Migrations
- Create with `cancer make:migration <name>`
- Run with `cancer migrate`
- Sync models with `cancer db:sync`

### Error Handling
- Use `#[domain_error]` macro for custom errors
- Errors automatically convert to appropriate HTTP responses

```rust
use cancer::domain_error;

#[domain_error(status = 404, message = "User not found")]
pub struct UserNotFound;
```

## CLI Commands

- `cancer new <name>` - Create new project
- `cancer serve` - Start dev servers
- `cancer make:controller <name>` - Generate controller
- `cancer make:middleware <name>` - Generate middleware
- `cancer make:migration <name>` - Generate migration
- `cancer make:event <name>` - Generate event
- `cancer make:job <name>` - Generate background job
- `cancer migrate` - Run migrations
- `cancer db:sync` - Sync DB schema to entities
- `cancer mcp` - Start MCP server for AI assistance

## Best Practices

1. **Use Actions for Business Logic**: Keep controllers thin, move logic to action classes
2. **Leverage the Type System**: Use Rust's types for validation and safety
3. **Auto-generate Types**: Run `cancer generate-types` to sync Rust structs to TypeScript
4. **Database Sync**: Use `cancer db:sync` after migrations to update entity files
5. **Middleware Order**: Register middleware in the correct order in routes.rs
"#
}

/// Cursor-specific rules file
pub fn cursor_rules_template() -> &'static str {
    r#"# Cancer Framework - Cursor Rules

You are working on a Cancer framework project. Cancer is a Rust web framework inspired by Laravel.

## Framework Knowledge

- Cancer uses Rust with async/await for the backend
- Frontend uses React + TypeScript with Inertia.js
- Database layer uses SeaORM with an Eloquent-like API
- The project follows Laravel conventions adapted for Rust

## Code Style

- Use `#[handler]` macro for route handlers
- Use `#[async_trait]` for middleware
- Use `#[domain_error]` for custom errors
- Follow Rust naming conventions (snake_case for functions, PascalCase for types)

## When Generating Code

1. Controllers go in `app/src/controllers/`
2. Middleware goes in `app/src/middleware/`
3. Models go in `app/src/models/`
4. React pages go in `frontend/src/pages/`

## Available MCP Tools

Use the Cancer MCP tools for introspection:
- `application_info` - Get app info, versions, crates
- `list_routes` - See all defined routes
- `db_schema` - Get database schema
- `db_query` - Run read-only SQL queries
- `list_migrations` - Check migration status
- `list_middleware` - See registered middleware
- `read_logs` - Read application logs
- `last_error` - Get recent errors
- `tinker` - Execute Rust code in app context
- `browser_logs` - Read frontend error logs

## Common Patterns

### Adding a new page
1. Create controller handler in `app/src/controllers/`
2. Add route in `app/src/routes.rs`
3. Create React page in `frontend/src/pages/`
4. Run `cancer generate-types` to sync types

### Adding a database table
1. `cancer make:migration create_table_name`
2. Edit migration file
3. `cancer migrate`
4. `cancer db:sync`
"#
}

/// CLAUDE.md template for Claude Code
pub fn claude_md_template() -> &'static str {
    r#"# Project Instructions

This is a Cancer framework project - a Rust web framework inspired by Laravel.

## Quick Reference

- **Backend**: Rust with async/await, SeaORM for database
- **Frontend**: React + TypeScript with Inertia.js
- **CLI**: Use `cancer` command for scaffolding

## MCP Tools Available

The Cancer MCP server provides these introspection tools:
- `application_info`, `list_routes`, `db_schema`, `db_query`
- `list_migrations`, `list_middleware`, `list_events`, `list_jobs`
- `read_logs`, `last_error`, `browser_logs`, `tinker`

## Development Workflow

1. Use `cancer serve` to start dev servers
2. Use `cancer make:*` commands for scaffolding
3. Use `cancer db:sync` after migrations to update models
4. Use `cancer generate-types` to sync TypeScript types

## Cancer Framework Guidelines

See `.ai/guidelines/cancer.md` for detailed framework conventions.
"#
}

/// Section to append to existing CLAUDE.md
pub fn claude_md_cancer_section() -> &'static str {
    r#"
---

# Cancer Framework

This is a Cancer framework project - a Rust web framework inspired by Laravel.

## MCP Tools Available

The Cancer MCP server provides introspection tools:
- `application_info`, `list_routes`, `db_schema`, `db_query`
- `list_migrations`, `list_middleware`, `list_events`, `list_jobs`
- `read_logs`, `last_error`, `browser_logs`, `tinker`

## Framework Conventions

See `.ai/guidelines/cancer.md` for detailed framework conventions.
"#
}

/// GitHub Copilot instructions
pub fn copilot_instructions_template() -> &'static str {
    r#"# GitHub Copilot Instructions

## Project Type
This is a Cancer framework project (Rust web framework inspired by Laravel).

## Key Files
- `app/src/routes.rs` - Route definitions
- `app/src/controllers/` - Request handlers
- `app/src/models/` - Database models (SeaORM)
- `frontend/src/pages/` - React/TypeScript pages

## Code Patterns

### Controller Handler
```rust
use cancer::{handler, json_response, Request, Response};

#[handler]
pub async fn index(_req: Request) -> Response {
    json_response!({ "data": value })
}
```

### SeaORM Query
```rust
let items = Model::query()
    .filter(Column::Field.eq(value))
    .all()
    .await?;
```

### Inertia Response
```rust
inertia_response!("PageName", { "prop": value })
```

## Conventions
- Controllers are async handlers with `#[handler]` macro
- Models use SeaORM with Eloquent-like query builder
- Frontend pages receive data as Inertia props
- TypeScript types are auto-generated from Rust structs
"#
}
