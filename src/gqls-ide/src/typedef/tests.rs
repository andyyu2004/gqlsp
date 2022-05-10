use std::collections::HashSet;

use crate::{Ide, Location};
use gqls_fixture::{fixture, Fixture};

fn test(fixture: Fixture) {
    let mut ide = Ide::default();
    ide.setup_fixture(&fixture);
    let snapshot = ide.snapshot();
    let expected_locations = fixture
        .all_ranges()
        .map(|(file, range)| Location::new(file, range.into()))
        .collect::<HashSet<_>>();

    for position in fixture.all_points() {
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
