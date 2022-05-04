use expect_test::{expect, Expect};
use gqls_fixture::{fixture, Fixture};

use crate::Ide;

fn test(fixture: Fixture, expect: Expect) {
    let mut ide = Ide::default();
    ide.setup_fixture(&fixture);
    let snapshot = ide.snapshot();
    let symbols = snapshot.workspace_symbols();
    expect.assert_debug_eq(&symbols);
}

#[test]
fn test_workspace_symbols() {
    let fixture = fixture! {
        "foo" => "
            type Foo {
                baz: Int @qux
            }

            enum Bar {
                A
                B
                C
            }
        "

        "bar" => "
            directive @qux on FIELD_DEFINITION | OBJECT

            extend type Foo {
                qux: Int
            }
        "
    };
    test(
        fixture,
        expect![[r#"
            [
                qux :: Constant @ bar:1:12..1:55,
                Foo :: Struct @ bar:3:12..5:13,
                Foo :: Struct @ foo:1:12..3:13,
                Bar :: Struct @ foo:5:12..9:13,
            ]
        "#]],
    );
}
