//! Password reset tokens migration

use ferro::migration::{Migration, Schema, Table};

pub struct CreatePasswordResetTokensTable;

impl Migration for CreatePasswordResetTokensTable {
    fn name(&self) -> &str {
        "create_password_reset_tokens_table"
    }

    fn up(&self, schema: &mut Schema) {
        schema.create("password_reset_tokens", |table: &mut Table| {
            table.string("email").primary_key();
            table.string("token");
            table.string("created_at");
            table.index(&["token"]);
        });
    }

    fn down(&self, schema: &mut Schema) {
        schema.drop("password_reset_tokens");
    }
}
