use indexmap::IndexMap;
use inflector::Inflector;

use super::*;

impl IntoMod for Responses {
    fn into_mod(self) -> syn::ItemMod {
        let mut response_enum: ItemEnum = parse_quote! {
          #[doc="Test this Response"]
          #[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
          pub enum Body { }
        };

        let mut structs: Vec<ItemStruct> = vec![];
        for (status_code, resp) in self.responses {
            let resp = resp.as_item().unwrap();
            let variant_ident = format_ident!("_{status_code}");
            let content = &resp.content;

            let struct_ident = format!("Body{status_code}");
            let ty = content_to_tokens(content, &mut structs, status_code.clone(), &struct_ident);

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
                    let schema = schema.as_item().unwrap();

                    let field_ty = schema.as_type(&mut structs, header_name, 0);

                    header_fields.named.push(
                        syn::Field::parse_named
                            .parse2(quote::quote! { pub #field_ident: #field_ty })
                            .unwrap(),
                    );
                }

                structs.push(header_struct);
            }

            response_enum.variants.push(parse_quote! {
              #variant_ident(#ty)
            });
        }

        let mut response_mod: syn::ItemMod = parse_quote! {
            pub mod response {}
        };
        let contents = &mut response_mod.content.as_mut().unwrap().1;

        contents.push(response_enum.into());
        for s in structs {
            contents.push(s.into());
        }

        response_mod
    }
}

pub(crate) fn content_to_tokens(
    content: &IndexMap<String, MediaType>,
    structs: &mut Vec<ItemStruct>,
    status_code: StatusCode,
    struct_ident: &str,
) -> TokenStream {
    let json_content = content.get("application/json").unwrap().clone();

    if let Some(schema) = json_content.schema {
        let schema = schema.as_item().unwrap();

        schema.as_type(structs, &struct_ident, 0)
    } else {
        let mut iter = json_content.examples.into_iter();
        let item = iter.next().unwrap();
        let example_value = item.1.into_item();
        let example_value = example_value.unwrap().value.unwrap();

        type_for(&example_value, structs, &struct_ident, 0)
    }
}