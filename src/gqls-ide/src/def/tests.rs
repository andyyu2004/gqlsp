use std::collections::HashSet;

use crate::tests::setup;
use crate::{point, range, Ide, Location};
use gqls_db::DefDatabase;
use gqls_fixture::{fixture, Fixture};
use gqls_ir::Name;

fn test(fixture: Fixture) {
    let mut ide = Ide::default();
    ide.setup_fixture(&fixture);
    let analysis = ide.analysis();
    let expected_locations = fixture
        .all_ranges()
        .map(|(file, range)| Location::new(file, range.into()))
        .collect::<HashSet<_>>();

    for (file, at) in fixture.all_points() {
        let locations = analysis.goto_definition(file, at).into_iter().collect::<HashSet<_>>();
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

    let analysis = ide.analysis();
    assert!(analysis.name_at(foo, point!(0:0)).is_none());

    assert!(analysis.name_at(foo, point!(1:0)).is_none());
    for j in 5..8 {
        assert_eq!(analysis.name_at(foo, point!(1: j)), Some(Name::new("Foo")));
        assert_eq!(analysis.name_at(foo, point!(1: j)), Some(Name::new("Foo")));
        assert_eq!(analysis.name_at(foo, point!(1: j)), Some(Name::new("Foo")));
    }
    assert!(analysis.name_at(foo, point!(1:8)).is_none());

    assert!(analysis.name_at(foo, point!(2:8)).is_none());
    for j in 9..12 {
        assert_eq!(analysis.name_at(foo, point!(2: j)), Some(Name::new("Bar")));
    }
    assert!(analysis.name_at(foo, point!(2:12)).is_none());

    assert!(analysis.goto_definition(foo, point!(0:0)).is_empty());
    assert_eq!(
        analysis.goto_definition(foo, point!(1:6)),
        vec![Location { file: foo, range: range!(1:0..3:1) }]
    );
}
