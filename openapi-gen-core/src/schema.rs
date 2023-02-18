use super::*;
pub(crate) trait IntoType {
    fn as_type(
        &self,
        refs: &ReferenceableAPI,
        new_structs: &mut Vec<ItemStruct>,
        name: &str,
        count: usize,
    ) -> TokenStream;
}

impl IntoType for Schema {
    fn as_type(
        &self,
        refs: &ReferenceableAPI,
        new_structs: &mut Vec<ItemStruct>,
        new_struct_name: &str,
        count: usize,
    ) -> TokenStream {
        match &self.schema_kind {
            SchemaKind::Type(t) => into_type(refs, t, new_structs, new_struct_name, count),
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

pub(crate) fn into_type(
    refs: &ReferenceableAPI,
    t: &Type,
    new_structs: &mut Vec<ItemStruct>,
    new_struct_name: &str,
    count: usize,
) -> TokenStream {
    match t {
        Type::String(_) => quote::quote!(String),
        Type::Number(_) => quote::quote!(f64),
        Type::Integer(_) => quote::quote!(i64),
        Type::Object(o) => {
            let ident = if count == 0 {
                new_struct_name.to_string()
            } else {
                format!("{new_struct_name}_{count}")
            };
            let ident = format_ident!("{ident}");
            let mut s: ItemStruct = parse_quote! {
              #[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
              pub struct #ident {}
            };

            if let Fields::Named(ref mut fields) = s.fields {
                if o.additional_properties.is_some() {
                    todo!("We dont support additional properties yet")
                }

                for (key, value) in o.properties.iter() {
                    let s: Schema = refs.resolve(value).unwrap();

                    let new_thing_is_array =
                        matches!(&s.schema_kind, SchemaKind::Type(Type::Array(_)));

                    let new_struct_name = key.to_pascal_case();

                    let new_struct_name = if new_thing_is_array {
                        new_struct_name.to_singular()
                    } else {
                        new_struct_name
                    };

                    let ty = s.as_type(refs, new_structs, &new_struct_name, 0);

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
                    let s: Schema = refs.resolve(s).unwrap();

                    s.as_type(refs, new_structs, new_struct_name, count)
                }
                None => todo!("What do we do with empty arrays?"),
            };

            quote::quote!(Vec<#ty>)
        }
        Type::Boolean {} => quote::quote!(bool),
    }
}

mod value;
use inflector::Inflector;
pub use value::*;
