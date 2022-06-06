use std::collections::HashMap;

use expect_test::{expect, Expect};
use gqls_fixture::{fixture, hashmap, Fixture};

use crate::Ide;

fn test(fixture: Fixture, expectations: HashMap<&'static str, Expect>) {
    let ide = Ide::from_fixture_allow_errors(&fixture);
    for (file, _) in fixture.files() {
        let tokens = ide.snapshot().semantic_tokens(file);
        expectations[file.to_str().unwrap()].assert_debug_eq(&tokens);
    }
}

#[test]
fn test_highlight_simple() {
    let fixture = fixture!("foo" => "
    type Foo {
        x: Int!
    }
    ");
    test(
        fixture,
        hashmap! {
            "foo" => expect![[r#"
            [
                1:4..1:8 :: Keyword,
                1:9..1:12 :: Object,
                2:8..2:9 :: Field,
                2:11..2:15 :: Type,
            ]
        "#]],
        },
    );
}

#[test]
fn test_highlight_scalars_and_enums() {
    let fixture = fixture!("foo" => "
    scalar Scalar

    enum E {
        A
        B
    }

    type Object {
        x: Object!
        s: Scalar
        list: [Scalar!]!
        enums: [E]
    }
    ");
    test(
        fixture,
        hashmap! {
            "foo" => expect![[r#"
                [
                    1:4..1:10 :: Keyword,
                    1:11..1:17 :: Scalar,
                    3:4..3:8 :: Keyword,
                    3:9..3:10 :: Enum,
                    4:8..4:9 :: EnumValue,
                    5:8..5:9 :: EnumValue,
                    8:4..8:8 :: Keyword,
                    8:9..8:15 :: Object,
                    9:8..9:9 :: Field,
                    9:11..9:18 :: Object,
                    10:8..10:9 :: Field,
                    10:11..10:17 :: Scalar,
                    11:8..11:12 :: Field,
                    11:14..11:24 :: Scalar,
                    12:8..12:13 :: Field,
                    12:15..12:18 :: Enum,
                ]
            "#]],
        },
    );
}

#[test]
fn test_highlight_union() {
    let fixture = fixture!("foo" => "
       scalar C
       union U = A | B | C
    ");
    test(
        fixture,
        hashmap! {
            "foo" => expect![[r#"
                [
                    1:7..1:13 :: Keyword,
                    1:14..1:15 :: Scalar,
                    2:7..2:12 :: Keyword,
                    2:13..2:14 :: Union,
                    2:17..2:18 :: Type,
                    2:21..2:22 :: Type,
                    2:25..2:26 :: Scalar,
                ]
            "#]],
        },
    );
}

#[test]
fn test_highlight_directive() {
    let fixture = fixture!("foo" => "
        directive @qux on FIELD_DEFINITION | OBJECT | INTERFACE | UNION | ENUM | ENUM_VALUE | INPUT_OBJECT | INPUT_FIELD_DEFINITION
        scalar Scalar @qux

        type Foo @qux {
            i: Scalar @qux
        }

        interface Iface @qux {
            i: Scalar @qux
        }

        enum E @qux {
            A @qux
            B @qux
        }

        union U @qux = Foo | SomethingUnresolved

        input I @qux {
            i: Scalar @qux
        }
    ");
    test(
        fixture,
        hashmap! {
            "foo" => expect![[r#"
                [
                    1:8..1:17 :: Keyword,
                    1:18..1:22 :: Directive,
                    1:23..1:25 :: Keyword,
                    2:8..2:14 :: Keyword,
                    2:15..2:21 :: Scalar,
                    2:22..2:26 :: Directive,
                    4:8..4:12 :: Keyword,
                    4:13..4:16 :: Object,
                    4:17..4:21 :: Directive,
                    5:12..5:13 :: Field,
                    5:15..5:21 :: Scalar,
                    5:22..5:26 :: Directive,
                    8:8..8:17 :: Keyword,
                    8:18..8:23 :: Interface,
                    8:24..8:28 :: Directive,
                    9:12..9:13 :: Field,
                    9:15..9:21 :: Scalar,
                    9:22..9:26 :: Directive,
                    12:8..12:12 :: Keyword,
                    12:13..12:14 :: Enum,
                    12:15..12:19 :: Directive,
                    13:12..13:13 :: EnumValue,
                    13:14..13:18 :: Directive,
                    14:12..14:13 :: EnumValue,
                    14:14..14:18 :: Directive,
                    17:8..17:13 :: Keyword,
                    17:14..17:15 :: Union,
                    17:16..17:20 :: Directive,
                    17:23..17:26 :: Object,
                    17:29..17:48 :: Type,
                    19:14..19:15 :: InputObject,
                    19:16..19:20 :: Directive,
                    20:12..20:13 :: Field,
                    20:15..20:21 :: Scalar,
                    20:22..20:26 :: Directive,
                ]
            "#]],
        },
    );
}

#[test]
fn test_highlight_fields() {
    let fixture = fixture!("foo" => "
    interface I {
        x: Int!
    }

    type Object {
        iface: I
    }

    input Input {
        x: Int!
    }

    ");
    test(
        fixture,
        hashmap! {
            "foo" => expect![[r#"
                [
                    1:4..1:13 :: Keyword,
                    1:14..1:15 :: Interface,
                    2:8..2:9 :: Field,
                    2:11..2:15 :: Type,
                    5:4..5:8 :: Keyword,
                    5:9..5:15 :: Object,
                    6:8..6:13 :: Field,
                    6:15..6:16 :: Interface,
                    9:10..9:15 :: InputObject,
                    10:8..10:9 :: Field,
                    10:11..10:15 :: Type,
                ]
            "#]]
        },
    );
}

#[test]
fn test_highlight_cross_file() {
    let fixture = fixture! {
        "bar" => "
            type Foo {
                a: A
                b: B
            }
        "

        "foo" => "
            scalar A
            enum B { B1 B2 }
        "
    };
    test(
        fixture,
        hashmap! {
            "bar" => expect![[r#"
                [
                    1:12..1:16 :: Keyword,
                    1:17..1:20 :: Object,
                    2:16..2:17 :: Field,
                    2:19..2:20 :: Scalar,
                    3:16..3:17 :: Field,
                    3:19..3:20 :: Enum,
                ]
            "#]],
            "foo" => expect![[r#"
                [
                    1:12..1:18 :: Keyword,
                    1:19..1:20 :: Scalar,
                    2:12..2:16 :: Keyword,
                    2:17..2:18 :: Enum,
                    2:21..2:23 :: EnumValue,
                    2:24..2:26 :: EnumValue,
                ]
            "#]]
        },
    );
}

#[test]
fn test_highlight_arguments() {
    let fixture = fixture! {
        "foo" => "
            type Foo {
                foo(
                    id: ID!
                    b: Boolean!
                ): Foo!
            }
        "
    };
    test(
        fixture,
        hashmap! {
            "foo" => expect![[r#"
                [
                    1:12..1:16 :: Keyword,
                    1:17..1:20 :: Object,
                    2:16..2:19 :: Field,
                    3:20..3:22 :: Argument,
                    3:24..3:27 :: Type,
                    4:20..4:21 :: Argument,
                    4:23..4:31 :: Type,
                    5:19..5:23 :: Object,
                ]
            "#]],
        },
    );
}
