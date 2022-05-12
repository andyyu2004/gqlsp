use std::collections::HashSet;

use crate::{Ide, Location};
use gqls_fixture::{fixture, Fixture};

fn test(fixture: Fixture) {
    let ide = Ide::from_fixture(&fixture);
    let snapshot = ide.snapshot();
    let expected_locations = fixture
        .ranges()
        .map(|(file, range)| Location::new(file, range.into()))
        .collect::<HashSet<_>>();

    for position in fixture.positions() {
        let locations = snapshot.goto_type_definition(position).into_iter().collect::<HashSet<_>>();
        assert_eq!(expected_locations, locations);
    }
}

#[test]
fn test_goto_type_definition() {
    let fixture = fixture!(
        "foo" => "
type Foo {
    #...
    bar: Bar
}
"
        "baz" => "
extend type Foo {
           #...
    i: Int!
}

type Bar {
    foo: [Foo]!
   #^^^^^^^^^^^
}

extend type Bar {
    f: Foo!
   #^^^^^^^
}
            "
    );
    test(fixture);
}

#[test]
fn test_goto_type_definition_fallback() {
    let fixture = fixture! {
        "foo" => "
            type Foo {
                #...
                bar: Int
            }

            union U = Foo
                     #^^^
        "
    };

    test(fixture);
}
