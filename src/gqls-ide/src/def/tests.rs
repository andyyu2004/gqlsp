use crate::{change, point, Ide};

#[test]
fn test_goto_def() {
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
    assert!(ide.analysis().find_definition(foo, point!(0:0)).is_none());
    let range = ide.analysis().find_definition(foo, point!(1:5)).unwrap();
}
