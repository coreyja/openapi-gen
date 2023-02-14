use std::{collections::HashMap, fs};

use darling::FromMeta;
use openapiv3::*;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned};
use serde_json::Value;
use syn::{parse::Parser, parse_quote, Fields, ItemEnum, ItemMod, ItemStruct};

pub use darling;
pub use syn;

mod response;
use response::*;

mod request;
use request::*;

mod op;
use op::*;

mod path;
use path::*;

mod schema;
use schema::*;

mod utils;
use utils::*;

trait IntoMods {
    fn as_mods(&self) -> Vec<syn::ItemMod>;
}

trait IntoMod {
    fn as_mod(&self) -> syn::ItemMod;
}

#[derive(Debug, FromMeta)]
pub struct MacroArgs {
    pub path: String,
}

pub fn api(args: MacroArgs, input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let mut item_mod = match syn::parse2::<syn::ItemMod>(input) {
        Ok(syntax_tree) => syntax_tree,
        Err(err) => return err.to_compile_error(),
    };

    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let path = format!("{}/{}", manifest_dir, args.path);
    let contents = fs::read_to_string(path);
    if contents.is_err() {
        return quote_spanned! {
          item_mod.ident.span() =>
            compile_error!("File not found");
        };
    }
    let contents = contents.unwrap();

    let openapi: OpenAPI = serde_json::from_str(&contents).expect("Could not deserialize input");

    if item_mod.content.is_none() {
        return quote_spanned! {
          item_mod.ident.span() =>
            compile_error!("Non-inline modules are not supported")
        };
    }

    let mods = openapi.paths.as_mods();

    for m in mods.into_iter() {
        item_mod.content.as_mut().unwrap().1.push(m.into());
    }

    quote! {
        #item_mod
    }
}
