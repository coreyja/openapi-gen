use std::{collections::HashMap, fs};

use darling::FromMeta;
use openapiv3::*;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned};
use serde_json::Value;
use syn::{parse::Parser, parse_quote, Fields, ItemEnum, ItemMod, ItemStruct};

pub use darling;
pub use syn;

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

trait IntoType {
    fn as_type(&self, new_structs: &mut Vec<ItemStruct>, name: &str, count: usize) -> TokenStream;
}

impl IntoType for Schema {
    fn as_type(&self, new_structs: &mut Vec<ItemStruct>, name: &str, count: usize) -> TokenStream {
        match &self.schema_kind {
            SchemaKind::Type(t) => into_type(t, new_structs, name, count),
            SchemaKind::OneOf { .. } => todo!("Generate an enum from the possible schemas"),
            SchemaKind::AllOf { .. } => {
                todo!("IDK... Try to make a struct thats the union of all the schemas?")
            }
            SchemaKind::AnyOf { .. } => todo!("Is this the same as oneOf?"),
            SchemaKind::Not { .. } => {
                todo!("Dont think we can really support a NOT schema.... Just skip it?")
            }
            SchemaKind::Any(_) => todo!("Untyped schema... Just skip it?"),
        }
    }
}

fn into_type(t: &Type, new_structs: &mut Vec<ItemStruct>, name: &str, count: usize) -> TokenStream {
    match t {
        Type::String(_) => quote::quote!(String),
        Type::Number(_) => quote::quote!(f64),
        Type::Integer(_) => quote::quote!(i64),
        Type::Object(o) => {
            let ident = if count == 0 {
                name.to_string()
            } else {
                format!("{name}_{count}")
            };
            let ident = format_ident!("{ident}");
            let mut s: ItemStruct = parse_quote! {
              pub struct #ident {}
            };

            if let Fields::Named(ref mut fields) = s.fields {
                if o.additional_properties.is_some() {
                    todo!("We dont support additional properties yet")
                }

                for (key, value) in o.properties.iter() {
                    let s = value.as_item().unwrap();

                    let ty = s.as_type(new_structs, name, count + 1);

                    let key = format_ident!("{key}");

                    fields.named.push(
                        syn::Field::parse_named
                            .parse2(quote::quote! { pub #key: #ty })
                            .unwrap(),
                    );
                }
            } else {
                panic!("This should always be named cause we just made the struct")
            };

            new_structs.push(s);

            quote::quote!(#ident)
        }
        Type::Array(a) => {
            let sample = a.items.iter().next();
            let ty = match sample {
                Some(s) => {
                    let s = s.as_item().unwrap();

                    s.as_type(new_structs, name, count + 1)
                }
                None => todo!("What do we do with empty arrays?"),
            };

            quote::quote!(Vec<#ty>)
        }
        Type::Boolean {} => quote::quote!(bool),
    }
}

impl IntoOperationMod for Operation {
    fn into_op_mod(self, verb: &str) -> syn::ItemMod {
        let mut verb = verb.to_string();
        make_ascii_titlecase(&mut verb);

        let ident = format_ident!("{verb}");

        let mut operation_mod: ItemMod = parse_quote! {
          pub mod #ident {}
        };
        let content = &mut operation_mod.content.as_mut().unwrap().1;

        let request_struct: ItemStruct = parse_quote! {
          pub struct Request {}
        };

        let mut response_enum: ItemEnum = parse_quote! {
          #[doc="Test this Response"]
          pub enum Response { }
        };

        let mut structs: Vec<ItemStruct> = vec![];
        for (status_code, resp) in self.responses.responses {
            let resp = resp.as_item().unwrap();
            let variant_ident = format_ident!("_{status_code}");

            let x = resp.content.get("application/json").unwrap().clone();
            let mut iter = x.examples.into_iter();
            let item = iter.next().unwrap();
            let example_value = item.1.into_item();
            let example_value = example_value.unwrap().value.unwrap();

            let struct_ident = format!("Response{status_code}");
            let ty = type_for(&example_value, &mut structs, &struct_ident, 0);

            response_enum.variants.push(parse_quote! {
              #variant_ident(#ty)
            });
        }

        let mut param_struct: ItemStruct = parse_quote! {
          pub struct QueryParams {}
        };
        let mut headers_struct: ItemStruct = parse_quote! {
          pub struct Headers {}
        };

        let Fields::Named(ref mut struct_fields) = param_struct.fields else { panic!("This should always be named cause we just made the struct") };
        let Fields::Named(ref mut header_fields) = headers_struct.fields else { panic!("This should always be named cause we just made the struct") };

        for param in self.parameters {
            let param = param.into_item().unwrap();
            match param {
                Parameter::Query { parameter_data, .. } => {
                    add_field_for_param(parameter_data, &mut structs, struct_fields, "InnerParam");
                }
                Parameter::Header { parameter_data, .. } => {
                    add_field_for_param(parameter_data, &mut structs, header_fields, "InnerHeader")
                }
                Parameter::Path { .. } => todo!(),
                Parameter::Cookie { .. } => todo!(),
            };
        }

        content.push(param_struct.into());
        content.push(headers_struct.into());

        content.push(request_struct.into());
        content.push(response_enum.into());

        for i in structs {
            content.push(i.into())
        }

        operation_mod
    }
}

fn add_field_for_param(
    parameter_data: ParameterData,
    structs: &mut Vec<ItemStruct>,
    struct_fields: &mut syn::FieldsNamed,
    default_struct_name: &str,
) {
    let name = parameter_data.name;
    let ident = format_ident!("{name}");

    if let ParameterSchemaOrContent::Schema(s) = parameter_data.format {
        let s = s.into_item().unwrap();

        let ty = s.as_type(structs, default_struct_name, 0);

        struct_fields.named.push(
            syn::Field::parse_named
                .parse2(quote::quote! { pub #ident: #ty })
                .unwrap(),
        );
    } else {
        todo!("Need to handle cases where we have the nested content instead of a schema")
    }
}

fn type_for_value(value: &Value) -> Type {
    match value {
        Value::Null => todo!("What do we do with nulls?"),
        Value::Bool(_) => Type::Boolean {},
        Value::Number(n) => {
            if n.is_f64() {
                Type::Number(Default::default())
            } else {
                Type::Integer(Default::default())
            }
        }
        Value::String(_) => Type::String(Default::default()),
        Value::Array(a) => {
            let sample = a.iter().next();
            let ty = match sample {
                Some(v) => type_for_value(v),
                None => todo!("What do we do with empty arrays?"),
            };

            let s = Schema {
                schema_data: Default::default(),
                schema_kind: SchemaKind::Type(ty),
            };
            let s = Box::new(s);
            let s = ReferenceOr::Item(s);

            Type::Array(ArrayType {
                items: s.into(),
                min_items: None,
                max_items: None,
                unique_items: false,
            })
        }
        Value::Object(o) => {
            let mut properties = HashMap::<String, _>::new();
            for (key, value) in o.iter() {
                let ty = type_for_value(value);
                let s = Schema {
                    schema_data: Default::default(),
                    schema_kind: SchemaKind::Type(ty),
                };
                let s = Box::new(s);
                let s = ReferenceOr::Item(s);

                properties.insert(key.to_string(), s);
            }

            Type::Object(ObjectType {
                properties: properties.into_iter().collect(),
                required: vec![],
                additional_properties: None,
                min_properties: None,
                max_properties: None,
            })
        }
    }
}

fn type_for(
    value: &Value,
    new_structs: &mut Vec<ItemStruct>,
    name: &str,
    count: usize,
) -> TokenStream {
    let openapi_type = type_for_value(value);

    into_type(&openapi_type, new_structs, name, count)
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

    let mods = openapi.paths.to_mods();

    for m in mods.into_iter() {
        item_mod
            .content
            .as_mut()
            .unwrap()
            .1
            .push(parse_quote! { #m });
    }

    quote! {
        #item_mod
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_path_mod_names() {
        let spec_string = include_str!("../tests/simple_site.json");
        let spec: OpenAPI = serde_json::from_str(spec_string).unwrap();

        let paths = spec.paths;
        let mods = paths.to_mods();

        let names: Vec<_> = mods.iter().map(|m| m.ident.to_string()).collect();
        assert_eq!(names, vec!["Test_More", "Root"]);
    }
}
