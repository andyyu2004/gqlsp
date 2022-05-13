use gqls_fixture::fixture;

#[test]
fn test_duplicate_definition() {
    let fixture = fixture! {
        "foo" => "
            type Foo {
                id: ID!
            }

            type Foo {
                id: ID!
            }
        "
    };
}
