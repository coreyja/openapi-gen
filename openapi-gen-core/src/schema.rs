use super::*;
pub(crate) trait IntoType {
    fn as_type(&self, new_structs: &mut Vec<ItemStruct>, name: &str, count: usize) -> TokenStream;
}

impl IntoType for Schema {
    fn as_type(&self, new_structs: &mut Vec<ItemStruct>, name: &str, count: usize) -> TokenStream {
        match &self.schema_kind {
            SchemaKind::Type(t) => into_type(t, new_structs, name, count),
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
    t: &Type,
    new_structs: &mut Vec<ItemStruct>,
    name: &str,
    count: usize,
) -> TokenStream {
    match t {
        Type::String(_) => quote::quote!(String),
        Type::Number(_) => quote::quote!(f64),
        Type::Integer(_) => quote::quote!(i64),
        Type::Object(o) => {
            let ident = if count == 0 {
                name.to_string()
            } else {
                format!("{name}_{count}")
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
                    let s = value.as_item().unwrap();

                    let ty = s.as_type(new_structs, name, count + 1);

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
                    let s = s.as_item().unwrap();

                    s.as_type(new_structs, name, count + 1)
                }
                None => todo!("What do we do with empty arrays?"),
            };

            quote::quote!(Vec<#ty>)
        }
        Type::Boolean {} => quote::quote!(bool),
    }
}

pub(crate) fn type_for_value(value: &Value) -> Type {
    match value {
        Value::Null => todo!("What do we do with nulls?"),
        Value::Bool(_) => Type::Boolean {},
        Value::Number(n) => {
            if n.is_f64() {
                Type::Number(Default::default())
            } else {
                Type::Integer(Default::default())
            }
        }
        Value::String(_) => Type::String(Default::default()),
        Value::Array(a) => {
            let sample = a.iter().next();
            let ty = match sample {
                Some(v) => type_for_value(v),
                None => todo!("What do we do with empty arrays?"),
            };

            let s = Schema {
                schema_data: Default::default(),
                schema_kind: SchemaKind::Type(ty),
            };
            let s = Box::new(s);
            let s = ReferenceOr::Item(s);

            Type::Array(ArrayType {
                items: s.into(),
                min_items: None,
                max_items: None,
                unique_items: false,
            })
        }
        Value::Object(o) => {
            let mut properties = Vec::<(String, _)>::new();
            for (key, value) in o.iter() {
                let ty = type_for_value(value);
                let s = Schema {
                    schema_data: Default::default(),
                    schema_kind: SchemaKind::Type(ty),
                };
                let s = Box::new(s);
                let s = ReferenceOr::Item(s);

                properties.push((key.to_owned(), s));
            }

            Type::Object(ObjectType {
                properties: properties.into_iter().collect(),
                required: vec![],
                additional_properties: None,
                min_properties: None,
                max_properties: None,
            })
        }
    }
}

pub(crate) fn type_for(
    value: &Value,
    new_structs: &mut Vec<ItemStruct>,
    name: &str,
    count: usize,
) -> TokenStream {
    let openapi_type = type_for_value(value);

    into_type(&openapi_type, new_structs, name, count)
}
