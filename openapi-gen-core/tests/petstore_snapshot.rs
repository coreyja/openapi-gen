use openapi_gen_core::{api, MacroArgs};

mod utils;
use syn::parse_quote;
use utils::*;

#[test]
fn test_snapshot() {
    let args = MacroArgs {
        path: "../fixtures/petstore.json".to_string(),
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
