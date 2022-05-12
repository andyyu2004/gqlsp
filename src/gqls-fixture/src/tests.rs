use gqls_syntax::Point;

use crate::{Annotation, FixtureFile};

fn test(
    fixture: &str,
    points: Vec<Point>,
    ranges: Vec<std::ops::Range<Point>>,
    annotations: Vec<Annotation>,
) {
    let actual = FixtureFile::parse(fixture);
    assert_eq!(actual.points, points);
    assert_eq!(actual.ranges, ranges);
    assert_eq!(actual.text, fixture.replace('$', " "));
}

#[test]
#[should_panic]
fn test_caret_on_first_line() {
    FixtureFile::parse("#^");
}

#[test]
fn test_parse_fixture_with_delimited_range() {
    test(
        r#"
#{
scalar Foo
scalar Bar
#}
    "#,
        vec![],
        vec![Point { row: 2, column: 0 }..Point { row: 3, column: 10 }],
        vec![],
    );
}

#[test]
fn test_parse_fixture() {
    test(
        r#"
scalar Foo
#      ^^^
 scalar Bar
#...... ...
#^^
    "#,
        vec![
            Point { row: 1, column: 7 },
            Point { row: 1, column: 8 },
            Point { row: 1, column: 9 },
            Point { row: 3, column: 1 },
            Point { row: 3, column: 2 },
        ],
        vec![
            Point { row: 3, column: 1 }..Point { row: 3, column: 7 },
            Point { row: 3, column: 8 }..Point { row: 3, column: 11 },
        ],
        vec![],
    );
}

#[test]
fn test_inline_points() {
    test(
        r#"
scalar $$$
        "#,
        vec![Point { row: 1, column: 7 }, Point { row: 1, column: 8 }, Point { row: 1, column: 9 }],
        vec![],
        vec![],
    );
}

#[test]
fn test_annotations() {
    test(
        r#"
scalar E
# ...'hello'
   "#,
        vec![],
        vec![],
        vec![Annotation {
            range: Point { row: 1, column: 2 }..Point { row: 1, column: 5 },
            text: "hello".to_owned(),
        }],
    );
}
