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
        assert_eq!(references.into_iter().collect::<HashSet<Location>>(), expected);
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
            directive @foo on FIELD_DEFINITION
                      #^^^

            type Foo {
                bar: Bar @foo
                         #...
            }

            type Bar {
                foo: Foo @qux
                         #...
            }

            interface Interface {
                foo: Foo @qux
                         #...
            }

            input Input {
                foo: Foo @qux
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
