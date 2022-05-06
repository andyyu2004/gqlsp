use expect_test::{expect, Expect};
use gqls_fixture::{fixture, Fixture};

use crate::Ide;

fn test(fixture: &Fixture, expect: Expect) {
    let mut ide = Ide::default();
    ide.setup_fixture_allow_errors(&fixture);
    for (file, at) in fixture.all_points() {
        let completions = ide.snapshot().completions(file, at);
        expect.assert_debug_eq(&completions);
    }
}

#[test]
fn test_toplevel_keyword_completions() {
    let fixture = fixture! {
        "foo" => "$"
    };
    test(
        &fixture,
        expect![[r#"
            [
                scalar :: Keyword,
                enum :: Keyword,
                struct :: Keyword,
                union :: Keyword,
                interface :: Keyword,
                directive :: Keyword,
                input :: Keyword,
            ]
        "#]],
    );
}

#[test]
fn test_object_field_completions() {
    let fixture = fixture! {
        "foo" => "
            type Foo {
               bar: $
            }

            input IgnoreInputs {
                bar: Int
            }
        "
    };
    test(
        &fixture,
        expect![[r#"
            [
                Foo :: Object,
            ]
        "#]],
    );
}

#[test]
fn test_input_object_field_completions() {
    let fixture = fixture! {
        "foo" => "
            type IgnoreObject {
               bar: Int
            }

            input Input {
                bar: $
            }
        "
    };
    test(
        &fixture,
        expect![[r#"
            [
                Input :: InputObject,
            ]
        "#]],
    );
}

#[cfg(test)]
mod infer_context;
