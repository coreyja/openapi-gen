mod test {
    pub mod pets {
        ///List all pets
        pub mod get {
            pub mod request {
                use serde::{Serialize, Deserialize};
                #[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
                pub struct Error {
                    pub code: i32,
                    pub message: String,
                }
                #[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
                pub struct Pet {
                    pub id: i64,
                    pub name: String,
                    #[serde(default, skip_serializing_if = "Option::is_none")]
                    pub tag: Option<String>,
                }
                #[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
                pub struct Pets(pub Vec<Pet>);
                impl std::ops::Deref for Pets {
                    type Target = Vec<Pet>;
                    fn deref(&self) -> &Self::Target {
                        &self.0
                    }
                }
                #[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
                pub struct QueryParams {
                    ///How many items to return at one time (max 100)
                    pub limit: i32,
                }
                #[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
                pub struct Headers {}
                #[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
                pub struct PathParams {}
            }
            pub mod response {
                use serde::{Serialize, Deserialize};
                #[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
                pub struct Error {
                    pub code: i32,
                    pub message: String,
                }
                #[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
                pub struct Pet {
                    pub id: i64,
                    pub name: String,
                    #[serde(default, skip_serializing_if = "Option::is_none")]
                    pub tag: Option<String>,
                }
                #[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
                pub struct Pets(pub Vec<Pet>);
                impl std::ops::Deref for Pets {
                    type Target = Vec<Pet>;
                    fn deref(&self) -> &Self::Target {
                        &self.0
                    }
                }
                ///Test this Response
                #[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
                pub enum Body {
                    ///A paged array of pets
                    _200(Vec<self::Pet>),
                }
                #[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
                pub enum Headers {
                    _200(Headers200),
                }
                #[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
                pub struct Headers200 {
                    ///A link to the next page of responses
                    pub x_next: String,
                }
            }
        }
        ///Create a pet
        pub mod post {
            pub mod request {
                use serde::{Serialize, Deserialize};
                #[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
                pub struct Error {
                    pub code: i32,
                    pub message: String,
                }
                #[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
                pub struct Pet {
                    pub id: i64,
                    pub name: String,
                    #[serde(default, skip_serializing_if = "Option::is_none")]
                    pub tag: Option<String>,
                }
                #[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
                pub struct Pets(pub Vec<Pet>);
                impl std::ops::Deref for Pets {
                    type Target = Vec<Pet>;
                    fn deref(&self) -> &Self::Target {
                        &self.0
                    }
                }
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
                pub struct Error {
                    pub code: i32,
                    pub message: String,
                }
                #[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
                pub struct Pet {
                    pub id: i64,
                    pub name: String,
                    #[serde(default, skip_serializing_if = "Option::is_none")]
                    pub tag: Option<String>,
                }
                #[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
                pub struct Pets(pub Vec<Pet>);
                impl std::ops::Deref for Pets {
                    type Target = Vec<Pet>;
                    fn deref(&self) -> &Self::Target {
                        &self.0
                    }
                }
                ///Test this Response
                #[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
                pub enum Body {
                    ///Null response
                    _201(()),
                }
            }
        }
    }
    pub mod pets_petid {
        ///Info for a specific pet
        pub mod get {
            pub mod request {
                use serde::{Serialize, Deserialize};
                #[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
                pub struct Error {
                    pub code: i32,
                    pub message: String,
                }
                #[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
                pub struct Pet {
                    pub id: i64,
                    pub name: String,
                    #[serde(default, skip_serializing_if = "Option::is_none")]
                    pub tag: Option<String>,
                }
                #[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
                pub struct Pets(pub Vec<Pet>);
                impl std::ops::Deref for Pets {
                    type Target = Vec<Pet>;
                    fn deref(&self) -> &Self::Target {
                        &self.0
                    }
                }
                #[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
                pub struct QueryParams {}
                #[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
                pub struct Headers {}
                #[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
                pub struct PathParams {
                    ///The id of the pet to retrieve
                    pub petId: String,
                }
            }
            pub mod response {
                use serde::{Serialize, Deserialize};
                #[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
                pub struct Body200 {
                    pub id: i64,
                    pub name: String,
                    #[serde(default, skip_serializing_if = "Option::is_none")]
                    pub tag: Option<String>,
                }
                #[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
                pub struct Error {
                    pub code: i32,
                    pub message: String,
                }
                #[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
                pub struct Pet {
                    pub id: i64,
                    pub name: String,
                    #[serde(default, skip_serializing_if = "Option::is_none")]
                    pub tag: Option<String>,
                }
                #[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
                pub struct Pets(pub Vec<Pet>);
                impl std::ops::Deref for Pets {
                    type Target = Vec<Pet>;
                    fn deref(&self) -> &Self::Target {
                        &self.0
                    }
                }
                ///Test this Response
                #[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
                pub enum Body {
                    ///Expected response to a valid request
                    _200(self::Body200),
                }
            }
        }
    }
}
