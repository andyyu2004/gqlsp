use expect_test::expect;
use vfs::Vfs;

use crate::tests::{idx, setup, TestDB};
use crate::{DefDatabase, ItemRes};

#[test]
fn test_lower_item_body() {
    let mut db = TestDB::default();
    let mut vfs = Vfs::default();
    let foo = vfs.intern("foo");
    setup!(db: {
        foo: r#"
        directive @qux on FIELD_DEFINITION
        type Foo {
            foo: Int @qux
            list: [Int]
            nonNull: Int! @qux
            nonNullList: [Int!]! @qux
            a: [Int!]
            b: [Int]!
        }"#,
    });

    let body = db.item_body(ItemRes { file: foo, idx: idx!(1) });
    expect![[r#"
        Some(
            ObjectTypeDefinition(
                TypeDefinitionBody {
                    fields: Fields {
                        fields: Arena {
                            len: 6,
                            data: [
                                Field {
                                    range: Range {
                                        start_byte: 75,
                                        end_byte: 88,
                                        start_point: Point {
                                            row: 3,
                                            column: 12,
                                        },
                                        end_point: Point {
                                            row: 3,
                                            column: 25,
                                        },
                                    },
                                    name: foo,
                                    ty: Int,
                                    directives: [
                                        Directive {
                                            name: qux,
                                        },
                                    ],
                                },
                                Field {
                                    range: Range {
                                        start_byte: 101,
                                        end_byte: 112,
                                        start_point: Point {
                                            row: 4,
                                            column: 12,
                                        },
                                        end_point: Point {
                                            row: 4,
                                            column: 23,
                                        },
                                    },
                                    name: list,
                                    ty: [Int],
                                    directives: [],
                                },
                                Field {
                                    range: Range {
                                        start_byte: 125,
                                        end_byte: 143,
                                        start_point: Point {
                                            row: 5,
                                            column: 12,
                                        },
                                        end_point: Point {
                                            row: 5,
                                            column: 30,
                                        },
                                    },
                                    name: nonNull,
                                    ty: Int!,
                                    directives: [
                                        Directive {
                                            name: qux,
                                        },
                                    ],
                                },
                                Field {
                                    range: Range {
                                        start_byte: 156,
                                        end_byte: 181,
                                        start_point: Point {
                                            row: 6,
                                            column: 12,
                                        },
                                        end_point: Point {
                                            row: 6,
                                            column: 37,
                                        },
                                    },
                                    name: nonNullList,
                                    ty: [Int!]!,
                                    directives: [
                                        Directive {
                                            name: qux,
                                        },
                                    ],
                                },
                                Field {
                                    range: Range {
                                        start_byte: 194,
                                        end_byte: 203,
                                        start_point: Point {
                                            row: 7,
                                            column: 12,
                                        },
                                        end_point: Point {
                                            row: 7,
                                            column: 21,
                                        },
                                    },
                                    name: a,
                                    ty: [Int!],
                                    directives: [],
                                },
                                Field {
                                    range: Range {
                                        start_byte: 216,
                                        end_byte: 225,
                                        start_point: Point {
                                            row: 8,
                                            column: 12,
                                        },
                                        end_point: Point {
                                            row: 8,
                                            column: 21,
                                        },
                                    },
                                    name: b,
                                    ty: [Int]!,
                                    directives: [],
                                },
                            ],
                        },
                    },
                },
            ),
        )
    "#]]
    .assert_debug_eq(&body);
}
