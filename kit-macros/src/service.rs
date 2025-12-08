//! Service trait macro for the Kit framework
//!
//! Provides the `#[service]` attribute macro that:
//! 1. Adds `Send + Sync + 'static` bounds to trait definitions
//! 2. Optionally auto-registers a concrete implementation with the container

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemTrait, Path};

/// Implements the `#[service]` attribute macro
///
/// This macro transforms a trait definition to add `Send + Sync + 'static` bounds,
/// making it suitable for use with the App container.
///
/// # Without concrete type (existing behavior)
///
/// ```rust,ignore
/// #[service]
/// pub trait HttpClient {
///     async fn get(&self, url: &str) -> Result<String, Error>;
/// }
///
/// // Expands to:
/// pub trait HttpClient: Send + Sync + 'static {
///     async fn get(&self, url: &str) -> Result<String, Error>;
/// }
/// ```
///
/// # With concrete type (auto-registration)
///
/// ```rust,ignore
/// #[service(RedisCache)]
/// pub trait CacheStore {
///     fn get(&self, key: &str) -> Option<String>;
/// }
///
/// // This also registers the binding at startup:
/// // App::bind::<dyn CacheStore>(Arc::new(RedisCache::default()))
/// ```
pub fn service_impl(attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut item_trait = parse_macro_input!(input as ItemTrait);

    // Add Send + Sync + 'static to the trait's supertraits
    let send_bound: syn::TypeParamBound = syn::parse_quote!(Send);
    let sync_bound: syn::TypeParamBound = syn::parse_quote!(Sync);
    let static_bound: syn::TypeParamBound = syn::parse_quote!('static);

    // Check if bounds already exist to avoid duplicates
    let has_send = item_trait.supertraits.iter().any(|bound| {
        if let syn::TypeParamBound::Trait(trait_bound) = bound {
            trait_bound
                .path
                .segments
                .last()
                .map(|s| s.ident == "Send")
                .unwrap_or(false)
        } else {
            false
        }
    });

    let has_sync = item_trait.supertraits.iter().any(|bound| {
        if let syn::TypeParamBound::Trait(trait_bound) = bound {
            trait_bound
                .path
                .segments
                .last()
                .map(|s| s.ident == "Sync")
                .unwrap_or(false)
        } else {
            false
        }
    });

    let has_static = item_trait.supertraits.iter().any(|bound| {
        matches!(bound, syn::TypeParamBound::Lifetime(lt) if lt.ident == "static")
    });

    // Add missing bounds
    if !has_send {
        item_trait.supertraits.push(send_bound);
    }
    if !has_sync {
        item_trait.supertraits.push(sync_bound);
    }
    if !has_static {
        item_trait.supertraits.push(static_bound);
    }

    let trait_name = &item_trait.ident;
    let trait_name_str = trait_name.to_string();

    // Check if a concrete type was specified in the attribute
    let expanded = if attr.is_empty() {
        // No concrete type - just add bounds
        quote! {
            #item_trait
        }
    } else {
        // Parse the concrete type from attribute
        let concrete_type = parse_macro_input!(attr as Path);

        // Generate inventory submission for auto-registration
        quote! {
            #item_trait

            // Auto-register this service binding at startup
            ::kit::inventory::submit! {
                ::kit::container::provider::ServiceBindingEntry {
                    register: || {
                        ::kit::App::bind::<dyn #trait_name>(
                            ::std::sync::Arc::new(<#concrete_type as ::std::default::Default>::default())
                        );
                    },
                    name: #trait_name_str,
                }
            }
        }
    };

    TokenStream::from(expanded)
}
