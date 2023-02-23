use indexmap::IndexMap;
use typify::{TypeSpace, TypeSpaceSettings};

use super::*;

struct ResponseStructNames {
    variant_ident: syn::Ident,
    struct_name: String,
    header_name: String,
}

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

            let names = ResponseStructNames {
                variant_ident: format_ident!("_{status_code}"),
                struct_name: format!("Body{status_code}"),
                header_name: format!("Headers{status_code}"),
            };

            process_response(
                resp,
                refs,
                &mut types,
                &mut response_enum,
                &mut headers_enum,
                names,
            );
        }

        if let Some(resp) = &self.default {
            let resp = refs.resolve(resp).unwrap();

            let names = ResponseStructNames {
                variant_ident: format_ident!("Default"),
                struct_name: "BodyDefault".to_string(),
                header_name: "HeadersDefault".to_string(),
            };

            process_response(
                resp,
                refs,
                &mut types,
                &mut response_enum,
                &mut headers_enum,
                names,
            );
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

fn process_response(
    resp: Response,
    refs: &ReferenceableAPI,
    types: &mut TypeSpace,
    response_enum: &mut ItemEnum,
    headers_enum: &mut ItemEnum,
    names: ResponseStructNames,
) {
    let response_variant =
        to_response_variant(&resp, refs, types, &names.variant_ident, &names.struct_name);
    response_enum.variants.push(response_variant);

    let header_variant =
        to_header_variant(resp, refs, types, &names.variant_ident, names.header_name);
    if let Some(header_variant) = header_variant {
        headers_enum.variants.push(header_variant);
    }
}

fn to_response_variant(
    resp: &Response,
    refs: &ReferenceableAPI,
    types: &mut TypeSpace,
    variant_ident: &syn::Ident,
    struct_name: &str,
) -> syn::Variant {
    let content = &resp.content;

    let ty = content_to_tokens(refs, types, content, struct_name);

    let desc = &resp.description;
    parse_quote! {
        #[doc = #desc]
        #variant_ident(#ty)
    }
}

fn to_header_variant(
    resp: Response,
    refs: &ReferenceableAPI,
    types: &mut TypeSpace,
    variant_ident: &syn::Ident,
    header_name: String,
) -> Option<syn::Variant> {
    let headers = &resp.headers;
    let header_variant = if !headers.is_empty() {
        let schema = (refs, headers.clone()).to_schema();
        let type_id = types
            .add_type_with_name(&schema, Some(header_name))
            .unwrap();

        let t = types.get_type(&type_id).unwrap();
        let t_ident = t.ident();
        let header_variant = parse_quote! {
            #variant_ident(#t_ident)
        };

        Some(header_variant)
    } else {
        None
    };

    header_variant
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
