use gqls_ir::Name;

use crate::{apply_changeset, point, range, Ide, Location};

#[test]
fn test_goto_definition() {
    let mut ide = Ide::default();
    let foo = ide.vfs.intern("foo.gql");
    let summary = apply_changeset!(ide: foo => r#"
type Foo {
    bar: Bar
}

type Bar {
    foo: Foo
}"#);
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
