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
    assert!(analysis.goto_definition(foo, point!(0:0)).is_empty());
    assert!(analysis.goto_definition(foo, point!(0:0)).is_empty());
    // dbg!(ide.analysis().goto_definition(foo, point!(1:5)).is_empty();
}
