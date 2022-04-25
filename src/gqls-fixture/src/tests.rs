use gqls_parse::Point;

use crate::{Fixture, FixtureFile};

fn test(fixture: &str, points: Vec<Point>, ranges: Vec<std::ops::Range<Point>>) {
    let actual = FixtureFile::parse(fixture);
    assert_eq!(actual.points, points);
    assert_eq!(actual.ranges, ranges);
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
    );
}
