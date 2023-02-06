use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote, Item};

#[proc_macro_attribute]
pub fn api(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut item_mod = parse_macro_input!(input as syn::ItemMod);

    if item_mod.content.is_none() {
        panic!("The mod can't be empty");
    }

    item_mod.content.as_mut().unwrap().1.push(parse_quote! {
          pub struct New {}
    });

    quote! {
        #item_mod
    }
    .into()
}
