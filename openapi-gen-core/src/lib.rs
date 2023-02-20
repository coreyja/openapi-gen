#![deny(
    warnings,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs
)]
//! `openapi-gen-core` is a library for generating Rust code from OpenAPI specifications.
//!
//! Among the universe of `openapi-gen` crates, it is the one is not expected to be used directly.
//! It helps power `openapi-gen` and `openapi-gen-macro`

use std::fs;

use darling::FromMeta;
use openapiv3::*;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use serde_json::Value;
use syn::{parse::Parser, parse_quote, Fields, ItemEnum, ItemMod, ItemStruct};

pub use darling;
pub use syn;

mod response;
use response::*;

mod request;
use request::*;

mod op;

mod path;

mod schema;
use schema::*;

mod utils;

mod refs;
use refs::*;

trait IntoMods {
    fn as_mods(&self, refs: &ReferenceableAPI) -> Vec<syn::ItemMod>;
}

trait IntoMod {
    fn as_mod(&self, refs: &ReferenceableAPI) -> syn::ItemMod;
}

#[derive(Debug, FromMeta)]
/// Struct for parsing the `#[openapi]` attribute
/// This is parsed by the `darling` crate
pub struct MacroArgs {
    /// Path, relative to the crate root, to the OpenAPI spec
    pub path: String,
    /// Optional name for the generated module. If not provided, the name will be inferred from the OpenAPI spec
    pub name: Option<String>,
}

/// Inner Function for the `#[api]` proc-macro
/// This is the main entry point for the `openapi-gen` crate
///
/// The arguments just contain the path to an OpenAPI spec, that must currently be a JSON file
///
/// The input token stream is a module that the generated code will be placed in.
/// Since this is expected to be run as a proc-macro, the input is expected to parse to a `syn::ItemMod`
/// and it returns a `syn::ItemMod` [converted to a TokenStream] for the same module with the generated code added to it.
pub fn api(args: MacroArgs) -> proc_macro2::TokenStream {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let path = format!("{}/{}", manifest_dir, args.path);
    let contents = fs::read_to_string(path);
    if contents.is_err() {
        return quote! {
            compile_error!("File not found")
        };
    }
    let contents = contents.unwrap();

    let openapi: OpenAPI = serde_json::from_str(&contents).expect("Could not deserialize input");

    let outer_mod_name = format_ident!("{}", args.name.unwrap_or_else(|| "openapi".to_string()));
    let mut item_mod: syn::ItemMod = parse_quote! {
        mod #outer_mod_name {}
    };

    let refable = ReferenceableAPI(openapi);

    let mods = refable.0.paths.as_mods(&refable);

    for m in mods.into_iter() {
        item_mod.content.as_mut().unwrap().1.push(m.into());
    }

    quote! {
        #item_mod
    }
}
