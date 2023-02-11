use openapi_gen_core::darling::FromMeta;
use openapi_gen_core::MacroArgs;

use openapi_gen_core::syn::parse_macro_input;

#[proc_macro_attribute]
pub fn api(
    attr_stream: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let attrs = parse_macro_input!(attr_stream as openapi_gen_core::syn::AttributeArgs);
    let args = match MacroArgs::from_list(&attrs) {
        Ok(v) => v,
        Err(e) => {
            return proc_macro::TokenStream::from(e.write_errors());
        }
    };

    openapi_gen_core::api(args, input.into()).into()
}
