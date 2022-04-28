use std::path::Path;

use expect_test::{expect, Expect};
use gqls_fixture::{fixture, Fixture};

use crate::Ide;

fn test(fixture: Fixture, file: &'static str, expect: Expect) {
    let mut ide = Ide::default();
    ide.setup_fixture(&fixture);
    let snapshot = ide.snapshot();
    let symbols = snapshot.document_symbols(Path::new(file));
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
                Symbol {
                    name: Foo,
                    kind: Struct,
                    range: (1, 12)..(4, 13),
                    children: [
                        Symbol {
                            name: bar,
                            kind: Field,
                            range: (2, 16)..(2, 24),
                            children: [],
                            detail: Some(
                                "Bar",
                            ),
                        },
                        Symbol {
                            name: baz,
                            kind: Field,
                            range: (3, 16)..(3, 24),
                            children: [],
                            detail: Some(
                                "Int",
                            ),
                        },
                    ],
                    detail: None,
                },
                Symbol {
                    name: Bar,
                    kind: Struct,
                    range: (6, 12)..(8, 13),
                    children: [
                        Symbol {
                            name: foo,
                            kind: Field,
                            range: (7, 16)..(7, 24),
                            children: [],
                            detail: Some(
                                "Foo",
                            ),
                        },
                    ],
                    detail: None,
                },
                Symbol {
                    name: Interface,
                    kind: Struct,
                    range: (10, 12)..(12, 13),
                    children: [
                        Symbol {
                            name: foo,
                            kind: Field,
                            range: (11, 16)..(11, 24),
                            children: [],
                            detail: Some(
                                "Foo",
                            ),
                        },
                    ],
                    detail: None,
                },
                Symbol {
                    name: Input,
                    kind: Struct,
                    range: (14, 12)..(16, 13),
                    children: [
                        Symbol {
                            name: i,
                            kind: Field,
                            range: (15, 16)..(15, 22),
                            children: [],
                            detail: Some(
                                "Int",
                            ),
                        },
                    ],
                    detail: None,
                },
                Symbol {
                    name: Extension,
                    kind: Struct,
                    range: (18, 12)..(20, 13),
                    children: [
                        Symbol {
                            name: foo,
                            kind: Field,
                            range: (19, 16)..(19, 24),
                            children: [],
                            detail: Some(
                                "Foo",
                            ),
                        },
                    ],
                    detail: None,
                },
            ]
        "#]],
    );
}
