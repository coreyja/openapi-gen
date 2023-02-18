use openapiv3::{Components, OpenAPI, ReferenceOr, Schema};
use regex::Regex;

pub(crate) struct ReferenceableAPI(pub OpenAPI);

pub(crate) trait Refable: Sized {
    fn resolve<'a>(refs: &'a Components, r: &'a str) -> Result<&'a Self, String>;

    fn regex() -> Regex;

    fn name(r: &str) -> Result<&str, String> {
        let reg: Regex = regex::Regex::new(r"#/components/schemas/(.*)").unwrap();
        let m = reg
            .captures(&r)
            .ok_or_else(|| "Reference does not match regex for Schema".to_owned())?;
        Ok(m.get(1).unwrap().as_str())
    }
}

impl ReferenceableAPI {
    pub(crate) fn resolve<'a, T: Refable>(
        &'a self,
        r: &'a ReferenceOr<T>,
    ) -> Result<&'a T, String> {
        match r {
            ReferenceOr::Reference { reference } => {
                let components = self
                    .0
                    .components
                    .as_ref()
                    .ok_or_else(|| "No refable components in the spec file".to_owned())?;
                T::resolve(components, reference).into()
            }
            ReferenceOr::Item(value) => Ok(value.into()),
        }
    }
}

impl Refable for Schema {
    fn regex() -> Regex {
        let reg: Regex = regex::Regex::new(r"#/components/schemas/(.*)").unwrap();
        reg
    }

    fn resolve<'a>(components: &'a Components, r: &'a str) -> Result<&'a Self, String> {
        let name = Self::name(r)?;

        let s = components
            .schemas
            .get(name)
            .ok_or_else(|| format!("Schema not found for: {}", name))?;

        Ok(s.as_item().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openapiv3::{OpenAPI, ReferenceOr, Schema};

    #[test]
    fn test_resolve() {
        let spec = include_str!("../../fixtures/petstore.json");
        // let spec: OpenAPI = serde_json::from_str(spec).expect("Could not deserialize input");
        let mut spec: OpenAPI = Default::default();

        let mut components = Components::default();
        let schema = Schema {
            schema_data: Default::default(),
            schema_kind: openapiv3::SchemaKind::Type(openapiv3::Type::Boolean {}),
        };
        components
            .schemas
            .insert("Error".to_string(), ReferenceOr::Item(schema.clone()));
        spec.components = Some(components);
        let spec = ReferenceableAPI(spec);

        assert_eq!(
            spec.resolve(&ReferenceOr::<Schema>::Reference {
                reference: "#/components/schemas/Error".to_owned()
            }),
            Ok(&schema)
        );
        assert_eq!(
            spec.resolve(&ReferenceOr::<Schema>::Reference {
                reference: "#/components/schemas/Other".to_owned()
            }),
            Err("Schema not found for: Other".to_string())
        );
    }
}
