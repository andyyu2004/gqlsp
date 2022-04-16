use gqls_parse::Point;

use crate::FixtureFile;

fn test(fixture: &str, points: Vec<Point>, ranges: Vec<std::ops::Range<Point>>) {
    let actual = FixtureFile::parse(fixture);
    assert_eq!(actual.points, points);
    assert_eq!(actual.ranges, ranges);
}

#[test]
#[should_panic]
fn test_caret_on_first_line() {
    FixtureFile::parse("^");
}

#[test]
fn test_parse_fixture() {
    test(
        r#"
scalar Foo
#      ^^^
scalar Bar
...... ...
    "#,
        vec![Point { row: 1, column: 7 }, Point { row: 1, column: 8 }, Point { row: 1, column: 9 }],
        vec![
            Point { row: 3, column: 0 }..Point { row: 3, column: 6 },
            Point { row: 3, column: 7 }..Point { row: 3, column: 10 },
        ],
    );
}
