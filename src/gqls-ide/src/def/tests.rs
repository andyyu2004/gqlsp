use gqls_ir::Name;

use crate::{change, point, Ide};

#[test]
fn test_goto_definition() {
    let mut ide = Ide::default();
    let foo = ide.vfs.intern("foo.gql");
    let summary = change!(ide: foo => r#"
type Foo {
    bar: Bar
}

type Bar {
    foo: Foo
}"#);
    assert!(summary.diagnostics.is_empty());

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
    assert!(analysis.goto_definition(foo, point!(0:0)).is_empty());
    // dbg!(ide.analysis().goto_definition(foo, point!(1:5)).is_empty();
}
