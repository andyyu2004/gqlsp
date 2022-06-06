use expect_test::expect;
use gqls_fixture::fixture;

use super::{test_error_code, test_error_message, test_rendered};

#[test]
fn test_unresolved_directives() {
    let fixture = fixture! {
        "foo" => "
            type Foo @qux {
                    #....(E0002)
                bar: Int @qux
                        #....(E0002)
            }

            input FooInput @qux {
                          #....(E0002)
                bar: Int @qux
                        #....(E0002)
                    #...[E0002]
            }

            interface Bar @qux {
                         #....(E0002)
                bar: Int @qux
                        #....(E0002)
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
                     #...(unresolved type `Bar`)
            }
        "
    };
    test_error_message(&fixture);
}

#[test]
fn test_unresolved_type_in_argument() {
    let fixture = fixture! {
        "foo" => "
            type Foo {
                bar(bar: Bar!): [ID!]!
                        #...(unresolved type `Bar`)
            }
        "
    };
    test_error_message(&fixture);
}

#[test]
fn test_unresolved_interface_in_implements_clause() {
    test_rendered(
        "type Foo implements Bar { id: ID! }",
        expect![[r#"
        error[0003]: unresolved type `Bar`
          ┌─ test.graphql:1:21
          │
        1 │ type Foo implements Bar { id: ID! }
          │                     ^^^

    "#]],
    );
}
