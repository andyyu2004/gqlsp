use gqls_fixture::{fixture, Fixture};
use gqls_ir::DirectiveLocations;

use crate::Ide;

use super::super::{CompletionCtxt, Context};

#[track_caller]
fn test(fixture: &Fixture, expected: Context) {
    let mut ide = Ide::default();
    ide.setup_fixture_allow_errors(&fixture);
    let snapshot = ide.snapshot();
    for (file, at) in fixture.all_points() {
        let context = CompletionCtxt::infer_context(&snapshot, file, at);
        assert_eq!(expected, context);
    }
}

#[test]
fn test_infer_field_context() {
    let fixture = fixture! {
        "foo" => "
            type Foo {
                bar: $$$
            }
        "

        "bar" => "
            type Foo {
                bar: Foo$$$
            }
        "

        "baz" => "
            type Foo {
                bar: Foo$$$
        "

        "wrapped types" => "
            interface EssentialsTrait {
              id: [TraitID!]! $
            }
        "

        "long type name" => "
            type Foo {
              unit: NameLongerThanLookbehind$
            }
        "


        // FIXME
        // "qux" => "
        //     type Foo {
        //         bar: $$$
        // "
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

        "bar" => "
            input Foo {
                bar: Foo$$$
            }
        "


        "wrapped_type" => "
            input Foo {
                bar: Foo!$$$
            }
        "

        // FIXMEs
        // "baz" => "
        //     input Foo {
        //         bar: Foo$$$
        // "

        // "qux" => "
        //     type Foo {
        //         bar: $$$
        // "
    };
    test(&fixture, Context::InputField);
}

#[test]
fn test_infer_top_level_context() {
    let fixture = fixture! {
        "bar" => "$"
        "foo" => "
            $
            $type Foo {
                bar: Int!
            }
            $
        "
        //FIXME adding a $ after closing } fails
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
        "a" => "type Foo $"
        "b" => "extend type Foo $"
        "c" => "extend type Foo $ {}"
        "d" => "extend type Foo$$${
            bar: Int
        }"
        "e" => "type Foo $"
        "f" => "type Foo $ {}"
        "g" => "type Foo ${
            bar: Int
        }"
    };
    // TODO interface context after implements
    // suggest `implements` keyword in this context too for types?
    test(&fixture, Context::Directive(DirectiveLocations::OBJECT));
}

#[test]
fn test_infer_enum_directive_context() {
    let fixture = fixture! {
        "a" => "enum Foo $"
        "b" => "enum Foo $ { A B }"
        "c" => "extend enum Foo $"
    };
    test(&fixture, Context::Directive(DirectiveLocations::ENUM));
}

#[test]
fn test_infer_union_directive_context() {
    let fixture = fixture! {
        "foo" => "union Foo $"
        "bar" => "extend union Foo $"
        "baz" => "union Foo $ = A | B"
    };
    test(&fixture, Context::Directive(DirectiveLocations::UNION));
}

#[test]
fn test_infer_interface_directive_context() {
    let fixture = fixture! {
        "foo" => "interface Foo $"
        "bar" => "extend interface Foo $"
        "foobar" => "interface Foo $ { bar: Int }"
    };
    test(&fixture, Context::Directive(DirectiveLocations::INTERFACE));
}

#[test]
fn test_infer_scalar_directive_context() {
    let fixture = fixture! {
        "foo" => "scalar Foo $"
        "bar" => "extend scalar Foo $"
    };
    test(&fixture, Context::Directive(DirectiveLocations::SCALAR));
}

#[test]
fn test_infer_scalar_input_object_context() {
    let fixture = fixture! {
        "foo" => "input Foo $"
        "bar" => "extend input Foo $"
        "foobar" => "input Foo $ {}"
    };
    test(&fixture, Context::Directive(DirectiveLocations::INPUT_OBJECT));
}

#[test]
fn test_infer_scalar_enum_value_context() {
    let fixture = fixture! {
        "foo" => "enum Foo { A $ "
    };
    test(&fixture, Context::Directive(DirectiveLocations::ENUM_VALUE));
}
