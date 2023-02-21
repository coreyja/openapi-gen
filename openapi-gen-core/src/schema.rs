pub use self::to_schema::ToSchema;

use super::*;

mod value;
use typify::TypeSpace;
pub(crate) use value::*;

mod to_schema;
pub(crate) trait IntoType {
    fn as_type(&self, types: &mut TypeSpace, name: &str) -> TokenStream;
}

impl IntoType for Schema {
    fn as_type(&self, types: &mut TypeSpace, new_struct_name: &str) -> TokenStream {
        let schema: schemars::schema::Schema = self.clone().to_schema();
        dbg!(&schema);

        let tid = types
            .add_type_with_name(&schema, Some(new_struct_name.to_string()))
            .unwrap();

        let t = types.get_type(&tid).unwrap();
        t.ident()
        // match &self.schema_kind {
        //     SchemaKind::Type(t) => into_type(refs, t, new_structs, new_struct_name, count),
        //     SchemaKind::OneOf { .. } => todo!("Generate an enum from the possible schemas"),
        //     SchemaKind::AllOf { .. } => {
        //         todo!("IDK... Try to make a struct thats the union of all the schemas?")
        //     }
        //     SchemaKind::AnyOf { .. } => todo!("Is this the same as oneOf?"),
        //     SchemaKind::Not { .. } => {
        //         todo!("Dont think we can really support a NOT schema.... Just skip it?")
        //     }
        //     SchemaKind::Any(_) => todo!("Untyped schema... Just skip it?"),
        // }
    }
}
