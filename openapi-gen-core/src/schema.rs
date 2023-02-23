pub use self::to_schema::ToSchema;

use super::*;

mod value;
use typify::TypeSpace;
pub(crate) use value::*;

mod headers;
pub use headers::*;

mod parameters;
pub(crate) use parameters::*;

mod to_schema;
pub(crate) trait IntoType {
    fn as_type(&self, types: &mut TypeSpace, name: &str) -> TokenStream;
}

impl ToSchema for schemars::schema::Schema {
    fn to_schema(&self) -> schemars::schema::Schema {
        self.clone()
    }
}

impl<T> IntoType for T
where
    T: ToSchema,
{
    fn as_type(&self, types: &mut TypeSpace, new_struct_name: &str) -> TokenStream {
        let schema: schemars::schema::Schema = self.to_schema();

        let tid = types
            .add_type_with_name(&schema, Some(new_struct_name.to_string()))
            .unwrap();

        let t = types.get_type(&tid).unwrap();
        t.ident()
    }
}
