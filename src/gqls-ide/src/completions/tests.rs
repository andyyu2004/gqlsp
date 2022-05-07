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

            interface Interface { bar: Int }
            scalar Scalar
            enum Enum { A, B }
            union Union = Foo | Foo
            input IgnoreInputs { bar: Int }
            directive @qux on FIELD_DEFINITION
            type Bar { bar: Int }
        "
    };
    test(
        &fixture,
        expect![[r#"
            [
                Foo :: Object,
                Interface :: Interface,
                Scalar :: Scalar,
                Enum :: Enum,
                Union :: Union,
                Bar :: Object,
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
            interface IgnoreInterface { bar: Int }
            scalar Scalar
            enum Enum { A, B }
            union IgnoreUnion = Foo | Foo
            input AnotherInput { bar: Int }
            directive @qux ON FIELD

            input Input {
                bar: $
            }
        "
    };
    test(
        &fixture,
        expect![[r#"
            [
                Scalar :: Scalar,
                Enum :: Enum,
                AnotherInput :: InputObject,
                Input :: InputObject,
            ]
        "#]],
    );
}

#[test]
fn test_union_member_field_completions() {
    let fixture = fixture! {
        "foo" => "
            type OnlyObjects {
               bar: Int
            }
            interface IgnoreInterface { bar: Int }
            scalar IgnoreScalar
            enum IgnoreEnum { A, B }
            union IgnoreUnion = Foo | Foo
            directive @qux ON FIELD

            union Union = Foo | $
        "
    };
    test(
        &fixture,
        expect![[r#"
            [
                OnlyObjects :: Object,
            ]
        "#]],
    );
}

#[cfg(test)]
mod infer_context;

#[cfg(test)]
mod queries;
