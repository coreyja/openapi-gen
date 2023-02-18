use openapi_gen_core::{api, MacroArgs};

mod utils;
use syn::parse_quote;
use utils::*;

#[test]
fn test_petstore_snapshot() {
    let args = MacroArgs {
        path: "../fixtures/petstore.json".to_string(),
    };
    let input = quote::quote! {
        mod test {}
    };

    let expected = parse_quote! {
      mod test {
        pub mod pets {
            pub mod get {
                pub mod request {
                    #[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
                    pub struct QueryParams {
                        pub limit: i64,
                    }
                    #[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
                    pub struct Headers {}
                    #[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
                    pub struct PathParams {}
                }
                pub mod response {
                    ///Test this Response
                    #[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
                    pub enum Body {
                        _200(Vec<Body200>),
                    }
                    #[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
                    pub enum Headers {
                        _200(Headers200),
                    }
                    #[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
                    pub struct Body200 {
                        pub id: i64,
                        pub name: String,
                        pub tag: String,
                    }
                    #[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
                    pub struct Headers200 {
                        pub x_next: String,
                    }
                }
            }
            pub mod post {
                pub mod request {
                    #[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
                    pub struct QueryParams {}
                    #[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
                    pub struct Headers {}
                    #[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
                    pub struct PathParams {}
                }
                pub mod response {
                    ///Test this Response
                    #[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
                    pub enum Body {
                        _201(()),
                    }
                }
            }
        }
        pub mod pets_petid {
            pub mod get {
                pub mod request {
                    #[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
                    pub struct QueryParams {}
                    #[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
                    pub struct Headers {}
                    #[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
                    pub struct PathParams {
                        pub petId: String,
                    }
                }
                pub mod response {
                    ///Test this Response
                    #[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
                    pub enum Body {
                        _200(Body200),
                    }
                    #[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
                    pub struct Body200 {
                        pub id: i64,
                        pub name: String,
                        pub tag: String,
                    }
                }
            }
        }
      }
    };

    assert_token_streams_match(api(args, input), expected);
}
