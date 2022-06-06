use expect_test::{expect, Expect};
use gqls_fixture::{fixture_file, FixtureFile};
use testing::{file_id, TestDatabaseExt};

use crate::tests::{idx, TestDB};
use crate::{DefDatabase, ItemRes};

fn test(fixture: &FixtureFile, expect: Expect) {
    let db = TestDB::from_fixture_file(&fixture);
    let body = db.item_body(ItemRes::new(file_id!(), idx!(0)));
    expect.assert_debug_eq(&body);
}

#[test]
fn test_lower_item_body() {
    let fixture = fixture_file! {
        "
        extend type Foo {
            foo: Int @qux
            list: [Int]
            nonNull: Int! @qux
            nonNullList: [Int!]! @qux
            a: [Int!]
            b: [Int]!
        }
        directive @qux on FIELD_DEFINITION
        "
    };

    test(
        &fixture,
        expect![[r#"
        Some(
            ItemBody {
                diagnostics: [],
                kind: ObjectTypeDefinition(
                    TypeDefinitionBody {
                        fields: [
                          foo: Int @qux
                          list: [Int]
                          nonNull: Int! @qux
                          nonNullList: [Int!]! @qux
                          a: [Int!]
                          b: [Int]!
                        ],
                    },
                ),
            },
        )
    "#]],
    );
}

#[test]
fn test_lower_item_body_with_field_args() {
    let fixture = fixture_file! {
        "
        extend type Foo {
            foo(a: Int!, b: Boolean!): Int @qux
        }
        directive @qux on FIELD_DEFINITION
        "
    };

    test(
        &fixture,
        expect![[r#"
            Some(
                ItemBody {
                    diagnostics: [],
                    kind: ObjectTypeDefinition(
                        TypeDefinitionBody {
                            fields: [
                              foo(a: Int!, b: Boolean!): Int @qux
                            ],
                        },
                    ),
                },
            )
        "#]],
    );
}
