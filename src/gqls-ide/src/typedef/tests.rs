use std::collections::HashSet;

use crate::{Ide, Location};
use gqls_fixture::{fixture, Fixture};

fn test(fixture: Fixture) {
    let mut ide = Ide::default();
    ide.setup_fixture(&fixture);
    let analysis = ide.analysis();
    let expected_locations = fixture
        .all_ranges()
        .map(|(file, range)| Location::new(file, range.into()))
        .collect::<HashSet<_>>();

    for (file, at) in fixture.all_points() {
        let locations = analysis.goto_type_definition(file, at).into_iter().collect::<HashSet<_>>();
        assert_eq!(expected_locations, locations);
    }
}

#[test]
fn test_goto_definition_cross_file() {
    let fixture = fixture!(
        "foo" => "
#{
type Foo {
    bar: Bar
}
#}
"
        "baz" => "
#{
extend type Foo {
    i: Int!
}
#}

type Bar {
    foo: Foo
    #^
}

type Baz {
    foo: Foo
     #^
}
            "
    );
    test(fixture);
}
