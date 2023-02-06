use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, parse_quote, Item};

#[proc_macro_attribute]
pub fn api(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut item_mod = parse_macro_input!(input as syn::ItemMod);

    if item_mod.content.is_none() {
        return quote_spanned! {
          item_mod.ident.span() =>
            compile_error!("Non-inline modules are not supported");
        }
        .into();
    }

    item_mod.content.as_mut().unwrap().1.push(parse_quote! {
          pub struct New {}
    });

    quote! {
        #item_mod
    }
    .into()
}
