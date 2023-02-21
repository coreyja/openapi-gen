use std::{
    collections::{BTreeMap, BTreeSet, HashSet},
    default,
};

use indexmap::IndexMap;
use openapiv3::{Header, ObjectType, ParameterSchemaOrContent, ReferenceOr};
use schemars::schema::{Metadata, ObjectValidation, Schema, SchemaObject};

use crate::refs::ReferenceableAPI;

use super::{IntoType, ToSchema};

impl ToSchema for (&ReferenceableAPI, IndexMap<String, ReferenceOr<Header>>) {
    fn to_schema(&self) -> schemars::schema::Schema {
        let (refs, headers) = self;
        let mut properties: BTreeMap<String, Schema> = Default::default();
        let mut required: BTreeSet<String> = Default::default();

        for (key, value) in headers {
            let header = refs.resolve(value).unwrap();
            let ParameterSchemaOrContent::Schema(schema) = header.format else {
              todo!();
            };
            let mut schema = schema.to_schema();

            let Schema::Object(x) = &mut schema else { todo!() };

            let m = x.metadata();
            m.description = header.description.clone();

            properties.insert(key.to_string(), schema);

            if header.required {
                required.insert(key.to_string());
            }
        }

        let schema = SchemaObject {
            metadata: None,
            instance_type: None,
            format: None,
            enum_values: None,
            const_value: None,
            subschemas: None,
            number: None,
            string: None,
            array: None,
            object: Some(Box::new(ObjectValidation {
                properties,
                required,
                ..Default::default()
            })),
            reference: None,
            extensions: Default::default(),
        };

        Schema::Object(schema)
    }
}
