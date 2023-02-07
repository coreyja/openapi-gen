use r#macro::api;

#[api(path = "src/some_site.json")]
mod some_site {
    /// This is a test
    pub struct Old {}
}

// #[api(path = "some_site")]
// mod test;

use some_site::Root::Get as _;
use some_site::Test_More::Get::Response as _;

fn main() {
    let _ = some_site::Old {};

    let _ = some_site::Root::Get::Request {};
    let x = some_site::Root::Get::Response200 {
        versions: some_site::Root::Get::Response200_1 {
            updated: "1".to_string(),
            id: "1".to_string(),
            status: "200".to_string(),
            links: some_site::Root::Get::Response200_2 {
                href: "no".to_string(),
                rel: "me".to_string(),
            },
        },
    };
    let _ = some_site::Root::Get::Response::_200(x);

    println!("Hello, world!");
}
