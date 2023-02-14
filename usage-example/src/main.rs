use openapi_gen_macro::api;

#[api(path = "../openapi-gen-core/tests/simple_site.json")]
mod some_site {}

fn main() {
    let _ = some_site::test_more::post::request::Body {
        foo: "bar".to_string(),
        bar: "foo".to_string(),
    };
    let _ = some_site::test_more::post::request::QueryParams {
        test: some_site::test_more::post::request::InnerParam {
            foo: "bar".to_string(),
            bar: "foo".to_string(),
        },
    };
    let x = some_site::root::get::response::Body200 {
        versions: vec![some_site::root::get::response::Version {
            updated: "1".to_string(),
            id: "1".to_string(),
            status: "200".to_string(),
            links: vec![some_site::root::get::response::Link {
                href: "no".to_string(),
                rel: "me".to_string(),
            }],
        }],
    };
    let _ = some_site::root::get::response::Body::_200(x);

    println!("Hello, world!");
}
