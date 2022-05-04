use expect_test::{expect, Expect};
use gqls_fixture::{fixture, Fixture};

use crate::Ide;

fn test(fixture: &Fixture, query: &str, expect: Expect) {
    let mut ide = Ide::default();
    ide.setup_fixture(fixture);
    let snapshot = ide.snapshot();
    let symbols = snapshot.workspace_symbols(query);
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
        &fixture,
        "",
        expect![[r#"
            [
                Bar :: Struct @ foo:5:12..9:13,
                Foo :: Struct @ foo:1:12..3:13,
                Foo :: Struct @ bar:3:12..5:13,
                qux :: Constant @ bar:1:12..1:55,
            ]
        "#]],
    );
}

#[test]
fn test_workspace_symbols_filtered() {
    let fixture = fixture! {
        "foo" => "
            scalar FooFighters
            scalar FooBars
        "

        "bar" => "
            scalar BarFighters
        "
    };
    test(
        &fixture,
        "foo",
        expect![[r#"
            [
                FooBars :: Struct @ foo:2:12..2:26,
                FooFighters :: Struct @ foo:1:12..1:30,
            ]
        "#]],
    );
    test(
        &fixture,
        "bar",
        expect![[r#"
            [
                BarFighters :: Struct @ bar:1:12..1:30,
                FooBars :: Struct @ foo:2:12..2:26,
            ]
        "#]],
    );
}
