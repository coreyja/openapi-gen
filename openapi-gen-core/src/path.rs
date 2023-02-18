use super::*;

impl IntoMods for Paths {
    fn as_mods(&self, refs: &ReferenceableAPI) -> Vec<syn::ItemMod> {
        self.iter()
            .map(|(path, item)| Path(path, item).as_mod(refs))
            .collect()
    }
}

struct Path<'a>(&'a str, &'a ReferenceOr<PathItem>);

impl IntoMod for Path<'_> {
    fn as_mod(&self, refs: &ReferenceableAPI) -> syn::ItemMod {
        let path = self.0;
        let item = self.1;

        let path_parts: Vec<_> = path
            .to_ascii_lowercase()
            .split('/')
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();
        let path_ident = path_parts.join("_");
        let path_ident = if path_ident.is_empty() {
            "root".to_string()
        } else {
            // TODO: Handle Path Parameters better here
            path_ident.replace(['{', '}'], "")
        };
        let path_ident = format_ident!("{}", path_ident);

        let item = item.as_item().expect("This is assumed to be safe since I don't believe PathItems can be referenced in components");

        let mut path_mod: syn::ItemMod = parse_quote! {
            pub mod #path_ident {}
        };
        let content = &mut path_mod.content.as_mut().unwrap().1;

        if let Some(op) = &item.get {
            content.push(("get", op).as_mod(refs).into());
        }
        if let Some(op) = &item.post {
            content.push(("post", op).as_mod(refs).into());
        }
        // TODO: Need to do the rest of the operations
        // annoying there isn't any easy loop that I found

        path_mod
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_path_mod_names() {
        let spec_string = include_str!("../../fixtures/simple_site.json");
        let spec: OpenAPI = serde_json::from_str(spec_string).unwrap();
        let refs = ReferenceableAPI(spec);

        let mods = refs.0.paths.as_mods(&refs);

        let names: Vec<_> = mods.iter().map(|m| m.ident.to_string()).collect();
        assert_eq!(names, vec!["test_more", "root"]);
    }
}
