use openapiv3::*;
use proc_macro2::{Ident, TokenStream};
use quote::format_ident;
use serde_json::Value;
use syn::{
    parse::Parser, parse_quote, token::Paren, Attribute, Fields, FieldsNamed, FieldsUnnamed,
    Generics, ItemEnum, ItemMod, ItemStruct, VisPublic,
};

fn make_ascii_titlecase(s: &mut str) {
    if let Some(r) = s.get_mut(0..1) {
        r.make_ascii_uppercase();
    }
}

pub trait IntoMods {
    fn to_mods(self) -> Vec<syn::ItemMod>;
}

impl IntoMods for Paths {
    fn to_mods(self) -> Vec<syn::ItemMod> {
        self.into_iter().map(IntoPathMod::to_path_mod).collect()
    }
}

pub trait IntoPathMod {
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

        let item = item.into_item().unwrap();

        // let mut id = item.get.unwrap().operation_id.unwrap();
        // make_ascii_titlecase(&mut id);

        // let id = format_ident!("{}", id);

        let mut path_mod: syn::ItemMod = parse_quote! {
            pub mod #path_ident {}
        };
        let content = &mut path_mod.content.as_mut().unwrap().1;

        if let Some(op) = item.get {
            content.push(op.into_op_mod("get").into());
        }
        if let Some(op) = item.post {
            content.push(op.into_op_mod("post").into());
        }
        // TODO: Need to do the rest of the operations
        // annoying there isn't any easy loop that I found

        path_mod
    }
}

trait IntoOperationMod {
    fn into_op_mod(self, verb: &str) -> syn::ItemMod;
}

impl IntoOperationMod for Operation {
    fn into_op_mod(self, verb: &str) -> syn::ItemMod {
        let mut verb = verb.to_string();
        make_ascii_titlecase(&mut verb);

        let ident = format_ident!("{verb}");

        let request_struct: ItemStruct = parse_quote! {
          pub struct Request {}
        };

        let mut response_enum: ItemEnum = parse_quote! {
          #[doc="Test this Response"]
          pub enum Response { }
        };
        let mut response_structs: Vec<ItemStruct> = vec![];
        for (status_code, resp) in self.responses.responses {
            let resp = resp.as_item().unwrap();
            let variant_ident = format_ident!("_{status_code}");

            let x = resp.content.get("application/json").unwrap().clone();
            let mut iter = x.examples.into_iter();
            let item = iter.next().unwrap();
            let example_value = item.1.into_item();
            let example_value = example_value.unwrap().value.unwrap();

            let struct_ident = format!("Response{status_code}");
            let ty = type_for(&example_value, &mut response_structs, &struct_ident, 0);

            response_enum.variants.push(parse_quote! {
              #variant_ident(#ty)
            });
        }

        let mut x: ItemMod = parse_quote! {
          pub mod #ident {
            #request_struct

            #response_enum
          }
        };
        let content = &mut x.content.as_mut().unwrap().1;

        for i in response_structs {
            content.push(i.into())
        }

        x
    }
}

fn type_for(value: &Value, structs: &mut Vec<ItemStruct>, name: &str, count: usize) -> TokenStream {
    match value {
        Value::Null => quote::quote!(()),
        Value::Bool(_) => quote::quote!(bool),
        // TODO: i64 isn't quite right here, we probably want to mess with this
        Value::Number(_) => quote::quote!(i64),
        Value::String(_) => quote::quote!(String),
        Value::Array(a) => {
            let sample = a.iter().next();
            match sample {
                Some(v) => type_for(v, structs, name, count),
                None => todo!("What do we do with empty arrays?"),
            }
        }
        Value::Object(o) => {
            let ident = if count == 0 {
                name.to_string()
            } else {
                format!("{name}_{count}")
            };
            let ident = format_ident!("{ident}");
            let mut s: ItemStruct = parse_quote! {
              pub struct #ident {}
            };

            if let Fields::Named(ref mut x) = s.fields {
                for (key, value) in o.into_iter() {
                    let ty = type_for(value, structs, name, count + 1);

                    let key = format_ident!("{key}");

                    x.named.push(
                        syn::Field::parse_named
                            .parse2(quote::quote! { pub #key: #ty })
                            .unwrap(),
                    );
                }
            } else {
                panic!("This should always be named cause we just made the struct")
            };

            structs.push(s);

            quote::quote!(#ident)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_path_mod_names() {
        let spec_string = include_str!("../../example_specs/some_site.json");
        let spec: OpenAPI = serde_json::from_str(spec_string).unwrap();

        let paths = spec.paths;
        let mods = paths.to_mods();

        let names: Vec<_> = mods.iter().map(|m| m.ident.to_string()).collect();
        assert_eq!(names, vec!["Test_Nested", "Root"]);
    }
}
