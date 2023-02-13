use openapi_gen_macro::api;

#[api(path = "../openapi-gen-core/tests/simple_site.json")]
mod some_site {}

fn main() {
    let _ = some_site::test_more::get::request::QueryParams {
        test: some_site::test_more::get::request::InnerParam {
            foo: "bar".to_string(),
            bar: "foo".to_string(),
        },
    };
    let _ = some_site::root::get::request::Body {};
    let x = some_site::root::get::response::Body200 {
        versions: vec![some_site::root::get::response::Body200_2 {
            updated: "1".to_string(),
            id: "1".to_string(),
            status: "200".to_string(),
            links: vec![some_site::root::get::response::Body200_4 {
                href: "no".to_string(),
                rel: "me".to_string(),
            }],
        }],
    };
    let _ = some_site::root::get::response::Body::_200(x);

    println!("Hello, world!");
}
