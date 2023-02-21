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
                pub struct InnerParam {
                    #[serde(default, skip_serializing_if = "Option::is_none")]
                    pub bar: Option<String>,
                    #[serde(default, skip_serializing_if = "Option::is_none")]
                    pub foo: Option<String>,
                }
                #[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
                pub struct QueryParams {
                    ///Test parameter
                    pub test: self::InnerParam,
                }
                #[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
                pub struct Headers {
                    ///Something passed as a header
                    pub RandomKey: String,
                }
                #[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
                pub struct PathParams {}
            }
            pub mod response {
                use serde::{Serialize, Deserialize};
                #[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
                pub struct Body200 {
                    pub id: String,
                    pub name: String,
                }
                ///Test this Response
                #[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
                pub enum Body {
                    ///200 response
                    _200(self::Body200),
                }
                #[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
                pub enum Headers {
                    _200(Headers200),
                }
                #[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
                pub struct Headers200 {
                    ///Test header
                    pub x_test: String,
                }
            }
        }
    }
    pub mod root {
        ///List API versions
        pub mod get {
            pub mod request {
                use serde::{Serialize, Deserialize};
                #[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
                pub struct QueryParams {}
                #[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
                pub struct Headers {}
                #[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
                pub struct PathParams {}
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
