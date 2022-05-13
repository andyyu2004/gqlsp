use std::collections::HashSet;

use gqls_fixture::{fixture, Fixture};

use crate::{Ide, Location};

#[track_caller]
fn test(fixture: Fixture) {
    let ide = Ide::from_fixture(&fixture);
    let expected = fixture
        .ranges()
        .map(|(file, range)| Location::new(file, range.into()))
        .collect::<HashSet<Location>>();

    for reference_position in fixture.positions() {
        let snapshot = ide.snapshot();
        let references = snapshot.find_references(reference_position);
        let actual = references.into_iter().collect::<HashSet<Location>>();

        assert_eq!(expected, actual);
    }
}

#[test]
fn test_find_references_to_object_like_type() {
    let foo = r#"
        extend type Foo {
                   #...
                   #^^^
            bar: Bar
        }

        type Bar {
            foo: Foo
               # ...
        }

        interface Interface {
            foo: Foo
                #...
        }

        input Input {
            foo: Foo
                #...
        }

        extend type Bar {
            k: Foo
              #...
        }"#;

    for kind in ["extend type", "type", "interface", "input"] {
        assert!(kind.len() <= "extend type".len());
        let padding = " ".repeat("extend type".len() - kind.len());
        let templated = foo.replace("extend type Foo", &format!("{padding}{kind} Foo"));
        let fixture = fixture! {
            "foo" => templated
            "baz" => r#"
                type Baz {
                    foo: Foo
                       # ...
                }
            "#
        };
        test(fixture);
    }
}

#[test]
fn test_find_references_when_cursor_on_reference() {
    let fixture = fixture! {
        "foo" => "
            scalar Scalar
                  #......

            type Foo {
                s: [Scalar!]!
                 # ^......^
                 #  ^^^^^^
            }
        "
    };
    test(fixture);
}

#[test]
fn test_find_references_to_enum() {
    let fixture = fixture! {
        "foo" => "
            enum Enum {
                #....
                #^^^^
                FOO
                BAR
            }

            type Foo {
                enum: Enum!
                     #....
            }
        "
    };
    test(fixture);
}

#[test]
fn test_find_references_to_scalar() {
    let fixture = fixture! {
        "foo" => "
            scalar Scalar
                  #......
                  #^^^^^^

            type Foo {
                s: Scalar!
                  #......
            }
        "
    };
    test(fixture);
}

#[test]
fn test_find_references_to_type_within_union() {
    let fixture = fixture! {
        "foo" => "
            scalar S
                  #.
                  #^
            scalar T


            union U = S | T
                     #.
        "
    };
    test(fixture);

    let fixture = fixture! {
        "foo" => "
            type Foo {
                #...
                #^^^
                t: T
            }
            scalar T


            union U = Foo | T
                     #...
        "
    };
    test(fixture);
}

#[test]
fn test_find_directive_references() {
    let fixture = fixture! {
        "foo" => r#"
            directive @qux on FIELD_DEFINITION
                     #....
                     #^^^^
                | OBJECT
                | SCALAR
                | UNION
                | ENUM
                | ENUM_VALUE
                | INPUT_OBJECT
                | INPUT_FIELD_DEFINITION

            scalar S @qux
                    #....

            union U @qux = Foo | Bar
                   #....

            enum E @qux {
                  #....
                FOO
                    # TODO enum value directives
                BAR
            }


            type Foo @qux {
                    #....
                bar: Bar @qux
                        #....
            }

            extend type Foo @qux {
                           #....
                baz: Baz @qux
                        #....
            }

            type Bar {
                foo: Foo @qux
                        #....
            }

            interface Interface @qux {
                               #....
                foo: Foo @qux
                        #....
            }

            input Input @qux {
                       #....
                foo: Foo @qux
                        #....
            }"#

        "baz" => "
            type Baz {
                foo: Foo @qux
                        #....
            }
            "
    };
    test(fixture);
}
