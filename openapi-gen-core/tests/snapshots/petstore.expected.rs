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
                #[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
                pub struct QueryParams {
                    #[serde(default, skip_serializing_if = "Option::is_none")]
                    pub limit: Option<i32>,
                }
            }
            pub mod response {
                use serde::{Serialize, Deserialize};
                #[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
                pub struct Error {
                    pub code: i32,
                    pub message: String,
                }
                #[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
                pub struct Headers200 {
                    ///A link to the next page of responses
                    #[serde(
                        rename = "x-next",
                        default,
                        skip_serializing_if = "Option::is_none"
                    )]
                    pub x_next: Option<String>,
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
                    _200(self::Pets),
                    ///unexpected error
                    Default(self::Error),
                }
                #[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
                pub enum Headers {
                    _200(self::Headers200),
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
                    ///unexpected error
                    Default(self::Error),
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
                pub struct PathParams {
                    #[serde(rename = "petId")]
                    pub pet_id: String,
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
                    ///Expected response to a valid request
                    _200(self::Pet),
                    ///unexpected error
                    Default(self::Error),
                }
            }
        }
    }
}
