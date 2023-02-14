use super::*;

impl IntoMods for Paths {
    fn as_mods(&self) -> Vec<syn::ItemMod> {
        self.iter()
            .map(|(path, item)| Path(path, item).as_mod())
            .collect()
    }
}

struct Path<'a>(&'a str, &'a ReferenceOr<PathItem>);

impl IntoMod for Path<'_> {
    fn as_mod(&self) -> syn::ItemMod {
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
            path_ident
        };
        let path_ident = format_ident!("{}", path_ident);

        let item = item.as_item().unwrap();

        // let mut id = item.get.unwrap().operation_id.unwrap();
        // make_ascii_titlecase(&mut id);

        // let id = format_ident!("{}", id);

        let mut path_mod: syn::ItemMod = parse_quote! {
            pub mod #path_ident {}
        };
        let content = &mut path_mod.content.as_mut().unwrap().1;

        if let Some(op) = &item.get {
            content.push(("get", op).as_mod().into());
        }
        if let Some(op) = &item.post {
            content.push(("post", op).as_mod().into());
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
        let spec_string = include_str!("../tests/simple_site.json");
        let spec: OpenAPI = serde_json::from_str(spec_string).unwrap();

        let paths = spec.paths;
        let mods = paths.as_mods();

        let names: Vec<_> = mods.iter().map(|m| m.ident.to_string()).collect();
        assert_eq!(names, vec!["test_more", "root"]);
    }
}
