use std::fs;
use std::path::Path;

pub fn run(name: String, fields: Vec<String>) {
    // Validate resource name
    if !is_valid_identifier(&name) {
        eprintln!(
            "Error: '{}' is not a valid identifier. Use PascalCase (e.g., Post, UserProfile)",
            name
        );
        std::process::exit(1);
    }

    // Parse fields
    let parsed_fields = match parse_fields(&fields) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error parsing fields: {}", e);
            std::process::exit(1);
        }
    };

    let snake_name = to_snake_case(&name);
    let plural_snake = pluralize(&snake_name);

    println!("🚀 Scaffolding {}...\n", name);

    // Generate migration
    generate_migration(&name, &snake_name, &plural_snake, &parsed_fields);

    // Generate model (includes entity)
    generate_model(&name, &snake_name, &parsed_fields);

    // Generate controller
    generate_controller(&name, &snake_name, &plural_snake, &parsed_fields);

    // Generate Inertia pages
    generate_inertia_pages(&name, &snake_name, &plural_snake, &parsed_fields);

    // Print route registration instructions
    print_route_instructions(&name, &snake_name, &plural_snake);

    println!("\n✅ Scaffold for {} created successfully!", name);
}

#[derive(Debug, Clone)]
struct Field {
    name: String,
    field_type: FieldType,
}

#[derive(Debug, Clone)]
enum FieldType {
    String,
    Text,
    Integer,
    BigInteger,
    Float,
    Boolean,
    DateTime,
    Date,
    Uuid,
}

impl FieldType {
    fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "string" | "str" => Ok(FieldType::String),
            "text" => Ok(FieldType::Text),
            "int" | "integer" | "i32" => Ok(FieldType::Integer),
            "bigint" | "biginteger" | "i64" => Ok(FieldType::BigInteger),
            "float" | "f64" | "double" => Ok(FieldType::Float),
            "bool" | "boolean" => Ok(FieldType::Boolean),
            "datetime" | "timestamp" => Ok(FieldType::DateTime),
            "date" => Ok(FieldType::Date),
            "uuid" => Ok(FieldType::Uuid),
            _ => Err(format!("Unknown field type: '{}'. Valid types: string, text, integer, bigint, float, bool, datetime, date, uuid", s)),
        }
    }

    fn to_rust_type(&self) -> &'static str {
        match self {
            FieldType::String => "String",
            FieldType::Text => "String",
            FieldType::Integer => "i32",
            FieldType::BigInteger => "i64",
            FieldType::Float => "f64",
            FieldType::Boolean => "bool",
            FieldType::DateTime => "chrono::DateTime<chrono::Utc>",
            FieldType::Date => "chrono::NaiveDate",
            FieldType::Uuid => "uuid::Uuid",
        }
    }

    fn to_sea_orm_method(&self) -> &'static str {
        match self {
            FieldType::String => "string()",
            FieldType::Text => "text()",
            FieldType::Integer => "integer()",
            FieldType::BigInteger => "big_integer()",
            FieldType::Float => "double()",
            FieldType::Boolean => "boolean()",
            FieldType::DateTime => "timestamp_with_time_zone()",
            FieldType::Date => "date()",
            FieldType::Uuid => "uuid()",
        }
    }

    fn to_typescript_type(&self) -> &'static str {
        match self {
            FieldType::String => "string",
            FieldType::Text => "string",
            FieldType::Integer => "number",
            FieldType::BigInteger => "number",
            FieldType::Float => "number",
            FieldType::Boolean => "boolean",
            FieldType::DateTime => "string",
            FieldType::Date => "string",
            FieldType::Uuid => "string",
        }
    }

    fn to_form_input_type(&self) -> &'static str {
        match self {
            FieldType::String => "text",
            FieldType::Text => "textarea",
            FieldType::Integer => "number",
            FieldType::BigInteger => "number",
            FieldType::Float => "number",
            FieldType::Boolean => "checkbox",
            FieldType::DateTime => "datetime-local",
            FieldType::Date => "date",
            FieldType::Uuid => "text",
        }
    }

    fn to_validation_attr(&self) -> &'static str {
        match self {
            FieldType::String => "#[rule(required, string)]",
            FieldType::Text => "#[rule(required, string)]",
            FieldType::Integer => "#[rule(required, integer)]",
            FieldType::BigInteger => "#[rule(required, integer)]",
            FieldType::Float => "#[rule(required, numeric)]",
            FieldType::Boolean => "#[rule(required, boolean)]",
            FieldType::DateTime => "#[rule(required, date)]",
            FieldType::Date => "#[rule(required, date)]",
            FieldType::Uuid => "#[rule(required, string)]",
        }
    }
}

fn parse_fields(fields: &[String]) -> Result<Vec<Field>, String> {
    let mut parsed = Vec::new();

    for field_str in fields {
        let parts: Vec<&str> = field_str.split(':').collect();
        if parts.len() != 2 {
            return Err(format!(
                "Invalid field format: '{}'. Expected format: name:type (e.g., title:string)",
                field_str
            ));
        }

        let name = parts[0].to_string();
        if !is_valid_field_name(&name) {
            return Err(format!(
                "Invalid field name: '{}'. Use snake_case (e.g., user_id)",
                name
            ));
        }

        let field_type = FieldType::from_str(parts[1])?;

        parsed.push(Field { name, field_type });
    }

    Ok(parsed)
}

fn is_valid_identifier(name: &str) -> bool {
    if name.is_empty() {
        return false;
    }
    let first = name.chars().next().unwrap();
    if !first.is_ascii_uppercase() {
        return false;
    }
    name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
}

fn is_valid_field_name(name: &str) -> bool {
    if name.is_empty() {
        return false;
    }
    let first = name.chars().next().unwrap();
    if !first.is_ascii_lowercase() && first != '_' {
        return false;
    }
    name.chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
}

fn to_snake_case(name: &str) -> String {
    let mut result = String::new();
    for (i, c) in name.chars().enumerate() {
        if c.is_uppercase() {
            if i > 0 {
                result.push('_');
            }
            result.push(c.to_ascii_lowercase());
        } else {
            result.push(c);
        }
    }
    result
}

fn to_pascal_case(name: &str) -> String {
    name.split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().chain(chars).collect(),
            }
        })
        .collect()
}

fn pluralize(name: &str) -> String {
    // Simple pluralization rules
    if name.ends_with('s') || name.ends_with('x') || name.ends_with("ch") || name.ends_with("sh") {
        format!("{}es", name)
    } else if name.ends_with('y')
        && !name.ends_with("ay")
        && !name.ends_with("ey")
        && !name.ends_with("oy")
        && !name.ends_with("uy")
    {
        format!("{}ies", &name[..name.len() - 1])
    } else {
        format!("{}s", name)
    }
}

fn generate_migration(_name: &str, _snake_name: &str, plural_snake: &str, fields: &[Field]) {
    // Check for both possible migration directory locations
    let migrations_dir = if Path::new("src/migrations").exists() {
        Path::new("src/migrations")
    } else if Path::new("src/database/migrations").exists() {
        Path::new("src/database/migrations")
    } else {
        eprintln!("Error: migrations directory not found. Are you in a Cancer project?");
        eprintln!("Expected: src/migrations or src/database/migrations");
        std::process::exit(1);
    };

    // Generate timestamp
    let timestamp = chrono::Utc::now().format("%Y%m%d%H%M%S").to_string();
    let migration_name = format!("m{}_create_{}_table", timestamp, plural_snake);
    let file_name = format!("{}.rs", migration_name);
    let file_path = migrations_dir.join(&file_name);

    // Build column definitions
    let mut columns = String::new();
    for field in fields {
        columns.push_str(&format!(
            "            .col(ColumnDef::new({name}::{column}).{method}.not_null())\n",
            name = to_pascal_case(plural_snake),
            column = to_pascal_case(&field.name),
            method = field.field_type.to_sea_orm_method()
        ));
    }

    let migration_content = format!(
        r#"use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {{
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {{
        manager
            .create_table(
                Table::create()
                    .table({table_enum}::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new({table_enum}::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
{columns}                    .col(
                        ColumnDef::new({table_enum}::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new({table_enum}::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }}

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {{
        manager
            .drop_table(Table::drop().table({table_enum}::Table).to_owned())
            .await
    }}
}}

#[derive(Iden)]
pub enum {table_enum} {{
    Table,
    Id,
{iden_columns}    CreatedAt,
    UpdatedAt,
}}
"#,
        table_enum = to_pascal_case(plural_snake),
        columns = columns,
        iden_columns = fields
            .iter()
            .map(|f| format!("    {},\n", to_pascal_case(&f.name)))
            .collect::<String>()
    );

    fs::write(&file_path, migration_content).expect("Failed to write migration file");

    // Update mod.rs
    update_migrations_mod(&migration_name);

    println!(
        "   📦 Created migration: {}/{}",
        migrations_dir.display(),
        file_name
    );
}

fn update_migrations_mod(migration_name: &str) {
    // Check for both possible mod.rs locations
    let mod_path = if Path::new("src/migrations/mod.rs").exists() {
        Path::new("src/migrations/mod.rs")
    } else if Path::new("src/database/migrations/mod.rs").exists() {
        Path::new("src/database/migrations/mod.rs")
    } else {
        eprintln!("Warning: migrations/mod.rs not found");
        return;
    };

    let content = fs::read_to_string(mod_path).expect("Failed to read mod.rs");

    // Add module declaration
    let mod_declaration = format!("pub mod {};", migration_name);
    if content.contains(&mod_declaration) {
        return;
    }

    // Find where to insert module declaration (after existing pub mod lines)
    let mut lines: Vec<&str> = content.lines().collect();
    let mut insert_index = 0;

    for (i, line) in lines.iter().enumerate() {
        if line.starts_with("pub mod m") {
            insert_index = i + 1;
        }
    }

    lines.insert(insert_index, &mod_declaration);

    // Also add to Migrator
    let migrator_addition = format!("            Box::new({}::Migration),", migration_name);

    let mut updated_lines = Vec::new();
    for line in lines {
        updated_lines.push(line.to_string());
        if line.contains("fn migrations()") {
            // Find the vec![ line and add before closing ]
        }
    }

    // Simple approach: find "]" line in migrations() and insert before it
    let content = updated_lines.join("\n");
    let content = if content.contains("vec![]") {
        content.replace(
            "vec![]",
            &format!("vec![\n{}\n        ]", migrator_addition),
        )
    } else if content.contains("vec![") {
        // Find last ] in migrations function and insert before it
        let mut result = String::new();
        let mut in_migrations = false;
        let mut bracket_depth = 0;

        for line in content.lines() {
            if line.contains("fn migrations()") {
                in_migrations = true;
            }

            if in_migrations {
                if line.contains("vec![") {
                    bracket_depth += 1;
                }
                if line.trim() == "]" && bracket_depth == 1 {
                    result.push_str(&migrator_addition);
                    result.push('\n');
                    bracket_depth = 0;
                    in_migrations = false;
                }
            }

            result.push_str(line);
            result.push('\n');
        }

        result
    } else {
        content
    };

    fs::write(mod_path, content).expect("Failed to write mod.rs");
}

fn generate_model(name: &str, snake_name: &str, fields: &[Field]) {
    let models_dir = Path::new("src/models");

    if !models_dir.exists() {
        fs::create_dir_all(models_dir).expect("Failed to create models directory");
    }

    let file_path = models_dir.join(format!("{}.rs", snake_name));

    // Build field definitions for the entity
    let mut field_defs = String::new();
    for field in fields {
        field_defs.push_str(&format!(
            "    pub {}: {},\n",
            field.name,
            field.field_type.to_rust_type()
        ));
    }

    let model_content = format!(
        r#"//! {name} model

use cancer::database::{{Model as DatabaseModel, ModelMut, QueryBuilder}};
use sea_orm::entity::prelude::*;
use sea_orm::Set;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "{table_name}")]
pub struct Model {{
    #[sea_orm(primary_key)]
    pub id: i64,
{field_defs}    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {{}}

impl ActiveModelBehavior for ActiveModel {{}}

impl DatabaseModel for Entity {{}}
impl ModelMut for Entity {{}}

/// Type alias for convenient access
pub type {name} = Model;

impl Model {{
    /// Start a query builder
    pub fn query() -> QueryBuilder<Entity> {{
        QueryBuilder::new()
    }}
}}
"#,
        name = name,
        table_name = pluralize(snake_name),
        field_defs = field_defs
    );

    fs::write(&file_path, model_content).expect("Failed to write model file");

    // Update mod.rs
    update_models_mod(snake_name);

    println!("   📦 Created model: src/models/{}.rs", snake_name);
}

fn update_models_mod(snake_name: &str) {
    let mod_path = Path::new("src/models/mod.rs");

    if !mod_path.exists() {
        let content = format!(
            "pub mod {snake};\npub use {snake}::*;\n",
            snake = snake_name
        );
        fs::write(mod_path, content).expect("Failed to write mod.rs");
        return;
    }

    let content = fs::read_to_string(mod_path).expect("Failed to read mod.rs");
    let mod_declaration = format!("pub mod {};", snake_name);

    if content.contains(&mod_declaration) {
        return;
    }

    let updated = format!(
        "{}{}\npub use {}::*;\n",
        content, mod_declaration, snake_name
    );
    fs::write(mod_path, updated).expect("Failed to write mod.rs");
}

fn generate_controller(name: &str, snake_name: &str, plural_snake: &str, fields: &[Field]) {
    let controllers_dir = Path::new("src/controllers");

    if !controllers_dir.exists() {
        fs::create_dir_all(controllers_dir).expect("Failed to create controllers directory");
    }

    let file_path = controllers_dir.join(format!("{}_controller.rs", snake_name));

    // Build update field assignments
    let mut update_fields = String::new();
    for field in fields {
        update_fields.push_str(&format!(
            "    model.{} = sea_orm::ActiveValue::Set(form.{}.clone());\n",
            field.name, field.name
        ));
    }

    // Build form struct fields with validation attributes
    let mut form_fields = String::new();
    for field in fields {
        let rule_attr = field.field_type.to_validation_attr();
        form_fields.push_str(&format!(
            "    {}\n    pub {}: {},\n",
            rule_attr,
            field.name,
            field.field_type.to_rust_type()
        ));
    }

    let controller_content = format!(
        r#"use cancer::{{
    http::{{Request, Response, HttpResponse}},
    inertia::{{Inertia, SavedInertiaContext}},
    validation::Validatable,
    ValidateRules,
}};
use sea_orm::{{EntityTrait, ActiveModelTrait, ActiveValue}};
use serde::{{Deserialize, Serialize}};

use crate::models::{snake_name}::{{self, Entity, Model as {name}}};

#[derive(Debug, Deserialize, Serialize, ValidateRules)]
pub struct {name}Form {{
{form_fields}}}

#[derive(Debug, Serialize)]
pub struct {plural_pascal}IndexProps {{
    pub {plural}: Vec<{name}>,
}}

#[derive(Debug, Serialize)]
pub struct {name}ShowProps {{
    pub {snake}: {name},
}}

#[derive(Debug, Serialize)]
pub struct {name}CreateProps {{
    pub errors: Option<std::collections::HashMap<String, Vec<String>>>,
}}

#[derive(Debug, Serialize)]
pub struct {name}EditProps {{
    pub {snake}: {name},
    pub errors: Option<std::collections::HashMap<String, Vec<String>>>,
}}

/// List all {plural}
pub async fn index(req: Request) -> Response {{
    let db = req.db();
    let {plural} = {snake_name}::Entity::find()
        .all(db)
        .await
        .map_err(|e| HttpResponse::internal_server_error(e.to_string()))?;

    Inertia::render(&req, "{plural}/Index", {plural_pascal}IndexProps {{ {plural} }})
}}

/// Show a single {snake}
pub async fn show(req: Request, id: i64) -> Response {{
    let db = req.db();
    let {snake} = {snake_name}::Entity::find_by_id(id)
        .one(db)
        .await
        .map_err(|e| HttpResponse::internal_server_error(e.to_string()))?
        .ok_or_else(|| HttpResponse::not_found("{name} not found"))?;

    Inertia::render(&req, "{plural}/Show", {name}ShowProps {{ {snake} }})
}}

/// Show create form
pub async fn create(req: Request) -> Response {{
    Inertia::render(&req, "{plural}/Create", {name}CreateProps {{ errors: None }})
}}

/// Store a new {snake}
pub async fn store(req: Request) -> Response {{
    let ctx = SavedInertiaContext::from(&req);
    let form: {name}Form = req.input().await.map_err(|e| {{
        HttpResponse::bad_request(format!("Invalid form data: {{}}", e))
    }})?;

    // Validate using derive macro
    if let Err(errors) = form.validate() {{
        return Inertia::render_ctx(&ctx, "{plural}/Create", {name}CreateProps {{
            errors: Some(errors.into_messages()),
        }});
    }}

    let db = req.db();
    let model = {snake_name}::ActiveModel {{
        id: ActiveValue::NotSet,
{insert_fields}        created_at: ActiveValue::Set(chrono::Utc::now()),
        updated_at: ActiveValue::Set(chrono::Utc::now()),
    }};

    let result = model.insert(db).await
        .map_err(|e| HttpResponse::internal_server_error(e.to_string()))?;

    HttpResponse::redirect(&format!("/{plural}/{{}}", result.id))
}}

/// Show edit form
pub async fn edit(req: Request, id: i64) -> Response {{
    let db = req.db();
    let {snake} = {snake_name}::Entity::find_by_id(id)
        .one(db)
        .await
        .map_err(|e| HttpResponse::internal_server_error(e.to_string()))?
        .ok_or_else(|| HttpResponse::not_found("{name} not found"))?;

    Inertia::render(&req, "{plural}/Edit", {name}EditProps {{ {snake}, errors: None }})
}}

/// Update an existing {snake}
pub async fn update(req: Request, id: i64) -> Response {{
    let ctx = SavedInertiaContext::from(&req);
    let form: {name}Form = req.input().await.map_err(|e| {{
        HttpResponse::bad_request(format!("Invalid form data: {{}}", e))
    }})?;

    let db = req.db();
    let {snake} = {snake_name}::Entity::find_by_id(id)
        .one(db)
        .await
        .map_err(|e| HttpResponse::internal_server_error(e.to_string()))?
        .ok_or_else(|| HttpResponse::not_found("{name} not found"))?;

    // Validate using derive macro
    if let Err(errors) = form.validate() {{
        return Inertia::render_ctx(&ctx, "{plural}/Edit", {name}EditProps {{
            {snake},
            errors: Some(errors.into_messages()),
        }});
    }}

    let mut model: {snake_name}::ActiveModel = {snake}.into();
{update_fields}    model.updated_at = ActiveValue::Set(chrono::Utc::now());

    model.update(db).await
        .map_err(|e| HttpResponse::internal_server_error(e.to_string()))?;

    HttpResponse::redirect(&format!("/{plural}/{{}}", id))
}}

/// Delete a {snake}
pub async fn destroy(req: Request, id: i64) -> Response {{
    let db = req.db();
    {snake_name}::Entity::delete_by_id(id)
        .exec(db)
        .await
        .map_err(|e| HttpResponse::internal_server_error(e.to_string()))?;

    HttpResponse::redirect("/{plural}")
}}
"#,
        name = name,
        snake = snake_name,
        snake_name = snake_name,
        plural = plural_snake,
        plural_pascal = to_pascal_case(plural_snake),
        form_fields = form_fields,
        update_fields = update_fields,
        insert_fields = fields
            .iter()
            .map(|f| format!(
                "        {}: ActiveValue::Set(form.{}.clone()),\n",
                f.name, f.name
            ))
            .collect::<String>()
    );

    fs::write(&file_path, controller_content).expect("Failed to write controller file");

    // Update mod.rs
    update_controllers_mod(snake_name);

    println!(
        "   📦 Created controller: src/controllers/{}_controller.rs",
        snake_name
    );
}

fn update_controllers_mod(snake_name: &str) {
    let mod_path = Path::new("src/controllers/mod.rs");
    let module_name = format!("{}_controller", snake_name);

    if !mod_path.exists() {
        let content = format!("pub mod {};\n", module_name);
        fs::write(mod_path, content).expect("Failed to write mod.rs");
        return;
    }

    let content = fs::read_to_string(mod_path).expect("Failed to read mod.rs");
    let mod_declaration = format!("pub mod {};", module_name);

    if content.contains(&mod_declaration) {
        return;
    }

    let updated = format!("{}{}\n", content, mod_declaration);
    fs::write(mod_path, updated).expect("Failed to write mod.rs");
}

fn generate_inertia_pages(name: &str, snake_name: &str, plural_snake: &str, fields: &[Field]) {
    let pages_dir = Path::new("frontend/src/pages").join(plural_snake);

    if !pages_dir.exists() {
        fs::create_dir_all(&pages_dir).expect("Failed to create pages directory");
    }

    // Generate Index page
    generate_index_page(&pages_dir, name, snake_name, plural_snake, fields);

    // Generate Show page
    generate_show_page(&pages_dir, name, snake_name, plural_snake, fields);

    // Generate Create page
    generate_create_page(&pages_dir, name, snake_name, plural_snake, fields);

    // Generate Edit page
    generate_edit_page(&pages_dir, name, snake_name, plural_snake, fields);

    println!(
        "   📦 Created Inertia pages: frontend/src/pages/{}/",
        plural_snake
    );
}

fn generate_index_page(
    pages_dir: &Path,
    name: &str,
    snake_name: &str,
    plural_snake: &str,
    fields: &[Field],
) {
    let file_path = pages_dir.join("Index.tsx");

    // Build table headers
    let headers: String = fields
        .iter()
        .map(|f| format!("              <th>{}</th>\n", to_pascal_case(&f.name)))
        .collect();

    // Build table cells
    let cells: String = fields
        .iter()
        .map(|f| {
            format!(
                "                <td>{{{snake}.{}}}</td>\n",
                f.name,
                snake = snake_name
            )
        })
        .collect();

    let content = format!(
        r#"import {{ Link }} from '@inertiajs/react';

interface {name} {{
  id: number;
{ts_fields}  created_at: string;
  updated_at: string;
}}

interface Props {{
  {plural}: {name}[];
}}

export default function Index({{ {plural} }}: Props) {{
  return (
    <div className="container mx-auto px-4 py-8">
      <div className="flex justify-between items-center mb-6">
        <h1 className="text-2xl font-bold">{name_display}</h1>
        <Link
          href="/{plural}/create"
          className="bg-blue-500 text-white px-4 py-2 rounded hover:bg-blue-600"
        >
          Create {name}
        </Link>
      </div>

      <table className="min-w-full bg-white border border-gray-200">
        <thead>
          <tr className="bg-gray-100">
            <th className="px-4 py-2 text-left">ID</th>
{headers}            <th className="px-4 py-2 text-left">Actions</th>
          </tr>
        </thead>
        <tbody>
          {{{plural}.map(({snake}) => (
            <tr key={{{snake}.id}} className="border-t">
              <td className="px-4 py-2">{{{snake}.id}}</td>
{cells}              <td className="px-4 py-2">
                <Link
                  href={{`/{plural}/${{{snake}.id}}`}}
                  className="text-blue-500 hover:underline mr-2"
                >
                  View
                </Link>
                <Link
                  href={{`/{plural}/${{{snake}.id}}/edit`}}
                  className="text-green-500 hover:underline"
                >
                  Edit
                </Link>
              </td>
            </tr>
          ))}}
        </tbody>
      </table>
    </div>
  );
}}
"#,
        name = name,
        name_display = pluralize(name),
        snake = snake_name,
        plural = plural_snake,
        headers = headers,
        cells = cells,
        ts_fields = fields
            .iter()
            .map(|f| format!("  {}: {};\n", f.name, f.field_type.to_typescript_type()))
            .collect::<String>()
    );

    fs::write(file_path, content).expect("Failed to write Index.tsx");
}

fn generate_show_page(
    pages_dir: &Path,
    name: &str,
    snake_name: &str,
    plural_snake: &str,
    fields: &[Field],
) {
    let file_path = pages_dir.join("Show.tsx");

    // Build field displays
    let field_displays: String = fields
        .iter()
        .map(|f| {
            format!(
                r#"        <div className="mb-4">
          <label className="block text-gray-700 font-bold">{label}</label>
          <p>{{{snake}.{field}}}</p>
        </div>
"#,
                label = to_pascal_case(&f.name),
                snake = snake_name,
                field = f.name
            )
        })
        .collect();

    let content = format!(
        r#"import {{ Link, router }} from '@inertiajs/react';

interface {name} {{
  id: number;
{ts_fields}  created_at: string;
  updated_at: string;
}}

interface Props {{
  {snake}: {name};
}}

export default function Show({{ {snake} }}: Props) {{
  const handleDelete = () => {{
    if (confirm('Are you sure you want to delete this {snake}?')) {{
      router.delete(`/{plural}/${{{snake}.id}}`);
    }}
  }};

  return (
    <div className="container mx-auto px-4 py-8">
      <div className="max-w-2xl mx-auto">
        <div className="flex justify-between items-center mb-6">
          <h1 className="text-2xl font-bold">{name} Details</h1>
          <div>
            <Link
              href="/{plural}"
              className="text-gray-500 hover:underline mr-4"
            >
              Back to list
            </Link>
            <Link
              href={{`/{plural}/${{{snake}.id}}/edit`}}
              className="bg-blue-500 text-white px-4 py-2 rounded hover:bg-blue-600 mr-2"
            >
              Edit
            </Link>
            <button
              onClick={{handleDelete}}
              className="bg-red-500 text-white px-4 py-2 rounded hover:bg-red-600"
            >
              Delete
            </button>
          </div>
        </div>

        <div className="bg-white shadow rounded-lg p-6">
          <div className="mb-4">
            <label className="block text-gray-700 font-bold">ID</label>
            <p>{{{snake}.id}}</p>
          </div>
{field_displays}
          <div className="mb-4">
            <label className="block text-gray-700 font-bold">Created At</label>
            <p>{{new Date({snake}.created_at).toLocaleString()}}</p>
          </div>
          <div className="mb-4">
            <label className="block text-gray-700 font-bold">Updated At</label>
            <p>{{new Date({snake}.updated_at).toLocaleString()}}</p>
          </div>
        </div>
      </div>
    </div>
  );
}}
"#,
        name = name,
        snake = snake_name,
        plural = plural_snake,
        field_displays = field_displays,
        ts_fields = fields
            .iter()
            .map(|f| format!("  {}: {};\n", f.name, f.field_type.to_typescript_type()))
            .collect::<String>()
    );

    fs::write(file_path, content).expect("Failed to write Show.tsx");
}

fn generate_create_page(
    pages_dir: &Path,
    name: &str,
    _snake_name: &str,
    plural_snake: &str,
    fields: &[Field],
) {
    let file_path = pages_dir.join("Create.tsx");

    // Build form inputs
    let form_inputs: String = fields
        .iter()
        .map(|f| {
            let input_type = f.field_type.to_form_input_type();
            if input_type == "textarea" {
                format!(
                    r#"        <div className="mb-4">
          <label className="block text-gray-700 mb-2">{label}</label>
          <textarea
            value={{data.{field}}}
            onChange={{e => setData('{field}', e.target.value)}}
            className="w-full border rounded px-3 py-2"
            rows={{4}}
          />
          {{errors.{field} && <p className="text-red-500 text-sm mt-1">{{errors.{field}}}</p>}}
        </div>
"#,
                    label = to_pascal_case(&f.name),
                    field = f.name
                )
            } else if input_type == "checkbox" {
                format!(
                    r#"        <div className="mb-4">
          <label className="flex items-center">
            <input
              type="checkbox"
              checked={{data.{field}}}
              onChange={{e => setData('{field}', e.target.checked)}}
              className="mr-2"
            />
            <span className="text-gray-700">{label}</span>
          </label>
          {{errors.{field} && <p className="text-red-500 text-sm mt-1">{{errors.{field}}}</p>}}
        </div>
"#,
                    label = to_pascal_case(&f.name),
                    field = f.name
                )
            } else {
                format!(
                    r#"        <div className="mb-4">
          <label className="block text-gray-700 mb-2">{label}</label>
          <input
            type="{input_type}"
            value={{data.{field}}}
            onChange={{e => setData('{field}', e.target.value)}}
            className="w-full border rounded px-3 py-2"
          />
          {{errors.{field} && <p className="text-red-500 text-sm mt-1">{{errors.{field}}}</p>}}
        </div>
"#,
                    label = to_pascal_case(&f.name),
                    field = f.name,
                    input_type = input_type
                )
            }
        })
        .collect();

    // Build initial data
    let initial_data: String = fields
        .iter()
        .map(|f| {
            let default_value = match f.field_type {
                FieldType::String | FieldType::Text => "''",
                FieldType::Integer | FieldType::BigInteger | FieldType::Float => "0",
                FieldType::Boolean => "false",
                FieldType::DateTime | FieldType::Date => "''",
                FieldType::Uuid => "''",
            };
            format!("    {}: {},\n", f.name, default_value)
        })
        .collect();

    let content = format!(
        r#"import {{ Link, useForm }} from '@inertiajs/react';

interface Props {{
  errors?: Record<string, string[]>;
}}

export default function Create({{ errors: serverErrors }}: Props) {{
  const {{ data, setData, post, processing, errors }} = useForm({{
{initial_data}  }});

  const handleSubmit = (e: React.FormEvent) => {{
    e.preventDefault();
    post('/{plural}');
  }};

  return (
    <div className="container mx-auto px-4 py-8">
      <div className="max-w-2xl mx-auto">
        <div className="flex justify-between items-center mb-6">
          <h1 className="text-2xl font-bold">Create {name}</h1>
          <Link href="/{plural}" className="text-gray-500 hover:underline">
            Back to list
          </Link>
        </div>

        <form onSubmit={{handleSubmit}} className="bg-white shadow rounded-lg p-6">
{form_inputs}
          <div className="flex justify-end">
            <button
              type="submit"
              disabled={{processing}}
              className="bg-blue-500 text-white px-4 py-2 rounded hover:bg-blue-600 disabled:opacity-50"
            >
              {{processing ? 'Creating...' : 'Create {name}'}}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}}
"#,
        name = name,
        plural = plural_snake,
        form_inputs = form_inputs,
        initial_data = initial_data
    );

    fs::write(file_path, content).expect("Failed to write Create.tsx");
}

fn generate_edit_page(
    pages_dir: &Path,
    name: &str,
    snake_name: &str,
    plural_snake: &str,
    fields: &[Field],
) {
    let file_path = pages_dir.join("Edit.tsx");

    // Build form inputs
    let form_inputs: String = fields
        .iter()
        .map(|f| {
            let input_type = f.field_type.to_form_input_type();
            if input_type == "textarea" {
                format!(
                    r#"        <div className="mb-4">
          <label className="block text-gray-700 mb-2">{label}</label>
          <textarea
            value={{data.{field}}}
            onChange={{e => setData('{field}', e.target.value)}}
            className="w-full border rounded px-3 py-2"
            rows={{4}}
          />
          {{errors.{field} && <p className="text-red-500 text-sm mt-1">{{errors.{field}}}</p>}}
        </div>
"#,
                    label = to_pascal_case(&f.name),
                    field = f.name
                )
            } else if input_type == "checkbox" {
                format!(
                    r#"        <div className="mb-4">
          <label className="flex items-center">
            <input
              type="checkbox"
              checked={{data.{field}}}
              onChange={{e => setData('{field}', e.target.checked)}}
              className="mr-2"
            />
            <span className="text-gray-700">{label}</span>
          </label>
          {{errors.{field} && <p className="text-red-500 text-sm mt-1">{{errors.{field}}}</p>}}
        </div>
"#,
                    label = to_pascal_case(&f.name),
                    field = f.name
                )
            } else {
                format!(
                    r#"        <div className="mb-4">
          <label className="block text-gray-700 mb-2">{label}</label>
          <input
            type="{input_type}"
            value={{data.{field}}}
            onChange={{e => setData('{field}', e.target.value)}}
            className="w-full border rounded px-3 py-2"
          />
          {{errors.{field} && <p className="text-red-500 text-sm mt-1">{{errors.{field}}}</p>}}
        </div>
"#,
                    label = to_pascal_case(&f.name),
                    field = f.name,
                    input_type = input_type
                )
            }
        })
        .collect();

    // Build initial data from prop
    let initial_data: String = fields
        .iter()
        .map(|f| format!("    {}: {}.{},\n", f.name, snake_name, f.name))
        .collect();

    let content = format!(
        r#"import {{ Link, useForm }} from '@inertiajs/react';

interface {name} {{
  id: number;
{ts_fields}  created_at: string;
  updated_at: string;
}}

interface Props {{
  {snake}: {name};
  errors?: Record<string, string[]>;
}}

export default function Edit({{ {snake}, errors: serverErrors }}: Props) {{
  const {{ data, setData, put, processing, errors }} = useForm({{
{initial_data}  }});

  const handleSubmit = (e: React.FormEvent) => {{
    e.preventDefault();
    put(`/{plural}/${{{snake}.id}}`);
  }};

  return (
    <div className="container mx-auto px-4 py-8">
      <div className="max-w-2xl mx-auto">
        <div className="flex justify-between items-center mb-6">
          <h1 className="text-2xl font-bold">Edit {name}</h1>
          <Link href="/{plural}" className="text-gray-500 hover:underline">
            Back to list
          </Link>
        </div>

        <form onSubmit={{handleSubmit}} className="bg-white shadow rounded-lg p-6">
{form_inputs}
          <div className="flex justify-end">
            <button
              type="submit"
              disabled={{processing}}
              className="bg-blue-500 text-white px-4 py-2 rounded hover:bg-blue-600 disabled:opacity-50"
            >
              {{processing ? 'Saving...' : 'Save Changes'}}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}}
"#,
        name = name,
        snake = snake_name,
        plural = plural_snake,
        form_inputs = form_inputs,
        initial_data = initial_data,
        ts_fields = fields
            .iter()
            .map(|f| format!("  {}: {};\n", f.name, f.field_type.to_typescript_type()))
            .collect::<String>()
    );

    fs::write(file_path, content).expect("Failed to write Edit.tsx");
}

fn print_route_instructions(name: &str, snake_name: &str, plural_snake: &str) {
    println!("\n📝 Add these routes to src/routes.rs:\n");
    println!(
        r#"use crate::controllers::{snake}_controller;

// {name} routes
route("/{plural}", {snake}_controller::index);
route("/{plural}/create", {snake}_controller::create);
route_post("/{plural}", {snake}_controller::store);
route("/{plural}/{{id}}", {snake}_controller::show);
route("/{plural}/{{id}}/edit", {snake}_controller::edit);
route_put("/{plural}/{{id}}", {snake}_controller::update);
route_delete("/{plural}/{{id}}", {snake}_controller::destroy);"#,
        name = name,
        snake = snake_name,
        plural = plural_snake
    );
}
