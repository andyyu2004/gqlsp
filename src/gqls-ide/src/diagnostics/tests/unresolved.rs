use gqls_fixture::fixture;

use super::{test_error_code, test_error_message};

#[test]
fn test_unresolved_directives() {
    let fixture = fixture! {
        "foo" => "
            type Foo @qux {
                    #....'E0002'
                bar: Int @qux
                        #....'E0002'
            }

            input FooInput @qux {
                          #....'E0002'
                bar: Int @qux
                        #....'E0002'
                    #...[E0002]
            }

            interface Bar @qux {
                         #....'E0002'
                bar: Int @qux
                        #....'E0002'
            }
        "
    };
    test_error_code(&fixture);
}

#[test]
fn test_unresolved_type_in_field() {
    let fixture = fixture! {
        "foo" => "
            type Foo {
                bar: [Bar!]
                    #......'unresolved type `Bar`'
            }
        "
    };
    test_error_message(&fixture);
}
