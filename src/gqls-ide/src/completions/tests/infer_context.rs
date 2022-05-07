use gqls_fixture::{fixture, Fixture};

use crate::Ide;

use super::super::{CompletionCtxt, Context};

fn test(fixture: &Fixture, expected: Context) {
    let mut ide = Ide::default();
    ide.setup_fixture_allow_errors(&fixture);
    let snapshot = ide.snapshot();
    for (file, at) in fixture.all_points() {
        let context = CompletionCtxt::infer_context(&snapshot, file, at);
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
    test(&fixture, Context::UnionMembers);
}

#[test]
fn test_infer_union_member_types_context_first_member() {
    let fixture = fixture! {
        "foo" => "union Union = $"
    };
    test(&fixture, Context::UnionMembers);
}

#[test]
fn test_infer_type_directive_context() {
    let fixture = fixture! {
        "foo" => "type Foo $"
        "bar" => "extend type Foo $"
    };
    // TODO interface context after implements
    // suggest implements keyword in this context too for types?
    test(&fixture, Context::Directive());
}

#[test]
fn test_infer_enum_directive_context() {
    let fixture = fixture! {
        "foo" => "enum Foo $"
        "bar" => "extend enum Foo $"
    };
    test(&fixture, Context::Directive());
}

#[test]
fn test_infer_union_directive_context() {
    let fixture = fixture! {
        "foo" => "union Foo $"
        "bar" => "extend union Foo $"
    };
    test(&fixture, Context::Directive());
}

#[test]
fn test_infer_interface_directive_context() {
    let fixture = fixture! {
        "foo" => "interface Foo $"
        "bar" => "extend interface Foo $"
    };
    test(&fixture, Context::Directive());
}

#[test]
fn test_infer_scalar_directive_context() {
    let fixture = fixture! {
        "foo" => "scalar Foo $"
        "bar" => "extend scalar Foo $"
    };
    test(&fixture, Context::Directive());
}

#[test]
fn test_infer_scalar_input_object_context() {
    let fixture = fixture! {
        "foo" => "input Foo $"
        "bar" => "extend input Foo $"
    };
    test(&fixture, Context::Directive());
}

#[test]
fn test_infer_scalar_enum_value_context() {
    let fixture = fixture! {
        "foo" => "enum Foo { A $ "
    };
    test(&fixture, Context::Directive());
}
