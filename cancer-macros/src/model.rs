//! Derive macro for reducing SeaORM model boilerplate
//!
//! Generates builder, setters, and trait implementations for Cancer models.

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput, Fields, Type};

/// Generate model boilerplate from a SeaORM Model struct
pub fn cancer_model_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    // Extract fields from struct
    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => {
                return syn::Error::new_spanned(&input, "CancerModel only supports named structs")
                    .to_compile_error()
                    .into();
            }
        },
        _ => {
            return syn::Error::new_spanned(&input, "CancerModel only supports structs")
                .to_compile_error()
                .into();
        }
    };

    // Generate builder name
    let builder_name = format_ident!("{}Builder", name);

    // Generate field data for various uses
    let mut builder_fields = Vec::new();
    let mut builder_default_fields = Vec::new();
    let mut builder_setters = Vec::new();
    let mut model_setters = Vec::new();
    let mut build_fields = Vec::new();

    for field in fields {
        let field_name = field.ident.as_ref().unwrap();
        let field_ty = &field.ty;

        // Skip id field for setters
        let is_id = field_name == "id";
        let is_timestamps = field_name == "created_at" || field_name == "updated_at";

        // Check if it's an Option type
        let is_option = is_option_type(field_ty);

        // Builder field - wrap in Option for builder pattern
        if !is_id {
            let builder_field_ty = if is_option {
                quote! { Option<#field_ty> }
            } else {
                quote! { Option<#field_ty> }
            };
            builder_fields.push(quote! {
                #field_name: #builder_field_ty
            });

            builder_default_fields.push(quote! {
                #field_name: None
            });
        }

        // Builder setter method
        if !is_id && !is_timestamps {
            let setter_name = format_ident!("set_{}", field_name);

            if is_option {
                // For Option<T> fields, accept T and wrap in Some
                let inner_ty = get_option_inner_type(field_ty);
                if is_string_like(&inner_ty) {
                    builder_setters.push(quote! {
                        /// Set the #field_name field
                        pub fn #setter_name(mut self, value: impl Into<String>) -> Self {
                            self.#field_name = Some(Some(value.into()));
                            self
                        }
                    });
                } else {
                    builder_setters.push(quote! {
                        /// Set the #field_name field
                        pub fn #setter_name(mut self, value: #inner_ty) -> Self {
                            self.#field_name = Some(Some(value));
                            self
                        }
                    });
                }
            } else if is_string_like(field_ty) {
                builder_setters.push(quote! {
                    /// Set the #field_name field
                    pub fn #setter_name(mut self, value: impl Into<String>) -> Self {
                        self.#field_name = Some(value.into());
                        self
                    }
                });
            } else {
                builder_setters.push(quote! {
                    /// Set the #field_name field
                    pub fn #setter_name(mut self, value: #field_ty) -> Self {
                        self.#field_name = Some(value);
                        self
                    }
                });
            }
        }

        // Model setter method (for updating existing records)
        if !is_id && !is_timestamps {
            let setter_name = format_ident!("set_{}", field_name);

            if is_option {
                let inner_ty = get_option_inner_type(field_ty);
                if is_string_like(&inner_ty) {
                    model_setters.push(quote! {
                        /// Set the #field_name field
                        pub fn #setter_name(mut self, value: Option<impl Into<String>>) -> Self {
                            self.#field_name = value.map(|v| v.into());
                            self
                        }
                    });
                } else {
                    model_setters.push(quote! {
                        /// Set the #field_name field
                        pub fn #setter_name(mut self, value: #field_ty) -> Self {
                            self.#field_name = value;
                            self
                        }
                    });
                }
            } else if is_string_like(field_ty) {
                model_setters.push(quote! {
                    /// Set the #field_name field
                    pub fn #setter_name(mut self, value: impl Into<String>) -> Self {
                        self.#field_name = value.into();
                        self
                    }
                });
            } else {
                model_setters.push(quote! {
                    /// Set the #field_name field
                    pub fn #setter_name(mut self, value: #field_ty) -> Self {
                        self.#field_name = value;
                        self
                    }
                });
            }
        }

        // Build field (for converting builder to ActiveModel)
        if is_id {
            build_fields.push(quote! {
                #field_name: sea_orm::ActiveValue::NotSet
            });
        } else {
            build_fields.push(quote! {
                #field_name: self.#field_name.map(sea_orm::Set).unwrap_or(sea_orm::ActiveValue::NotSet)
            });
        }
    }

    // Generate to_active_model fields
    let to_active_fields: Vec<_> = fields
        .iter()
        .map(|f| {
            let field_name = f.ident.as_ref().unwrap();
            quote! {
                #field_name: sea_orm::Set(self.#field_name.clone())
            }
        })
        .collect();

    let expanded = quote! {
        impl #name {
            /// Start a new query builder
            pub fn query() -> cancer::database::QueryBuilder<Entity> {
                cancer::database::QueryBuilder::new()
            }

            /// Create a new record builder
            pub fn create() -> #builder_name {
                #builder_name::default()
            }

            #(#model_setters)*

            /// Save changes to the database
            pub async fn update(self) -> Result<Self, cancer::FrameworkError> {
                let active = self.to_active_model();
                Entity::update_one(active).await
            }

            /// Delete this record from the database
            pub async fn delete(self) -> Result<u64, cancer::FrameworkError> {
                Entity::delete_by_pk(self.id).await
            }

            fn to_active_model(&self) -> ActiveModel {
                ActiveModel {
                    #(#to_active_fields),*
                }
            }
        }

        /// Builder for creating new #name records
        #[derive(Default)]
        pub struct #builder_name {
            #(#builder_fields),*
        }

        impl #builder_name {
            #(#builder_setters)*

            /// Insert the record into the database
            pub async fn insert(self) -> Result<#name, cancer::FrameworkError> {
                let active = self.build();
                Entity::insert_one(active).await
            }

            fn build(self) -> ActiveModel {
                ActiveModel {
                    #(#build_fields),*
                }
            }
        }
    };

    TokenStream::from(expanded)
}

/// Check if a type is Option<T>
fn is_option_type(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident == "Option";
        }
    }
    false
}

/// Get the inner type of Option<T>
fn get_option_inner_type(ty: &Type) -> Type {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            if segment.ident == "Option" {
                if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                    if let Some(syn::GenericArgument::Type(inner)) = args.args.first() {
                        return inner.clone();
                    }
                }
            }
        }
    }
    ty.clone()
}

/// Check if type is String or string-like
fn is_string_like(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident == "String";
        }
    }
    false
}
