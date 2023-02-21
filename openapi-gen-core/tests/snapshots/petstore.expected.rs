mod test {
    pub mod test_more {
        ///Test summary
        pub mod post {
            pub mod request {
                #[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
                pub struct QueryParams {
                    ///Test parameter
                    pub test: InnerParam,
                }
                #[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
                pub struct Headers {
                    ///Something passed as a header
                    pub RandomKey: String,
                }
                #[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
                pub struct PathParams {}
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
                    ///200 response
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
                    ///200 response
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
