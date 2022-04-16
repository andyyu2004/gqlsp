use std::collections::HashSet;

use gqls_fixture::{fixture, Fixture};

use crate::{Ide, Location};

fn test(fixture: Fixture) {
    let mut ide = Ide::default();
    ide.setup_fixture(&fixture);

    let (reference_file, at) = fixture.sole_point();
    let analysis = ide.analysis();
    let references = analysis.find_references(reference_file, at);
    let expected = fixture
        .all_ranges()
        .map(|(file, range)| Location::new(file, range.into()))
        .collect::<HashSet<Location>>();
    assert_eq!(references.into_iter().collect::<HashSet<Location>>(), expected);
}

#[test]
fn test_find_references() {
    let fixture = fixture! {
        "foo" => r#"
            type Foo {
                 #^
                bar: Bar
            }

            type Bar {
                foo: Foo
                   # ...
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
