use super::*;
use expect_test::expect;

#[test]
fn test_duplicate_directive_definition() {
    let gql = "
        directive @foo on FIELD_DEFINITION
        directive @foo on FIELD_DEFINITION
    ";
    test_rendered(
        gql,
        expect![[r#"
            error[0004]: duplicate directive definition `@foo`
              ┌─ test.graphql:3:9
              │
            2 │         directive @foo on FIELD_DEFINITION
              │                   ---- previous definition of directive `@foo` here
            3 │         directive @foo on FIELD_DEFINITION
              │         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

        "#]],
    )
}

#[test]
fn test_duplicate_type_definition() {
    let gql = "
        scalar Foo
        enum Foo { A B }
    ";
    test_rendered(
        gql,
        expect![[r#"
            error[0005]: duplicate type definition `Foo`
              ┌─ test.graphql:3:9
              │
            2 │         scalar Foo
              │                --- previous definition of type `Foo` here
            3 │         enum Foo { A B }
              │         ^^^^^^^^^^^^^^^^

        "#]],
    )
}

#[test]
fn test_type_extension_no_duplicate_diagnostic() {
    let gql = "
        type Foo { id: ID! }
        extend type Foo { foo: Foo! }
    ";
    test_rendered(gql, expect![[]])
}
