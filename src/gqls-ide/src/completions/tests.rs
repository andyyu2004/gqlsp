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
        "foo" => "
            type Foo {
               bar: $
            }
        "
    };
    // FIXME
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
fn test_field_completions() {
    let fixture = fixture! {
        "foo" => "
            type Foo {
               bar: $
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

#[cfg(test)]
mod infer_context;
