use indexmap::IndexMap;
use inflector::Inflector;

use super::*;

impl IntoMod for Responses {
    fn as_mod(&self, refs: &ReferenceableAPI) -> syn::ItemMod {
        let mut response_enum: ItemEnum = parse_quote! {
          #[doc="Test this Response"]
          #[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
          pub enum Body { }
        };
        let mut headers_enum: ItemEnum = parse_quote! {
          #[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
          pub enum Headers { }
        };

        let mut response_mod: syn::ItemMod = parse_quote! {
            pub mod response {}
        };
        let contents = &mut response_mod.content.as_mut().unwrap().1;

        let mut structs: Vec<ItemStruct> = vec![];
        let mut header_structs: Vec<ItemStruct> = vec![];
        for (status_code, resp) in &self.responses {
            let resp = resp.as_item().unwrap();
            let variant_ident = format_ident!("_{status_code}");
            let content = &resp.content;

            if !content.is_empty() {
                let struct_ident = format!("Body{status_code}");
                let ty = content_to_tokens(refs, content, &mut structs, &struct_ident);

                response_enum.variants.push(parse_quote! {
                  #variant_ident(#ty)
                });
            }

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
                    let header = header.as_item().unwrap();
                    let field_ident = format_ident!("{}", header_name.to_snake_case());
                    let ParameterSchemaOrContent::Schema(schema) = &header.format else { panic!("We only support schemas for headers for now")};
                    let schema = refs.resolve(schema).unwrap();

                    let field_ty = schema.as_type(refs, &mut header_structs, header_name, 0);

                    header_fields.named.push(
                        syn::Field::parse_named
                            .parse2(quote::quote! { pub #field_ident: #field_ty })
                            .unwrap(),
                    );
                }

                header_structs.push(header_struct);
                headers_enum.variants.push(parse_quote! {
                    #variant_ident(#header_struct_ident)
                });
            }
        }

        contents.push(response_enum.into());
        if !header_structs.is_empty() {
            contents.push(headers_enum.into());
        }
        for s in structs {
            contents.push(s.into());
        }
        for s in header_structs {
            contents.push(s.into());
        }

        response_mod
    }
}

pub(crate) fn content_to_tokens(
    refs: &ReferenceableAPI,
    content: &IndexMap<String, MediaType>,
    structs: &mut Vec<ItemStruct>,
    struct_ident: &str,
) -> TokenStream {
    if content.is_empty() {
        return quote! { () };
    }

    let json_content = content.get("application/json").unwrap().clone();

    if let Some(schema) = json_content.schema {
        let schema = refs.resolve(&schema).unwrap();

        schema.as_type(refs, structs, struct_ident, 0)
    } else {
        let mut iter = json_content.examples.into_iter();
        let item = iter.next().unwrap();
        let example_value = item.1.into_item();
        let example_value = example_value.unwrap().value.unwrap();

        type_for(refs, &example_value, structs, struct_ident, 0)
    }
}
