use gqls_parse::Point;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fixture {
    points: Vec<Point>,
}

impl Fixture {
    pub fn parse(fixture: &str) -> Self {
        let mut points = vec![];
        for (row, line) in fixture.lines().enumerate() {
            for (column, char) in line.char_indices() {
                if char == '^' {
                    if row == 0 {
                        panic!("cannot contain `^` in the first line");
                    }
                    points.push(Point { row: row - 1, column });
                }
            }
        }
        Self { points }
    }
}

#[cfg(test)]
mod tests;
