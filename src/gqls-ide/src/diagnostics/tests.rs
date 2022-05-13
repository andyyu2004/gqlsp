use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;
use std::str::FromStr;

use gqls_fixture::{Annotation, Fixture};

use crate::diagnostics::ErrorCode;
use crate::{Diagnostic, Ide, Range};

fn test_common<R>(fixture: &Fixture, f: impl Fn(&Diagnostic) -> R, g: impl Fn(&Annotation) -> R)
where
    R: Hash + Eq + Debug,
{
    let ide = Ide::from_fixture_allow_errors(fixture);
    let snapshot = ide.snapshot();
    for (file, annotations) in fixture.annotations() {
        let diagnostics = snapshot.diagnostics(file);
        let expected =
            diagnostics.iter().map(|diag| (diag.range, f(&diag))).collect::<HashSet<_>>();
        let actual = annotations
            .map(|annotation| (Range::from(annotation.range.clone()), g(&annotation)))
            .collect::<HashSet<_>>();
        assert_eq!(expected, actual);
    }
}

fn test_error_message(fixture: &Fixture) {
    test_common(fixture, |diag| diag.message.clone(), |annotation| annotation.text.clone());
}

fn test_error_code(fixture: &Fixture) {
    test_common(
        fixture,
        |diag| diag.code,
        |annotation| annotation.text.parse::<ErrorCode>().unwrap(),
    );
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

mod duplicate;
mod io;
mod syntax;
mod unresolved;
