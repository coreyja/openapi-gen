use super::*;

impl IntoMod for (&str, &Operation) {
    fn as_mod(&self, refs: &ReferenceableAPI) -> syn::ItemMod {
        let (ident, operation) = self;

        let ident = ident.to_ascii_lowercase();
        let ident = format_ident!("{ident}");
        let mut operation_mod: ItemMod = parse_quote! {
          pub mod #ident {}
        };

        let content = &mut operation_mod.content.as_mut().unwrap().1;

        content.push(operation.as_request_mod(refs).into());
        content.push(operation.responses.as_mod(refs).into());

        operation_mod
    }
}
