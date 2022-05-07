use expect_test::expect;
use gqls_fixture::fixture;

use super::test;

const DIRECTIVES: &str = "
    directive @onFieldAndObject on FIELD_DEFINITION | OBJECT
    directive @onObjectAndInterface on OBJECT | INTERFACE
    directive @onSchema on SCHEMA
    directive @onScalar on SCALAR
    directive @onObject on OBJECT
    directive @onField on FIELD_DEFINITION
    directive @onArgument on ARGUMENT_DEFINITION
    directive @onInterface on INTERFACE
    directive @onUnion on UNION
    directive @onEnum on ENUM
    directive @onEnumValue on ENUM_VALUE
    directive @onInputObject on INPUT_OBJECT
    directive @onInputFieldDefinition on INPUT_FIELD_DEFINITION
";

macro_rules! test {
    ($src:literal, $expect:expr) => {{
        let fixture = fixture! {
            "foo" => format!("
            {DIRECTIVES}
            {},
        ", $src)
        };
        test(&fixture, $expect)
    }};
}

#[test]
fn test_complete_directives() {
    test!(
        "type Foo $",
        expect![[r#"
            [
                onFieldAndObject :: Directive,
                onObjectAndInterface :: Directive,
                onObject :: Directive,
            ]
        "#]]
    );

    test!(
        "interface Foo $",
        expect![[r#"
            [
                onObjectAndInterface :: Directive,
                onInterface :: Directive,
            ]
        "#]]
    );

    test!(
        "enum Foo $",
        expect![[r#"
            [
                onEnum :: Directive,
            ]
        "#]]
    );

    test!(
        "enum Foo { A, B $ }",
        expect![[r#"
            [
                onEnumValue :: Directive,
            ]
        "#]]
    );

    test!(
        "union Foo $",
        expect![[r#"
            [
                onUnion :: Directive,
            ]
        "#]]
    );

    test!(
        "input Foo $",
        expect![[r#"
            [
                onInputObject :: Directive,
            ]
        "#]]
    );

    test!(
        "scalar Foo $",
        expect![[r#"
            [
                onScalar :: Directive,
            ]
        "#]]
    );
}
