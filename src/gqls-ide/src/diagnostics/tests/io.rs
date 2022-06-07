use crate::diagnostics::tests::test_rendered;
use expect_test::expect;

#[test]
fn test_input_type_referenced_within_output_type() {
    let gql = "
        input Input {
            x: Int!
        }

        type Output {
            i: Input!
        }
    ";
    test_rendered(
        gql,
        expect![[r#"
        error[0009]: expected an output type, found non-null input object `Input!`
          ┌─ test.graphql:7:16
          │
        7 │             i: Input!
          │                ^^^^^^

    "#]],
    )
}

#[test]
fn test_output_type_referenced_within_input_type() {
    let gql = "
        input Input {
            out: [Output!]
        }

        type Output {
            i: Int!
        }
    ";

    test_rendered(
        gql,
        expect![[r#"
            error[0008]: expected an input type, found list of non-null objects `[Output!]`
              ┌─ test.graphql:3:18
              │
            3 │             out: [Output!]
              │                  ^^^^^^^^^

        "#]],
    )
}

#[test]
fn test_output_type_referenced_within_argument() {
    let gql = "
        type Output {
            foo(id: Output!): Int!
        }
    ";

    test_rendered(
        gql,
        expect![[r#"
            error[0008]: expected an input type, found non-null object `Output!`
              ┌─ test.graphql:3:21
              │
            3 │             foo(id: Output!): Int!
              │                     ^^^^^^^

        "#]],
    )
}
