use std::fs;

use darling::FromMeta;
use openapiv3::OpenAPI;
use quote::{format_ident, quote, quote_spanned};
use syn::{parse_macro_input, parse_quote};

#[derive(Debug, FromMeta)]
struct MacroArgs {
    path: String,
}

fn make_ascii_titlecase(s: &mut str) {
    if let Some(r) = s.get_mut(0..1) {
        r.make_ascii_uppercase();
    }
}

#[proc_macro_attribute]
pub fn api(
    attr_stream: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let attrs = parse_macro_input!(attr_stream as syn::AttributeArgs);
    let args = match MacroArgs::from_list(&attrs) {
        Ok(v) => v,
        Err(e) => {
            return proc_macro::TokenStream::from(e.write_errors());
        }
    };

    let mut item_mod = parse_macro_input!(input as syn::ItemMod);

    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let path = format!("{}/{}", manifest_dir, args.path);
    let contents = fs::read_to_string(path);
    if contents.is_err() {
        return quote_spanned! {
          item_mod.ident.span() =>
            compile_error!("File not found");
        }
        .into();
    }
    let contents = contents.unwrap();

    let openapi: OpenAPI = serde_json::from_str(&contents).expect("Could not deserialize input");

    if item_mod.content.is_none() {
        return quote_spanned! {
          item_mod.ident.span() =>
            compile_error!("Non-inline modules are not supported")
        }
        .into();
    }

    item_mod.content.as_mut().unwrap().1.push(parse_quote! {
          pub struct New {}
    });

    for (path, item) in openapi.paths.into_iter() {
        let item = item.into_item().unwrap();

        if let Some(op) = item.get {
            let mut id = op.operation_id.unwrap();
            make_ascii_titlecase(&mut id);

            let id = format_ident!("{}", id);

            item_mod.content.as_mut().unwrap().1.push(parse_quote! {
                pub struct #id {}
            });
        }
    }

    quote! {
        #item_mod
    }
    .into()
}
