use typify::{TypeSpace, TypeSpaceSettings};

use super::*;

pub(crate) trait AsRequestMod {
    fn as_request_mod(&self, refs: &ReferenceableAPI) -> syn::ItemMod;
}

impl AsRequestMod for Operation {
    fn as_request_mod(&self, refs: &ReferenceableAPI) -> syn::ItemMod {
        let mut settings = TypeSpaceSettings::default();
        settings.with_type_mod("my_fake_mod");
        let mut types = TypeSpace::new(&settings);

        if let Some(request_body) = &self.request_body {
            let request_body = refs.resolve(request_body).unwrap();

            content_to_tokens(refs, &mut types, &request_body.content, "Body");
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
            let param = refs.resolve(param).unwrap();
            match param {
                Parameter::Query { parameter_data, .. } => {
                    add_field_for_param(
                        refs,
                        &mut types,
                        &parameter_data,
                        struct_fields,
                        "InnerParam",
                    );
                }
                Parameter::Header { parameter_data, .. } => add_field_for_param(
                    refs,
                    &mut types,
                    &parameter_data,
                    header_fields,
                    "InnerHeader",
                ),
                Parameter::Path { parameter_data, .. } => {
                    add_field_for_param(refs, &mut types, &parameter_data, path_fields, "InnerPath")
                }
                Parameter::Cookie { .. } => todo!(),
            };
        }

        let types_content = types.to_stream();
        let mut request_mod: syn::ItemMod = parse_quote! {
            pub mod request {
              #types_content
            }
        };
        let content = &mut request_mod.content.as_mut().unwrap().1;

        content.push(param_struct.into());
        content.push(headers_struct.into());
        content.push(path_struct.into());

        request_mod
    }
}

fn add_field_for_param(
    refs: &ReferenceableAPI,
    types: &mut TypeSpace,
    parameter_data: &ParameterData,
    struct_fields: &mut syn::FieldsNamed,
    default_struct_name: &str,
) {
    let name = &parameter_data.name;
    let ident = format_ident!("{name}");

    let desc = &parameter_data.description;
    if let ParameterSchemaOrContent::Schema(s) = &parameter_data.format {
        let s = refs.resolve(s).unwrap();

        let ty = s.as_type(types, default_struct_name);

        let mut field = syn::Field::parse_named
            .parse2(quote::quote! {
                pub #ident: #ty
            })
            .unwrap();
        if let Some(desc) = desc {
            field.attrs.push(parse_quote!(#[doc = #desc]));
        }
        struct_fields.named.push(field);
    } else {
        todo!("Need to handle cases where we have the nested content instead of a schema")
    }
}
