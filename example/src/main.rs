use r#macro::api;

#[api(path = "src/some_site.json")]
mod some_site {
    pub struct Old {}
}

// #[api(path = "some_site")]
// mod test;

use some_site::Root::Get as _;
use some_site::Test_More::Get::Response as _;

fn main() {
    let _ = some_site::Old {};

    let _ = some_site::Root::Get::Request {};
    let x = some_site::Root::Get::Response200 { a: "".into() };
    let _ = some_site::Root::Get::Response::_200(x);

    println!("Hello, world!");
}
