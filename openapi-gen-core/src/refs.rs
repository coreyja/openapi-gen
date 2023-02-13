use std::borrow::Borrow;

use indexmap::IndexMap;
use openapiv3::{Components, OpenAPI, ReferenceOr, RequestBody, Schema};
use regex::Regex;

pub(crate) struct ReferenceableAPI(pub OpenAPI);

pub(crate) trait Refable: Sized + Clone {
    fn resolve<'a>(refs: &'a ReferenceableAPI, r: &'a str) -> Result<Self, String> {
        let components = refs.components()?;
        let name = Self::name(r)?;

        let s = Self::get_index_map(components)
            .get(name)
            .ok_or_else(|| format!("Schema not found for: {}", name))?;

        refs.resolve(s)
    }

    fn get_index_map(components: &Components) -> &IndexMap<String, ReferenceOr<Self>>;

    fn regex() -> Regex;

    fn name(r: &str) -> Result<&str, String> {
        let reg: Regex = Self::regex();
        let m = reg
            .captures(r)
            .ok_or_else(|| "Reference does not match regex for Schema".to_owned())?;
        Ok(m.get(1).unwrap().as_str())
    }
}

impl ReferenceableAPI {
    pub(crate) fn resolve<ReturnType>(
        &self,
        r: &ReferenceOr<impl Borrow<ReturnType>>,
    ) -> Result<ReturnType, String>
    where
        ReturnType: Refable + Clone,
    {
        match r {
            ReferenceOr::Reference { reference } => ReturnType::resolve(self, reference),
            ReferenceOr::Item(value) => Ok(value.borrow().clone()),
        }
    }

    fn components(&self) -> Result<&Components, String> {
        let components = self
            .0
            .components
            .as_ref()
            .ok_or_else(|| "No refable components in the spec file".to_owned())?;
        Ok(components)
    }
}

impl Refable for Schema {
    fn regex() -> Regex {
        regex::Regex::new(r"#/components/schemas/(.*)").unwrap()
    }

    fn get_index_map(components: &Components) -> &IndexMap<String, ReferenceOr<Self>> {
        &components.schemas
    }
}

impl Refable for RequestBody {
    fn regex() -> Regex {
        regex::Regex::new(r"#/components/requestBodies/(.*)").unwrap()
    }

    fn get_index_map(components: &Components) -> &IndexMap<String, ReferenceOr<Self>> {
        &components.request_bodies
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openapiv3::{OpenAPI, ReferenceOr, Schema};

    #[test]
    fn test_resolve() {
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
            Ok(schema)
        );
        assert_eq!(
            spec.resolve(&ReferenceOr::<Schema>::Reference {
                reference: "#/components/schemas/Other".to_owned()
            }),
            Err("Schema not found for: Other".to_string())
        );
    }
}
