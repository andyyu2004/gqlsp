use expect_test::expect;

use super::test_rendered;

#[test]
fn test_no_field() {
    let gql = "
        scalar Scalar
        enum Enum { A B }
        type Q {
            foo: Foo
        }
        union U = Foo | Q

        type Foo {}
        interface Bar {}
        input FooInput {}
    ";
    test_rendered(
        gql,
        expect![[r#"
            error[0006]: object `Foo` must define at least one field
              ┌─ test.graphql:9:9
              │
            9 │         type Foo {}
              │         ^^^^^^^^^^^

            error[0006]: interface `Bar` must define at least one field
               ┌─ test.graphql:10:9
               │
            10 │         interface Bar {}
               │         ^^^^^^^^^^^^^^^^

            error[0006]: input `FooInput` must define at least one field
               ┌─ test.graphql:11:9
               │
            11 │         input FooInput {}
               │         ^^^^^^^^^^^^^^^^^

        "#]],
    );
}
