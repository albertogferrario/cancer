//! Dynamic crate path resolution for proc macros
//!
//! Resolves the actual ferro crate name from user's Cargo.toml,
//! allowing `ferro = ...` instead of requiring `ferro_rs = ...`.

use proc_macro2::TokenStream;
use proc_macro_crate::{crate_name, FoundCrate};
use quote::quote;

/// Returns a TokenStream for the ferro crate path.
///
/// Attempts to find "ferro" first (the published name), then falls back
/// to "ferro_rs" for backwards compatibility, then defaults to "ferro_rs".
///
/// # Example
///
/// ```ignore
/// let ferro = ferro_crate();
/// quote! { #ferro::Response }
/// // Generates: ::ferro::Response (or ::my_ferro::Response if renamed)
/// ```
pub fn ferro_crate() -> TokenStream {
    // Try "ferro" first (the crates.io published name)
    if let Ok(found) = crate_name("ferro") {
        return match found {
            FoundCrate::Itself => quote!(crate),
            FoundCrate::Name(name) => {
                let ident = proc_macro2::Ident::new(&name, proc_macro2::Span::call_site());
                quote!(::#ident)
            }
        };
    }

    // Fall back to "ferro_rs" for backwards compatibility
    if let Ok(found) = crate_name("ferro_rs") {
        return match found {
            FoundCrate::Itself => quote!(crate),
            FoundCrate::Name(name) => {
                let ident = proc_macro2::Ident::new(&name, proc_macro2::Span::call_site());
                quote!(::#ident)
            }
        };
    }

    // Default fallback
    quote!(::ferro_rs)
}
