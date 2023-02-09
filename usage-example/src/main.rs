use openapi_gen_macro::api;

#[api(path = "src/some_site.json")]
mod some_site {}

fn main() {
    let _ = some_site::Test_More::Get::QueryParams {
        test: "1".to_string(),
    };
    let _ = some_site::Root::Get::Request {};
    let x = some_site::Root::Get::Response200 {
        versions: vec![some_site::Root::Get::Response200_2 {
            updated: "1".to_string(),
            id: "1".to_string(),
            status: "200".to_string(),
            links: vec![some_site::Root::Get::Response200_4 {
                href: "no".to_string(),
                rel: "me".to_string(),
            }],
        }],
    };
    let _ = some_site::Root::Get::Response::_200(x);

    println!("Hello, world!");
}
