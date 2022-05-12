use gqls_fixture::fixture;

use super::test;

#[test]
fn test_syntax_diagnostics() {
    let fixture = fixture! {
        "foo" => "
            type
           #....'E0001'
        "
    };
    test(&fixture);
}
