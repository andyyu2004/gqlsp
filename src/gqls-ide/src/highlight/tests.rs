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
