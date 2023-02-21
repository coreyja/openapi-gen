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

        let mut header_structs: Vec<ItemStruct> = vec![];
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

            let header_struct_ident = format!("Headers{status_code}");
            let header_struct_ident = format_ident!("{}", header_struct_ident);
            let headers = &resp.headers;
            if !headers.is_empty() {
                let mut header_struct: ItemStruct = parse_quote! {
                  #[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
                  pub struct #header_struct_ident {}
                };

                let Fields::Named(ref mut header_fields) = header_struct.fields
                    else { panic!("This should always be named cause we just made the struct") };

                for (header_name, header) in headers {
                    let header = refs.resolve(header).unwrap();
                    let field_ident = format_ident!("{}", header_name.to_snake_case());
                    let ParameterSchemaOrContent::Schema(schema) = &header.format else { panic!("We only support schemas for headers for now")};
                    let schema = refs.resolve(schema).unwrap();

                    let field_ty = schema.as_type(&mut types, header_name);

                    let mut field = syn::Field::parse_named
                        .parse2(quote::quote! { pub #field_ident: #field_ty })
                        .unwrap();
                    if let Some(desc) = &header.description {
                        field.attrs.push(parse_quote! {
                            #[doc = #desc]
                        });
                    };
                    header_fields.named.push(field);
                }

                header_structs.push(header_struct);
                headers_enum.variants.push(parse_quote! {
                    #variant_ident(#header_struct_ident)
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
        if !header_structs.is_empty() {
            contents.push(headers_enum.into());
        }
        for s in header_structs {
            contents.push(s.into());
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
        let schema = refs.resolve(&schema).unwrap();

        schema.as_type(types, struct_ident)
    } else {
        let mut iter = json_content.examples.into_iter();
        let (_name, example_value) = iter.next().unwrap();
        let example_value = refs.resolve(&example_value).unwrap();
        let example_value = example_value.value.unwrap();

        type_for(types, &example_value, struct_ident)
    }
}
