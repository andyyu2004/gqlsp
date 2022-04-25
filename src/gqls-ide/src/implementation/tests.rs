use std::collections::HashSet;

use gqls_fixture::{fixture, Fixture};

use crate::{Ide, Location};

fn test(fixture: Fixture) {
    let mut ide = Ide::default();
    ide.setup_fixture(&fixture);
    let snapshot = ide.snapshot();
    let expected_locations = fixture
        .all_ranges()
        .map(|(file, range)| Location::new(file, range.into()))
        .collect::<HashSet<_>>();

    for (file, at) in fixture.all_points() {
        let locations = snapshot.goto_implementation(file, at).into_iter().collect::<HashSet<_>>();
        assert_eq!(expected_locations, locations);
    }
}

#[test]
fn test_find_implementations() {
    let fixture = fixture!(
        "foo" => "
interface Iface {
         #^^^^^
    i: Int
}

type Foo implements Iface {
    #...
    i: Int
}

extend type Foo implements Iface {
           #...
    k: Int
}"
    );
    test(fixture);
}
