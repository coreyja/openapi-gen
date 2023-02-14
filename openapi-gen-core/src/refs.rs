use openapiv3::{OpenAPI, ReferenceOr};

pub(crate) struct ReferenceableAPI(pub OpenAPI);

impl ReferenceableAPI {
    fn resolve<T>(r: ReferenceOr<T>) -> Result<T, String> {
        todo!()
    }
}
