use expect_test::expect;

use super::test_rendered;

#[test]
fn test_no_field() {
    let gql = "
        type Foo {}
        interface Bar {}
        input FooInput {}
    ";
    test_rendered(
        gql,
        expect![[r#"
            error[0006]: object `Foo` must define at least one field
              ┌─ test.graphql:2:9
              │
            2 │         type Foo {}
              │         ^^^^^^^^^^^

            error[0006]: interface `Bar` must define at least one field
              ┌─ test.graphql:3:9
              │
            3 │         interface Bar {}
              │         ^^^^^^^^^^^^^^^^

            error[0006]: input `FooInput` must define at least one field
              ┌─ test.graphql:4:9
              │
            4 │         input FooInput {}
              │         ^^^^^^^^^^^^^^^^^

        "#]],
    );
}
