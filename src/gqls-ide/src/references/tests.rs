use crate::tests::setup;
use crate::Ide;

#[test]
fn test_find_references() {
    let mut ide = Ide::default();
    let foo = ide.vfs.intern("foo.gql");
    setup!(ide: {
        foo: r#"
            type Foo {
                bar: Bar
            }

            type Bar {
                foo: Foo
            }
        "#,
    });
}
