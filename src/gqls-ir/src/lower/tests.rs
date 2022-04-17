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
        foo: r#"type Foo {
            foo: Int
            list: [Int]
            nonNull: Int!
            nonNullList: [Int!]!
            a: [Int!]
            b: [Int]!
        }"#,
    });

    let body = db.item_body(ItemRes { file: foo, idx: idx!(0) });
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
                                        start_byte: 23,
                                        end_byte: 31,
                                        start_point: Point {
                                            row: 1,
                                            column: 12,
                                        },
                                        end_point: Point {
                                            row: 1,
                                            column: 20,
                                        },
                                    },
                                    name: foo,
                                    ty: Int,
                                },
                                Field {
                                    range: Range {
                                        start_byte: 44,
                                        end_byte: 55,
                                        start_point: Point {
                                            row: 2,
                                            column: 12,
                                        },
                                        end_point: Point {
                                            row: 2,
                                            column: 23,
                                        },
                                    },
                                    name: list,
                                    ty: [Int],
                                },
                                Field {
                                    range: Range {
                                        start_byte: 68,
                                        end_byte: 81,
                                        start_point: Point {
                                            row: 3,
                                            column: 12,
                                        },
                                        end_point: Point {
                                            row: 3,
                                            column: 25,
                                        },
                                    },
                                    name: nonNull,
                                    ty: Int!,
                                },
                                Field {
                                    range: Range {
                                        start_byte: 94,
                                        end_byte: 114,
                                        start_point: Point {
                                            row: 4,
                                            column: 12,
                                        },
                                        end_point: Point {
                                            row: 4,
                                            column: 32,
                                        },
                                    },
                                    name: nonNullList,
                                    ty: [Int!]!,
                                },
                                Field {
                                    range: Range {
                                        start_byte: 127,
                                        end_byte: 136,
                                        start_point: Point {
                                            row: 5,
                                            column: 12,
                                        },
                                        end_point: Point {
                                            row: 5,
                                            column: 21,
                                        },
                                    },
                                    name: a,
                                    ty: [Int!],
                                },
                                Field {
                                    range: Range {
                                        start_byte: 149,
                                        end_byte: 158,
                                        start_point: Point {
                                            row: 6,
                                            column: 12,
                                        },
                                        end_point: Point {
                                            row: 6,
                                            column: 21,
                                        },
                                    },
                                    name: b,
                                    ty: [Int]!,
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
