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

#[test]
fn test_lower_item_body_with_field_arg_defaults() {
    let fixture = fixture_file! {
        "
        extend type Foo {
            foo(a: Int!, b: Boolean! = false): Int @qux
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
                              foo(a: Int!, b: Boolean! = false): Int @qux
                            ],
                        },
                    ),
                },
            )
        "#]],
    );
}

#[test]
fn test_lower_simple_value() {
    let fixture = fixture_file! {
        r#"
        extend type Foo {
            int(i: Int! = 5): Int
            fls(b: Boolean! = false): Int
            t(b: Boolean! = true): Int
            n(b: Boolean = null): Int
            s(b: String = "test"): Int
        }
        "#
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
                              int(i: Int! = 5): Int
                              fls(b: Boolean! = false): Int
                              t(b: Boolean! = true): Int
                              n(b: Boolean = null): Int
                              s(b: String = "test"): Int
                            ],
                        },
                    ),
                },
            )
        "#]],
    );
}

#[test]
fn test_lower_value() {
    let fixture = fixture_file! {
        r#"
        extend type Foo {
            e(e: E = A): Int
            xs(xs: [Int]! = [1,2,3]): Int
            obj(input: Input! = { foo: 1, bar: false }): Int
        }

        enum E { A B }

        input Input {
            foo: Int
            bar: Bool!
        }
        "#
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
                              e(e: E = A): Int
                              xs(xs: [Int]! = [1, 2, 3]): Int
                              obj(input: Input! = { bar: false, foo: 1 }): Int
                            ],
                        },
                    ),
                },
            )
        "#]],
    );
}
