use gqls_fixture::fixture;

#[test]
fn test_input_type_referenced_within_output_type() {
    fixture! {
        "foo" => "
            input Input {
                x: Int!
            }

            type Output {
                i: Input!
            }
        }"
    };
}
