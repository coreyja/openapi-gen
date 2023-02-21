use typify::{TypeSpace, TypeSpaceSettings};

use super::*;

pub(crate) trait AsRequestMod {
    fn as_request_mod(&self, refs: &ReferenceableAPI) -> syn::ItemMod;
}

impl AsRequestMod for Operation {
    fn as_request_mod(&self, refs: &ReferenceableAPI) -> syn::ItemMod {
        let mut settings = TypeSpaceSettings::default();
        settings.with_type_mod("self");
        settings.with_derive("PartialEq".to_owned());
        let mut types = TypeSpace::new(&settings);

        refs.add_reference_schemas(&mut types).unwrap();

        if let Some(request_body) = &self.request_body {
            let request_body = refs.resolve(request_body).unwrap();

            content_to_tokens(refs, &mut types, &request_body.content, "Body");
        }

        let params: Vec<Parameter> = self
            .parameters
            .iter()
            .map(|p| refs.resolve(p).unwrap())
            .collect();
        let schemas: ParamSchemas = params.into();
        schemas.cookie.add_type(&mut types, "Cookies");
        schemas.query.add_type(&mut types, "QueryParams");
        schemas.path.add_type(&mut types, "PathParams");
        schemas.header.add_type(&mut types, "Headers");

        let types_content = types.to_stream();
        let request_mod: syn::ItemMod = parse_quote! {
            pub mod request {
              use serde::{Serialize, Deserialize};

              #types_content
            }
        };
        // let content = &mut request_mod.content.as_mut().unwrap().1;

        // content.push(param_struct.into());
        // content.push(headers_struct.into());
        // content.push(path_struct.into());

        request_mod
    }
}

fn add_field_for_param(
    types: &mut TypeSpace,
    parameter_data: &ParameterData,
    struct_fields: &mut syn::FieldsNamed,
    default_struct_name: &str,
) {
    let name = &parameter_data.name;
    let ident = format_ident!("{name}");

    let desc = &parameter_data.description;
    if let ParameterSchemaOrContent::Schema(s) = &parameter_data.format {
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
