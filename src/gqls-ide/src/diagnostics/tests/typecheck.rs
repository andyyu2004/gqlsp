use super::*;
use expect_test::expect;

#[test]
fn test_incompatible_field_argument_scalar() {
    let gql = r#"
        type Foo {
            badid(id: ID! = false): [Foo!]
            id(id: ID! = ""): [Foo!]

            errTypeShouldNotResultInFurtherErrors(id: BadType! = ""): [Foo!]

            bool(id: Boolean! = false): [Foo!]
            badbool(id: Boolean! = 0): [Foo!]

            str(id: String! = ""): [Foo!]
            badstr(id: String! = 234): [Foo!]
        }
    "#;
    test_rendered(
        gql,
        expect![[r#"
            error[0010]: value `false` is incompatible with type `ID!` (cannot use boolean value as ID type)
              ┌─ test.graphql:3:19
              │
            3 │             badid(id: ID! = false): [Foo!]
              │                   ^^^^^^^^^^^^^^^

            error[0003]: unresolved type `BadType`
              ┌─ test.graphql:6:55
              │
            6 │             errTypeShouldNotResultInFurtherErrors(id: BadType! = ""): [Foo!]
              │                                                       ^^^^^^^

            error[0010]: value `0` is incompatible with type `Boolean!` (cannot use integer value as boolean type)
              ┌─ test.graphql:9:21
              │
            9 │             badbool(id: Boolean! = 0): [Foo!]
              │                     ^^^^^^^^^^^^^^^^

            error[0010]: value `234` is incompatible with type `String!` (cannot use integer value as string type)
               ┌─ test.graphql:12:20
               │
            12 │             badstr(id: String! = 234): [Foo!]
               │                    ^^^^^^^^^^^^^^^^^

        "#]],
    )
}

#[test]
fn test_incompatible_field_argument_enum() {
    let gql = r#"
        enum E { A B C }

        type Foo {
            a(e: E! = A): [Foo!]
            b(e: E! = B): [Foo!]
            c(e: E! = C): [Foo!]
            d(e: E! = D): [Foo!]
        }
    "#;
    test_rendered(
        gql,
        expect![[r#"
            error[0010]: value `D` is incompatible with type `E!` (`D` is not a valid variant of enum `E`)
              ┌─ test.graphql:8:15
              │
            8 │             d(e: E! = D): [Foo!]
              │               ^^^^^^^^^

        "#]],
    )
}

#[test]
fn test_incompatible_field_argument_object() {
    let gql = r#"
        input Input {
            id: ID!
            bool: Boolean
        }

        type Foo {
            good(input: Input! = { id: "1", bool: false }): Boolean!
            goodOmitNullable(input: Input! = { id: "1" }): Boolean!
            badOmitNonNullable(input: Input! = { bool: false }): Boolean!
            badExtraField(input: Input! = { id: "1", random: 1 }): Boolean!
            incorrectFieldType(input: Input! = { id: { random: 3 } }): Boolean!
        }
    "#;
    test_rendered(
        gql,
        expect![[r#"
            error[0010]: value `{ bool: false }` is incompatible with type `Input!` (non-nullable field `id` must be provided)
               ┌─ test.graphql:10:32
               │
            10 │             badOmitNonNullable(input: Input! = { bool: false }): Boolean!
               │                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

            error[0010]: value `{ id: "1", random: 1 }` is incompatible with type `Input!` (field `random` is not a member of type `Input`)
               ┌─ test.graphql:11:27
               │
            11 │             badExtraField(input: Input! = { id: "1", random: 1 }): Boolean!
               │                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

            error[0010]: value `{ id: { random: 3 } }` is incompatible with type `Input!` (cannot use object value as ID type)
               ┌─ test.graphql:12:32
               │
            12 │             incorrectFieldType(input: Input! = { id: { random: 3 } }): Boolean!
               │                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

        "#]],
    )
}
