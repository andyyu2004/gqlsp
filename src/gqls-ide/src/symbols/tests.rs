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
                    range: Range {
                        start: Point {
                            row: 1,
                            column: 12,
                        },
                        end: Point {
                            row: 4,
                            column: 13,
                        },
                    },
                    children: [
                        Symbol {
                            name: bar,
                            kind: Field,
                            range: Range {
                                start: Point {
                                    row: 2,
                                    column: 16,
                                },
                                end: Point {
                                    row: 2,
                                    column: 24,
                                },
                            },
                            children: [],
                            detail: Some(
                                "Bar",
                            ),
                        },
                        Symbol {
                            name: baz,
                            kind: Field,
                            range: Range {
                                start: Point {
                                    row: 3,
                                    column: 16,
                                },
                                end: Point {
                                    row: 3,
                                    column: 24,
                                },
                            },
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
                    range: Range {
                        start: Point {
                            row: 6,
                            column: 12,
                        },
                        end: Point {
                            row: 8,
                            column: 13,
                        },
                    },
                    children: [
                        Symbol {
                            name: foo,
                            kind: Field,
                            range: Range {
                                start: Point {
                                    row: 7,
                                    column: 16,
                                },
                                end: Point {
                                    row: 7,
                                    column: 24,
                                },
                            },
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
                    range: Range {
                        start: Point {
                            row: 10,
                            column: 12,
                        },
                        end: Point {
                            row: 12,
                            column: 13,
                        },
                    },
                    children: [
                        Symbol {
                            name: foo,
                            kind: Field,
                            range: Range {
                                start: Point {
                                    row: 11,
                                    column: 16,
                                },
                                end: Point {
                                    row: 11,
                                    column: 24,
                                },
                            },
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
                    range: Range {
                        start: Point {
                            row: 14,
                            column: 12,
                        },
                        end: Point {
                            row: 16,
                            column: 13,
                        },
                    },
                    children: [
                        Symbol {
                            name: i,
                            kind: Field,
                            range: Range {
                                start: Point {
                                    row: 15,
                                    column: 16,
                                },
                                end: Point {
                                    row: 15,
                                    column: 22,
                                },
                            },
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
                    range: Range {
                        start: Point {
                            row: 18,
                            column: 12,
                        },
                        end: Point {
                            row: 20,
                            column: 13,
                        },
                    },
                    children: [
                        Symbol {
                            name: foo,
                            kind: Field,
                            range: Range {
                                start: Point {
                                    row: 19,
                                    column: 16,
                                },
                                end: Point {
                                    row: 19,
                                    column: 24,
                                },
                            },
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
