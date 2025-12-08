//! Service trait macro for the Kit framework
//!
//! Provides the `#[service]` attribute macro that automatically adds
//! `Send + Sync + 'static` bounds to trait definitions.

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemTrait};

/// Implements the `#[service]` attribute macro
///
/// This macro transforms a trait definition to add `Send + Sync + 'static` bounds,
/// making it suitable for use with the App container.
///
/// # Example
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
pub fn service_impl(input: TokenStream) -> TokenStream {
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

    // If there were no supertraits before, we need to add the colon
    // syn handles this automatically

    let expanded = quote! {
        #item_trait
    };

    TokenStream::from(expanded)
}
