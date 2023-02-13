use std::hash::Hash;

use indexmap::IndexMap;

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
            let ty = content_to_tokens(content, &mut structs, status_code, &struct_ident);

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
