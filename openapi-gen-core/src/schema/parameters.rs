use std::collections::{BTreeMap, BTreeSet};

use openapiv3::{Parameter, ParameterData, ParameterSchemaOrContent};
use schemars::schema::{ObjectValidation, Schema, SchemaObject};
use typify::TypeSpace;

use super::{IntoType, ToSchema};

pub(crate) struct MaybeSchema(Option<Schema>);

pub(crate) struct ParamSchemas {
    pub path: MaybeSchema,
    pub header: MaybeSchema,
    pub query: MaybeSchema,
    pub cookie: MaybeSchema,
}

impl From<Vec<Parameter>> for ParamSchemas {
    fn from(value: Vec<Parameter>) -> Self {
        let mut paths = vec![];
        let mut headers = vec![];
        let mut query = vec![];
        let mut cookie = vec![];

        for p in value {
            match p.clone() {
                Parameter::Query { parameter_data, .. } => query.push(parameter_data),
                Parameter::Header { parameter_data, .. } => headers.push(parameter_data),
                Parameter::Path { parameter_data, .. } => paths.push(parameter_data),
                Parameter::Cookie { parameter_data, .. } => cookie.push(parameter_data),
            }
        }

        ParamSchemas {
            path: paths.into(),
            header: headers.into(),
            query: query.into(),
            cookie: cookie.into(),
        }
    }
}

impl From<Vec<ParameterData>> for MaybeSchema {
    fn from(value: Vec<ParameterData>) -> Self {
        if value.is_empty() {
            return MaybeSchema(None);
        }

        let mut properties: BTreeMap<String, Schema> = Default::default();
        let mut required: BTreeSet<String> = Default::default();

        for param in value {
            let ParameterSchemaOrContent::Schema(schema) = param.format else {
              todo!();
            };
            let schema = schema.to_schema();

            properties.insert(param.name.clone(), schema);

            if param.required {
                required.insert(param.name.clone());
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

        let schema = Schema::Object(schema);

        MaybeSchema(Some(schema))
    }
}

impl MaybeSchema {
    pub(crate) fn add_type(&self, types: &mut TypeSpace, name: &str) {
        let Some(schema) = &self.0 else { return; };

        schema.as_type(types, name);
    }
}
