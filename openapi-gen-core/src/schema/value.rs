use super::*;

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

            let keys = o.keys().cloned().collect();

            Type::Object(ObjectType {
                properties: properties.into_iter().collect(),
                required: keys,
                additional_properties: None,
                min_properties: None,
                max_properties: None,
            })
        }
    }
}

pub(crate) fn type_for(types: &mut TypeSpace, value: &Value, name: &str) -> TokenStream {
    let openapi_type = type_for_value(value);

    let schema = Schema {
        schema_data: Default::default(),
        schema_kind: SchemaKind::Type(openapi_type),
    };
    let schema = schema.to_schema();
    let tid = types
        .add_type_with_name(&schema, Some(name.to_string()))
        .unwrap();

    let t = types.get_type(&tid).unwrap();
    t.ident()
}
