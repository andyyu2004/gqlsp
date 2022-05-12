use std::collections::HashSet;
use std::str::FromStr;

use gqls_fixture::Fixture;

use crate::diagnostics::ErrorCode;
use crate::{Ide, Range};

fn test(fixture: &Fixture) {
    let ide = Ide::from_fixture_allow_errors(fixture);
    let snapshot = ide.snapshot();
    for (file, annotations) in fixture.annotations() {
        let diagnostics = snapshot.diagnostics(file);
        let expected =
            diagnostics.iter().map(|diag| (diag.range, diag.code)).collect::<HashSet<_>>();
        let actual = annotations
            .map(|annotation| {
                (
                    Range::from(annotation.range.clone()),
                    annotation.text.parse::<ErrorCode>().unwrap(),
                )
            })
            .collect::<HashSet<_>>();
        assert_eq!(expected, actual);
    }
}

#[test]
fn test_display_error_code() {
    assert_eq!(ErrorCode(2).to_string(), "0002");
    assert_eq!(ErrorCode(42).to_string(), "0042");
}

#[test]
fn test_parse_error_code() {
    assert_eq!(ErrorCode::from_str("E0002"), Ok(ErrorCode(2)));
    assert_eq!(ErrorCode::from_str("E42"), Ok(ErrorCode(42)));
}

#[cfg(test)]
mod syntax;
