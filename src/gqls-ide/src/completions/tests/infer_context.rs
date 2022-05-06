use gqls_fixture::{fixture, Fixture};

use crate::Ide;

use super::super::{CompletionCtxt, Context};

fn test(fixture: &Fixture, expected: Context) {
    let mut ide = Ide::default();
    ide.setup_fixture_allow_errors(&fixture);
    let snapshot = ide.snapshot();
    for (file, at) in fixture.all_points() {
        let context = CompletionCtxt::context(&snapshot, file, at);
        assert_eq!(context, expected);
    }
}

#[test]
fn test_infer_field_context() {
    let fixture = fixture! {
        "foo" => "
            type Foo {
                bar:$$$
            }
        "
    };
    test(&fixture, Context::Field);
}

#[test]
fn test_infer_input_field_context() {
    let fixture = fixture! {
        "foo" => "
            input Foo {
                bar:$$$
            }
        "
    };
    test(&fixture, Context::InputField);
}

#[test]
fn test_infer_top_level_context() {
    let fixture = fixture! {
        "bar" => "$"
        "foo" => "
            $

            type Foo {
                bar: Int!
            }

            $
        "
    };
    test(&fixture, Context::Document);
}

#[test]
fn test_infer_union_member_types_context() {
    let fixture = fixture! {
        "foo" => "union Union = $ Foo $ | $ Bar $ | $ Baz $$$"
    };
    test(&fixture, Context::Union);
}

#[test]
fn test_infer_union_member_types_context_first_member() {
    let fixture = fixture! {
        "foo" => "union Union = $"
    };
    test(&fixture, Context::Union);
}
