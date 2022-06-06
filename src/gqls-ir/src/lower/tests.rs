use expect_test::expect;
use gqls_fixture::fixture;
use testing::TestDatabaseExt;
use vfs::Vfs;

use crate::tests::{idx, TestDB};
use crate::{DefDatabase, ItemRes};

#[test]
fn test_lower_item_body() {
    let mut vfs = Vfs::default();
    let foo = vfs.intern("foo");
    let fixture = fixture! {
        foo => "
        directive @qux on FIELD_DEFINITION
        extend type Foo {
            foo: Int @qux
            list: [Int]
            nonNull: Int! @qux
            nonNullList: [Int!]! @qux
            a: [Int!]
            b: [Int]!
        }
        "
    };
    let db = TestDB::from_fixture(&fixture);

    let body = db.item_body(ItemRes::new(foo, idx!(1)));
    expect![[r#"
        Some(
            ItemBody {
                diagnostics: [],
                kind: ObjectTypeDefinition(
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
                                                name: @qux,
                                            },
                                        ],
                                        arguments: Arena {
                                            len: 0,
                                            data: [],
                                        },
                                        default: None,
                                    },
                                    Field {
                                        range: 4:12..4:23,
                                        name: list,
                                        ty: [Int],
                                        directives: [],
                                        arguments: Arena {
                                            len: 0,
                                            data: [],
                                        },
                                        default: None,
                                    },
                                    Field {
                                        range: 5:12..5:30,
                                        name: nonNull,
                                        ty: Int!,
                                        directives: [
                                            Directive {
                                                range: 5:26..5:30,
                                                name: @qux,
                                            },
                                        ],
                                        arguments: Arena {
                                            len: 0,
                                            data: [],
                                        },
                                        default: None,
                                    },
                                    Field {
                                        range: 6:12..6:37,
                                        name: nonNullList,
                                        ty: [Int!]!,
                                        directives: [
                                            Directive {
                                                range: 6:33..6:37,
                                                name: @qux,
                                            },
                                        ],
                                        arguments: Arena {
                                            len: 0,
                                            data: [],
                                        },
                                        default: None,
                                    },
                                    Field {
                                        range: 7:12..7:21,
                                        name: a,
                                        ty: [Int!],
                                        directives: [],
                                        arguments: Arena {
                                            len: 0,
                                            data: [],
                                        },
                                        default: None,
                                    },
                                    Field {
                                        range: 8:12..8:21,
                                        name: b,
                                        ty: [Int]!,
                                        directives: [],
                                        arguments: Arena {
                                            len: 0,
                                            data: [],
                                        },
                                        default: None,
                                    },
                                ],
                            },
                        },
                    },
                ),
            },
        )
    "#]]
    .assert_debug_eq(&body);
}
