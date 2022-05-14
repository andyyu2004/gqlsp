use gqls_fixture::fixture;

use super::test_error_code;

#[test]
fn test_duplicate_directive_definition() {
    let fixture = fixture! {
        "foo" => "
            directive @qux on FIELD_DEFINITION
            directive @qux on INTERFACE
        "
    };
    test_error_code(&fixture);
}
