use expect_test::{expect, Expect};
use gqls_fixture::{fixture, Fixture};

use crate::Ide;

fn test(fixture: Fixture, file: &'static str, expect: Expect) {
    let mut ide = Ide::default();
    ide.setup_fixture(&fixture);
    let snapshot = ide.snapshot();
    let symbols = snapshot.document_symbols(file.as_ref());
    expect.assert_debug_eq(&symbols);
}

#[test]
fn test_document_symbols() {
    let fixture = fixture! {
        "foo" => "
            type Foo {
                bar: Bar
                baz: Int
            }

            type Bar {
                foo: Foo
            }

            interface Interface {
                foo: Foo
            }

            input Input {
                i: Int
            }

            extend type Extension {
                foo: Foo
            }
            "
    };

    test(
        fixture,
        "foo",
        expect![[r#"
            [
                Foo :: Struct @ 1:12..4:13
                  bar :: Field @ 2:16..2:24 (Bar)
                  baz :: Field @ 3:16..3:24 (Int),
                Bar :: Struct @ 6:12..8:13
                  foo :: Field @ 7:16..7:24 (Foo),
                Interface :: Struct @ 10:12..12:13
                  foo :: Field @ 11:16..11:24 (Foo),
                Input :: Struct @ 14:12..16:13
                  i :: Field @ 15:16..15:22 (Int),
                Extension :: Struct @ 18:12..20:13
                  foo :: Field @ 19:16..19:24 (Foo),
            ]
        "#]],
    );
}
