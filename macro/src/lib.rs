use std::fs;

use darling::FromMeta;
use openapiv3::{OpenAPI, PathItem, Paths, ReferenceOr};
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

    let mods = openapi.paths.to_mods();

    for m in mods.into_iter() {
        item_mod
            .content
            .as_mut()
            .unwrap()
            .1
            .push(parse_quote! { #m });
    }

    // item_mod.content.as_mut().unwrap().1.push(parse_quote! {
    //       pub struct New {}
    // });

    // for (path, item) in openapi.paths.into_iter() {
    //     let item = item.into_item().unwrap();

    //     if let Some(op) = item.get {
    //         // let mut id = op.operation_id.unwrap();
    //         // make_ascii_titlecase(&mut id);

    //         // let id = format_ident!("{}", id);

    //         // item_mod.content.as_mut().unwrap().1.push(parse_quote! {
    //         //     pub struct #id {}
    //         // });

    //         for (code, response) in op.responses.responses.into_iter() {
    //             let response = response.into_item().unwrap();
    //             // let mut id = response.description.clone();

    //             for (mime, media) in response.content.into_iter() {
    //                 for (mut example_name, example) in media.examples.into_iter() {
    //                     let example = example.into_item().unwrap();

    //                     example_name = example_name.replace(' ', "");
    //                     make_ascii_titlecase(&mut example_name);
    //                     let id = format_ident!("{}", example_name);

    //                     let example_json = example.value.unwrap();

    //                     // item_mod.content.as_mut().unwrap().1.push(parse_quote! {
    //                     //     pub struct #id {}
    //                     // });
    //                 }
    //             }
    //         }
    //     }
    // }

    quote! {
        #item_mod
    }
    .into()
}

trait IntoMods {
    fn to_mods(self) -> Vec<syn::ItemMod>;
}

impl IntoMods for Paths {
    fn to_mods(self) -> Vec<syn::ItemMod> {
        self.into_iter().map(IntoPathMod::to_path_mod).collect()
    }
}

trait IntoPathMod {
    fn to_path_mod(self) -> syn::ItemMod;
}

impl IntoPathMod for (String, ReferenceOr<PathItem>) {
    fn to_path_mod(self) -> syn::ItemMod {
        let (path, item) = self;
        let mut path_parts: Vec<_> = path
            .split('/')
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();
        for part in path_parts.iter_mut() {
            make_ascii_titlecase(part);
        }
        let path_ident = path_parts.join("_");
        let path_ident = if path_ident.is_empty() {
            "Root".to_string()
        } else {
            path_ident
        };
        let path_ident = format_ident!("{}", path_ident);

        // let item = item.into_item().unwrap();

        // let mut id = item.get.unwrap().operation_id.unwrap();
        // make_ascii_titlecase(&mut id);

        // let id = format_ident!("{}", id);

        parse_quote! {
            pub mod #path_ident {}
        }
    }
}
