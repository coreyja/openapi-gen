use openapi_gen_core::{api, MacroArgs};
use quote::quote;
use similar::{ChangeTag, TextDiff};
use syn::{parse2, parse_file, parse_quote};

fn assert_token_streams_match(
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

#[test]
fn test_simple_site() {
    let args = MacroArgs {
        path: "tests/simple_site.json".to_string(),
    };
    let input = quote::quote! {
        mod test {}
    };

    let expected = parse_quote! {
      mod test {
        pub mod test_more {
          pub mod get {
            pub mod request {
              pub struct QueryParams {
                  pub test: InnerParam,
              }
              pub struct Headers {
                  pub RandomKey: String,
              }
              pub struct Body {}
              pub struct InnerParam {
                  pub foo: String,
                  pub bar: String,
              }
            }
            pub mod response {
              ///Test this Response
              pub enum Body {
                  _200(Body200),
              }
              pub struct Body200 {
                  pub id: String,
                  pub name: String,
              }
            }
          }
        }
        pub mod root {
          pub mod get {
            pub mod request {
              pub struct QueryParams {}
              pub struct Headers {}
              pub struct Body {}
            }
            pub mod response {
              ///Test this Response
              pub enum Body {
                  _200(Body200),
              }
              pub struct Body200_4 {
                  pub href: String,
                  pub rel: String,
              }
              pub struct Body200_2 {
                  pub status: String,
                  pub updated: String,
                  pub id: String,
                  pub links: Vec<Body200_4>,
              }
              pub struct Body200 {
                  pub versions: Vec<Body200_2>,
              }
            }
          }
        }
      }
    };

    assert_token_streams_match(api(args, input), expected);
}
