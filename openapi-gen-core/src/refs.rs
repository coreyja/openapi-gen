use std::borrow::Borrow;

use openapiv3::{Components, OpenAPI, ReferenceOr, Schema};
use regex::Regex;

pub(crate) struct ReferenceableAPI(pub OpenAPI);

pub(crate) trait Refable: Sized {
    fn resolve<'a>(refs: &'a Components, r: &'a str) -> Result<&'a Self, String>;

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
            ReferenceOr::Reference { reference } => {
                let components = self
                    .0
                    .components
                    .as_ref()
                    .ok_or_else(|| "No refable components in the spec file".to_owned())?;
                ReturnType::resolve(components, reference).map(Clone::clone)
            }
            ReferenceOr::Item(value) => Ok(value.borrow().clone()),
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
