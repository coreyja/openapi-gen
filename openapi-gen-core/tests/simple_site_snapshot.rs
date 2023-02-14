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
        path: "../fixtures/simple_site.json".to_string(),
    };
    let input = quote::quote! {
        mod test {}
    };

    let expected = parse_quote! {
      mod test {
        pub mod test_more {
          pub mod post {
            pub mod request {
              #[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
              pub struct QueryParams {
                  pub test: InnerParam,
              }
              #[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
              pub struct Headers {
                  pub RandomKey: String,
              }
              #[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
              pub struct Body {
                  pub foo: String,
                  pub bar: String,
              }
              #[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
              pub struct InnerParam {
                  pub foo: String,
                  pub bar: String,
              }
            }
            pub mod response {
              ///Test this Response
              #[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
              pub enum Body {
                  _200(Body200),
              }
              #[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
              pub enum Headers {
                  _200(Headers200),
              }
              #[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
              pub struct Body200 {
                  pub id: String,
                  pub name: String,
              }
              #[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
              pub struct Headers200 {
                  pub x_test: String,
              }
            }
          }
        }
        pub mod root {
          pub mod get {
            pub mod request {
              #[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
              pub struct QueryParams {}
              #[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
              pub struct Headers {}
            }
            pub mod response {
              ///Test this Response
              #[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
              pub enum Body {
                  _200(Body200),
              }
              #[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
              pub struct Link {
                  pub href: String,
                  pub rel: String,
              }
              #[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
              pub struct Version {
                  pub status: String,
                  pub updated: String,
                  pub id: String,
                  pub links: Vec<Link>,
              }
              #[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
              pub struct Body200 {
                  pub versions: Vec<Version>,
              }
            }
          }
        }
      }
    };

    assert_token_streams_match(api(args, input), expected);
}
