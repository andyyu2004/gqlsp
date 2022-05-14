use super::*;

#[test]
fn test_object_implements_non_interface() {
    let gql = "
        scalar Scalar
        type Foo implements Scalar {
            bar: Int
        }
    ";
    test_rendered(
        gql,
        expect![[r#"
            error[0007]: expected an interface, found scalar `Scalar`
              ┌─ test.graphql:3:29
              │
            2 │         scalar Scalar
              │                ------ not an interface
            3 │         type Foo implements Scalar {
              │                             ^^^^^^

        "#]],
    );
}
