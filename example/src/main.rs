use r#macro::api;

#[api(path = "src/some_site.json")]
mod some_site {
    pub struct Old {}
}

// #[api(path = "some_site")]
// mod test;

fn main() {
    let _ = some_site::Old {};
    let _ = some_site::New {};
    let _ = some_site::ListVersionsv2 {};
    let _ = some_site::TestOperation {};
    println!("Hello, world!");
}
