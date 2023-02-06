use r#macro::api;

#[api]
mod some_site {
    pub struct Old {}
}

#[api]
mod test;

fn main() {
    let _ = some_site::Old {};
    let _ = some_site::New {};
    println!("Hello, world!");
}
