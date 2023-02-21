use indexmap::IndexMap;
use inflector::Inflector;
use typify::{TypeSpace, TypeSpaceSettings};

use super::*;

impl IntoMod for Responses {
    fn as_mod(&self, refs: &ReferenceableAPI) -> syn::ItemMod {
        let mut settings = TypeSpaceSettings::default();
        settings.with_type_mod("self");
        settings.with_derive("PartialEq".to_owned());
        let mut types = TypeSpace::new(&settings);

        refs.add_reference_schemas(&mut types).unwrap();

        let mut response_enum: ItemEnum = parse_quote! {
          #[doc="Test this Response"]
          #[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
          pub enum Body { }
        };
        let mut headers_enum: ItemEnum = parse_quote! {
          #[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
          pub enum Headers { }
        };

        for (status_code, resp) in &self.responses {
            let resp = refs.resolve(resp).unwrap();

            let variant_ident = format_ident!("_{status_code}");
            let content = &resp.content;

            let struct_ident = format!("Body{status_code}");
            let ty = content_to_tokens(refs, &mut types, content, &struct_ident);

            let desc = &resp.description;
            let variant: syn::Variant = parse_quote! {
                #[doc = #desc]
                #variant_ident(#ty)
            };
            response_enum.variants.push(variant);

            let headers = &resp.headers;
            if !headers.is_empty() {
                let header_name = format!("Headers{status_code}");
                let schema = (refs, headers.clone()).to_schema();
                let type_id = types
                    .add_type_with_name(&schema, Some(header_name))
                    .unwrap();

                let t = types.get_type(&type_id).unwrap();
                let t_ident = t.ident();
                headers_enum.variants.push(parse_quote! {
                    #variant_ident(#t_ident)
                });
            }
        }

        let types_content = types.to_stream();
        let mut response_mod: syn::ItemMod = parse_quote! {
            pub mod response {
              use serde::{Serialize, Deserialize};

              #types_content
            }
        };
        let contents = &mut response_mod.content.as_mut().unwrap().1;

        contents.push(response_enum.into());
        if !headers_enum.variants.is_empty() {
            contents.push(headers_enum.into());
        }

        response_mod
    }
}

pub(crate) fn content_to_tokens(
    refs: &ReferenceableAPI,
    types: &mut TypeSpace,
    content: &IndexMap<String, MediaType>,
    struct_ident: &str,
) -> TokenStream {
    if content.is_empty() {
        return quote! { () };
    }

    let json_content = content.get("application/json").unwrap().clone();

    if let Some(schema) = json_content.schema {
        schema.as_type(types, struct_ident)
    } else {
        let mut iter = json_content.examples.into_iter();
        let (_name, example_value) = iter.next().unwrap();
        let example_value = refs.resolve(&example_value).unwrap();
        let example_value = example_value.value.unwrap();

        type_for(types, &example_value, struct_ident)
    }
}
