use gqls_fixture::fixture;

use super::test_error_code;

#[test]
fn test_unresolved_item_directive() {
    let fixture = fixture! {
        "foo" => "
            type Foo @qux {
                     #...'E0002'
                bar: Int
            }

            input FooInput @qux {
                           #...'E0002'
                bar: Int
            }

            interface Bar @qux {
                          #...'E0002'
                bar: Int
            }
        "
    };
    test_error_code(&fixture);
}
