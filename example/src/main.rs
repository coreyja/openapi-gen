use r#macro::api;

#[api(path = "src/some_site.json")]
mod some_site {
    pub struct Old {}
}

// #[api(path = "some_site")]
// mod test;

use some_site::Root::Get as _;
use some_site::Test_More as _;

fn main() {
    let _ = some_site::Old {};

    println!("Hello, world!");
}
