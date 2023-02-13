use super::*;

pub(crate) trait AsRequestMod {
    fn as_request_mod(&self, refs: &ReferenceableAPI) -> syn::ItemMod;
}

impl AsRequestMod for Operation {
    fn as_request_mod(&self, refs: &ReferenceableAPI) -> syn::ItemMod {
        let mut request_mod: syn::ItemMod = parse_quote! {
            pub mod request {}
        };
        let content = &mut request_mod.content.as_mut().unwrap().1;
        let mut structs: Vec<ItemStruct> = vec![];

        if let Some(request_body) = &self.request_body {
            let request_body = refs.resolve(request_body).unwrap();

            content_to_tokens(refs, &request_body.content, &mut structs, "Body");
        }

        let mut param_struct: ItemStruct = parse_quote! {
          #[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
          pub struct QueryParams {}
        };
        let mut headers_struct: ItemStruct = parse_quote! {
          #[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
          pub struct Headers {}
        };
        let mut path_struct: ItemStruct = parse_quote! {
          #[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
          pub struct PathParams {}
        };

        let Fields::Named(ref mut struct_fields) = param_struct.fields else { panic!("This should always be named cause we just made the struct") };
        let Fields::Named(ref mut header_fields) = headers_struct.fields else { panic!("This should always be named cause we just made the struct") };
        let Fields::Named(ref mut path_fields) = path_struct.fields else { panic!("This should always be named cause we just made the struct") };

        for param in &self.parameters {
            let param = param.as_item().unwrap();
            match param {
                Parameter::Query { parameter_data, .. } => {
                    add_field_for_param(
                        refs,
                        parameter_data,
                        &mut structs,
                        struct_fields,
                        "InnerParam",
                    );
                }
                Parameter::Header { parameter_data, .. } => add_field_for_param(
                    refs,
                    parameter_data,
                    &mut structs,
                    header_fields,
                    "InnerHeader",
                ),
                Parameter::Path { parameter_data, .. } => add_field_for_param(
                    refs,
                    parameter_data,
                    &mut structs,
                    path_fields,
                    "InnerPath",
                ),
                Parameter::Cookie { .. } => todo!(),
            };
        }

        content.push(param_struct.into());
        content.push(headers_struct.into());
        content.push(path_struct.into());

        for i in structs {
            content.push(i.into())
        }

        request_mod
    }
}

fn add_field_for_param(
    refs: &ReferenceableAPI,
    parameter_data: &ParameterData,
    structs: &mut Vec<ItemStruct>,
    struct_fields: &mut syn::FieldsNamed,
    default_struct_name: &str,
) {
    let name = &parameter_data.name;
    let ident = format_ident!("{name}");

    if let ParameterSchemaOrContent::Schema(s) = &parameter_data.format {
        let s = s.as_item().unwrap();

        let ty = s.as_type(refs, structs, default_struct_name, 0);

        struct_fields.named.push(
            syn::Field::parse_named
                .parse2(quote::quote! { pub #ident: #ty })
                .unwrap(),
        );
    } else {
        todo!("Need to handle cases where we have the nested content instead of a schema")
    }
}
