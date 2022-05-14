use std::collections::HashSet;

use gqls_fixture::{fixture, Fixture};

use crate::{Ide, Location};

fn test(fixture: Fixture) {
    let ide = Ide::from_fixture(&fixture);
    let snapshot = ide.snapshot();
    let expected_locations =
        fixture.ranges().map(|(file, range)| Location::new(file, range)).collect::<HashSet<_>>();

    for position in fixture.positions() {
        let locations = snapshot.goto_implementation(position).into_iter().collect::<HashSet<_>>();
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
