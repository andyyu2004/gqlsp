use expect_test::expect;
use vfs::Vfs;

use crate::tests::{idx, setup, TestDB};
use crate::DefDatabase;

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

    let body = db.item_body(foo, idx!(0));
    expect![[r#"
        TypeDefinition(
            TypeDefinitionBody {
                fields: Fields {
                    fields: Arena {
                        len: 6,
                        data: [
                            Field {
                                name: foo,
                                ty: Int,
                            },
                            Field {
                                name: list,
                                ty: [Int],
                            },
                            Field {
                                name: nonNull,
                                ty: Int!,
                            },
                            Field {
                                name: nonNullList,
                                ty: [Int!]!,
                            },
                            Field {
                                name: a,
                                ty: [Int!],
                            },
                            Field {
                                name: b,
                                ty: [Int]!,
                            },
                        ],
                    },
                },
            },
        )
    "#]]
    .assert_debug_eq(&body);
}
