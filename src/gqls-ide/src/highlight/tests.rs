use expect_test::{expect, Expect};
use gqls_fixture::{fixture, Fixture};

use crate::Ide;

fn test(fixture: Fixture, expect: Expect) {
    let mut ide = Ide::default();
    ide.setup_fixture(&fixture);
    let tokens = ide.snapshot().semantic_tokens(std::path::Path::new(""));
    expect.assert_debug_eq(&tokens);
}

#[test]
fn test_highlight_simple() {
    let fixture = fixture!("" => "
    type Foo {
        x: Int!
    }
    ");
    test(
        fixture,
        expect![[r#"
            [
                1:4..1:8 :: Keyword,
                1:9..1:12 :: Object,
                2:8..2:9 :: Field,
                2:11..2:15 :: Type,
            ]
        "#]],
    );
}

#[test]
fn test_highlight_scalars_and_enums() {
    let fixture = fixture!("" => "
    scalar scalar

    enum e {
        a
        b
    }

    type object {
        x: object!
        s: scalar
        list: [scalar!]!
        enums: [e]
    }
    ");
    test(
        fixture,
        expect![[r#"
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
    );
}

#[test]
fn test_highlight_directive() {
    let fixture = fixture!("" => "
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
    ");
    test(
        fixture,
        expect![[r#"
        [
            1:8..1:14 :: Keyword,
            1:15..1:21 :: Scalar,
            1:22..1:26 :: Directive,
            2:8..2:12 :: Keyword,
            2:13..2:16 :: Object,
            2:17..2:21 :: Directive,
            3:12..3:13 :: Field,
            3:15..3:21 :: Scalar,
            3:22..3:26 :: Directive,
            5:8..5:17 :: Keyword,
            5:18..5:21 :: Interface,
            5:22..5:26 :: Directive,
            6:12..6:13 :: Field,
            6:15..6:21 :: Scalar,
            6:22..6:26 :: Directive,
            9:13..9:14 :: Enum,
            9:15..9:19 :: Directive,
            10:12..10:13 :: EnumValue,
            10:14..10:18 :: Directive,
            11:12..11:13 :: EnumValue,
            11:14..11:18 :: Directive,
        ]
    "#]],
    );
}

#[test]
fn test_highlight() {
    let fixture = fixture!("" => "
    interface I {
        x: Int!
    }

    type Object {
        iface: I
    }

    ");
    test(
        fixture,
        expect![[r#"
            [
                1:4..1:13 :: Keyword,
                1:14..1:15 :: Interface,
                2:8..2:9 :: Field,
                2:11..2:15 :: Type,
                5:4..5:8 :: Keyword,
                5:9..5:15 :: Object,
                6:8..6:13 :: Field,
                6:15..6:16 :: Interface,
            ]
        "#]],
    );
}
