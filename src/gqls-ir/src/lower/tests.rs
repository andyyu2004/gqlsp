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
        extend type Foo {
            foo: Int @qux
            list: [Int]
            nonNull: Int! @qux
            nonNullList: [Int!]! @qux
            a: [Int!]
            b: [Int]!
        }
        "#,

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
                                    range: 3:12..3:25,
                                    name: foo,
                                    ty: Int,
                                    directives: [
                                        Directive {
                                            range: 3:21..3:25,
                                            name: qux,
                                        },
                                    ],
                                },
                                Field {
                                    range: 4:12..4:23,
                                    name: list,
                                    ty: [Int],
                                    directives: [],
                                },
                                Field {
                                    range: 5:12..5:30,
                                    name: nonNull,
                                    ty: Int!,
                                    directives: [
                                        Directive {
                                            range: 5:26..5:30,
                                            name: qux,
                                        },
                                    ],
                                },
                                Field {
                                    range: 6:12..6:37,
                                    name: nonNullList,
                                    ty: [Int!]!,
                                    directives: [
                                        Directive {
                                            range: 6:33..6:37,
                                            name: qux,
                                        },
                                    ],
                                },
                                Field {
                                    range: 7:12..7:21,
                                    name: a,
                                    ty: [Int!],
                                    directives: [],
                                },
                                Field {
                                    range: 8:12..8:21,
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
