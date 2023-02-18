use similar::{ChangeTag, TextDiff};
use syn::parse_file;

pub(crate) fn assert_token_streams_match(
    actual: proc_macro2::TokenStream,
    expected: proc_macro2::TokenStream,
) {
    let string_actual = actual.to_string();
    let string_expected = expected.to_string();

    let parsed_file_actual = parse_file(&string_actual).unwrap();
    let parsed_file_expected = parse_file(&string_expected).unwrap();

    let formatted_actual = prettyplease::unparse(&parsed_file_actual);
    let formatted_expected = prettyplease::unparse(&parsed_file_expected);

    if formatted_actual != formatted_expected {
        let diff = TextDiff::from_lines(&formatted_expected, &formatted_actual);

        println!("Diff:");
        for change in diff.iter_all_changes() {
            let sign = match change.tag() {
                ChangeTag::Delete => "-",
                ChangeTag::Insert => "+",
                ChangeTag::Equal => " ",
            };
            print!("{sign}{change}");
        }

        panic!("Token streams do not match");
    }
}