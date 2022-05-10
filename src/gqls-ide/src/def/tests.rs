use std::collections::HashSet;

use crate::tests::setup;
use crate::{point, position, range, Ide, Location};
use gqls_db::DefDatabase;
use gqls_fixture::{fixture, Fixture};
use gqls_ir::Name;

fn test(fixture: Fixture) {
    let mut ide = Ide::default();
    ide.setup_fixture(&fixture);
    let snapshot = ide.snapshot();
    let expected_locations = fixture
        .all_ranges()
        .map(|(file, range)| Location::new(file, range.into()))
        .collect::<HashSet<_>>();

    for position in fixture.all_points() {
        let locations = snapshot.goto_definition(position).into_iter().collect::<HashSet<_>>();
        assert_eq!(expected_locations, locations);
    }
}

#[test]
fn test_goto_definition_cross_file() {
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
    foo: Foo
        #^^^
}

type Baz {
    foo: Foo!
        #^^^^
}

type Qux {
    foo: [Foo!]!
        #^^^^^^^
}
            "
    );
    test(fixture);
}

#[test]
fn test_goto_definition() {
    let mut ide = Ide::default();
    let foo = ide.vfs.intern("foo.graphql");
    let summary = setup!(ide: {
               foo: r#"
type Foo {
    bar: Bar
}

type Bar {
    foo: Foo
}"#,
    });

    assert!(summary[foo].diagnostics.is_empty());

    let snapshot = ide.snapshot();
    assert!(snapshot.name_at(position!(foo:0:0)).is_none());

    assert!(snapshot.name_at(position!(foo:1:0)).is_none());
    for j in 5..8 {
        assert_eq!(snapshot.name_at(position!(foo:1:j)), Some(Name::unranged("Foo")));
        assert_eq!(snapshot.name_at(position!(foo:1:j)), Some(Name::unranged("Foo")));
        assert_eq!(snapshot.name_at(position!(foo:1:j)), Some(Name::unranged("Foo")));
    }
    assert!(snapshot.name_at(position!(foo:1:8)).is_none());

    assert!(snapshot.name_at(position!(foo:2:8)).is_none());
    for j in 9..12 {
        assert_eq!(snapshot.name_at(position!(foo:2:j)), Some(Name::unranged("Bar")));
    }
    assert!(snapshot.name_at(position!(foo:2:12)).is_none());

    assert!(snapshot.goto_definition(position!(foo:0:0)).is_empty());
    assert_eq!(
        vec![Location { file: foo, range: range!(1:5..1:8) }],
        snapshot.goto_definition(position!(foo:1:6)),
    );
}
