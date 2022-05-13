use gqls_fixture::fixture;

use super::{test_error_code, test_error_message};

#[test]
fn test_syntax_diagnostics_by_code() {
    let fixture = fixture! {
        "foo" => "
            type
           #....'E0001'
        "
    };
    test_error_code(&fixture);
}

#[test]
fn test_syntax_diagnostics_by_message() {
    let fixture = fixture! {
        "foo" => "
            type
           #....'syntax error'
        "
    };
    test_error_message(&fixture);
}
