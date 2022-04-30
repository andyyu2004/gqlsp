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
fn test_highlight() {
    let fixture = fixture!("" => "
    type Foo {
        x: Int!
    }
    ");
    test(
        fixture,
        expect![[r#"
            [
                SemanticToken {
                    range: (1, 4)..(1, 8),
                    kind: Keyword,
                },
                SemanticToken {
                    range: (1, 9)..(1, 12),
                    kind: Type,
                },
                SemanticToken {
                    range: (2, 8)..(2, 9),
                    kind: Type,
                },
                SemanticToken {
                    range: (2, 11)..(2, 15),
                    kind: Type,
                },
            ]
        "#]],
    );
}
