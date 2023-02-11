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

    if string_actual != string_expected {
        let formatted_actual = prettyplease::unparse(&parse_file(&string_actual).unwrap());
        let formatted_expected = prettyplease::unparse(&parse_file(&string_expected).unwrap());

        // println!("Actual:\n{formatted_actual}");
        // println!("Expected:\n{formatted_expected}");
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
        pub mod Test_More {
          pub mod Get {
              pub struct QueryParams {
                  pub test: InnerParam,
              }
              pub struct Headers {
                  pub RandomKey: String,
              }
              pub struct Request {}
              ///Test this Response
              pub enum Response {
                  _200(Response200),
              }
              pub struct Response200 {
                  pub id: String,
                  pub name: String,
              }
              pub struct InnerParam {
                  pub foo: String,
                  pub bar: String,
              }
          }
        }
        pub mod Root {
            pub mod Get {
                pub struct QueryParams {}
                pub struct Headers {}
                pub struct Request {}
                ///Test this Response
                pub enum Response {
                    _200(Response200),
                }
                pub struct Response200_4 {
                    pub href: String,
                    pub rel: String,
                }
                pub struct Response200_2 {
                    pub status: String,
                    pub updated: String,
                    pub id: String,
                    pub links: Vec<Response200_4>,
                }
                pub struct Response200 {
                    pub versions: Vec<Response200_2>,
                }
            }
        }
      }
    };

    assert_token_streams_match(api(args, input), expected);
}
