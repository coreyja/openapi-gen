mod test {
    pub mod test_more {
        ///Test summary
        pub mod post {
            pub mod request {
                use serde::{Serialize, Deserialize};
                #[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
                pub struct Body {
                    #[serde(default, skip_serializing_if = "Option::is_none")]
                    pub bar: Option<String>,
                    ///Foo is to bar as red is to wine
                    #[serde(
                        rename = "Foo",
                        default,
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub foo: Option<String>,
                }
                #[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
                pub struct Headers {
                    #[serde(rename = "RandomKey")]
                    pub random_key: String,
                }
                #[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
                pub struct QueryParams {
                    pub test: QueryParamsTest,
                }
                #[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
                pub struct QueryParamsTest {
                    #[serde(default, skip_serializing_if = "Option::is_none")]
                    pub bar: Option<String>,
                    #[serde(default, skip_serializing_if = "Option::is_none")]
                    pub foo: Option<String>,
                }
            }
            pub mod response {
                use serde::{Serialize, Deserialize};
                #[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
                pub struct Body200 {
                    pub id: String,
                    pub name: String,
                }
                #[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
                pub struct DefaultBody {
                    #[serde(default, skip_serializing_if = "Option::is_none")]
                    pub code: Option<i32>,
                    #[serde(default, skip_serializing_if = "Option::is_none")]
                    pub message: Option<String>,
                }
                #[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
                pub struct Headers200 {
                    ///Test header
                    #[serde(
                        rename = "X-Test",
                        default,
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub x_test: Option<String>,
                }
                ///Test this Response
                #[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
                pub enum Body {
                    ///200 response
                    _200(self::Body200),
                    ///Default Error Response
                    Default(self::DefaultBody),
                }
                #[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
                pub enum Headers {
                    _200(self::Headers200),
                }
            }
        }
    }
    pub mod root {
        ///List API versions
        pub mod get {
            pub mod request {
                use serde::{Serialize, Deserialize};
            }
            pub mod response {
                use serde::{Serialize, Deserialize};
                #[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
                pub struct Body200 {
                    pub versions: Vec<Body200VersionsItem>,
                }
                #[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
                pub struct Body200VersionsItem {
                    pub id: String,
                    pub links: Vec<Body200VersionsItemLinksItem>,
                    pub status: String,
                    pub updated: String,
                }
                #[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
                pub struct Body200VersionsItemLinksItem {
                    pub href: String,
                    pub rel: String,
                }
                ///Test this Response
                #[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
                pub enum Body {
                    ///200 response
                    _200(self::Body200),
                }
            }
        }
    }
}
