mod utils;
use utils::*;

#[test]
fn test_simple() {
    test_fixture_snapshot("simple_site.json");
}

#[test]
fn test_petshop() {
    test_fixture_snapshot("petstore.json");
}
