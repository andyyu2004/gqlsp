use std::collections::HashSet;

use gqls_fixture::{fixture, Fixture};

use crate::{Ide, Location};

fn test(fixture: Fixture) {
    let mut ide = Ide::default();
    ide.setup_fixture(&fixture);

    for (reference_file, at) in fixture.all_points() {
        let analysis = ide.analysis();
        let references = analysis.find_references(reference_file, at);
        let expected = fixture
            .all_ranges()
            .map(|(file, range)| Location::new(file, range.into()))
            .collect::<HashSet<Location>>();
        let actual = references.into_iter().collect::<HashSet<Location>>();

        assert_eq!(expected, actual);
    }
}

#[test]
fn test_find_references() {
    let fixture = fixture! {
        "foo" => r#"
            type Foo {
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
            "#

        "baz" => r#"
            type Baz {
                foo: Foo
                   # ...
            }
            "#
    };
    test(fixture);
}

#[test]
fn test_find_directive_references() {
    let fixture = fixture! {
        "foo" => r#"
            directive @qux on FIELD_DEFINITION
                      #^^^
                | OBJECT
                | SCALAR
                | UNION
                | ENUM
                | ENUM_VALUE
                | INPUT_OBJECT
                | INPUT_FIELD_DEFINITION

            # TODO scalar enum union

            type Foo @qux {
                    #....
                bar: Bar @qux
                        #....
            }

        # TODO directive on interface types inputs extend types etc
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
