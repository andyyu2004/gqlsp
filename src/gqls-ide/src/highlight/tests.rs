use std::collections::HashMap;

use expect_test::{expect, Expect};
use gqls_fixture::{fixture, hashmap, Fixture};

use crate::Ide;

fn test(fixture: Fixture, expectations: HashMap<&'static str, Expect>) {
    let mut ide = Ide::default();
    ide.setup_fixture(&fixture);
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
fn test_highlight_directive() {
    let fixture = fixture!("foo" => "
        scalar Scalar @qux

        type Foo @qux {
            i: Scalar @qux
        }

        interface Foo @qux {
            i: Scalar @qux
        }

        enum E @qux {
            A @qux
            B @qux
        }

        union U @qux = Foo | SomethingUnresolved
    ");
    test(
        fixture,
        hashmap! {
            "foo" => expect![[r#"
                [
                    1:8..1:14 :: Keyword,
                    1:15..1:21 :: Scalar,
                    1:22..1:26 :: Directive,
                    3:8..3:12 :: Keyword,
                    3:13..3:16 :: Object,
                    3:17..3:21 :: Directive,
                    4:12..4:13 :: Field,
                    4:15..4:21 :: Scalar,
                    4:22..4:26 :: Directive,
                    7:8..7:17 :: Keyword,
                    7:18..7:21 :: Interface,
                    7:22..7:26 :: Directive,
                    8:12..8:13 :: Field,
                    8:15..8:21 :: Scalar,
                    8:22..8:26 :: Directive,
                    11:13..11:14 :: Enum,
                    11:15..11:19 :: Directive,
                    12:12..12:13 :: EnumValue,
                    12:14..12:18 :: Directive,
                    13:12..13:13 :: EnumValue,
                    13:14..13:18 :: Directive,
                    16:8..16:13 :: Keyword,
                    16:14..16:15 :: Union,
                    16:16..16:20 :: Directive,
                    16:23..16:26 :: Object,
                    16:29..16:48 :: Type,
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
