use gqls_parse::Point;

use crate::Fixture;

fn test(fixture: &str, expected: Fixture) {
    let actual = Fixture::parse(fixture);
    assert_eq!(actual, expected);
}

#[test]
#[should_panic]
fn test_caret_on_first_line() {
    Fixture::parse("^");
}

#[test]
fn test_parse_fixture() {
    test(
        r#"
scalar Foo
#      ^^^
    "#,
        Fixture {
            points: vec![
                Point { row: 1, column: 7 },
                Point { row: 1, column: 8 },
                Point { row: 1, column: 9 },
            ],
        },
    );
}
