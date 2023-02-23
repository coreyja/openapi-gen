use std::{fs, path::PathBuf};

use openapi_gen_core::{api, MacroArgs};
use similar::{ChangeTag, TextDiff};
use syn::parse_file;

pub fn test_fixture_snapshot(fixture_name: &str) {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("../fixtures/");
    path.push(fixture_name);

    let args_path = path.to_str().unwrap().to_string();
    let args = MacroArgs {
        path: format!("../fixtures/{fixture_name}"),
        name: Some("test".to_string()),
    };
    let actual = api(args);

    let mut expected_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    expected_path.push("tests/snapshots");
    expected_path.push(fixture_name);

    let expected_path = expected_path.with_extension("expected.rs");

    if option_env!("CREATE_SNAPSHOTS").is_some() {
        let parsed = parse_file(&actual.to_string()).unwrap();
        let formatted = prettyplease::unparse(&parsed);
        std::fs::write(&expected_path, formatted).unwrap();
    }

    let expected_content = fs::read_to_string(&expected_path).unwrap();
    let expected = expected_content.parse().unwrap();

    assert_token_streams_match(actual, expected)
}
pub fn assert_token_streams_match(
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

        println!("Actual:\n{formatted_actual}");

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
