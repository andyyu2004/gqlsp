use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;
use std::str::FromStr;

use expect_test::Expect;
use gqls_fixture::{Annotation, Fixture};
use gqls_syntax::Point;

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

fn test_rendered(gql: &str, expect: Expect) {
    let (ide, file) = Ide::from_file(gql);
    let snapshot = ide.snapshot();
    let diagnostics = snapshot.diagnostics(file);
    let rendered = render_diagnostics(gql, diagnostics);
    expect.assert_eq(&rendered);
}

// render diagnostics into a string using codespan
fn render_diagnostics(gql: &str, diagnostics: impl IntoIterator<Item = Diagnostic>) -> String {
    use codespan_reporting::{diagnostic, term};
    let point_to_byte = |point: Point| {
        gql.lines().take(point.row).map(|line| 1 + line.len()).sum::<usize>() + point.column
    };
    let range_to_span = |range: Range| -> std::ops::Range<usize> {
        point_to_byte(range.start)..point_to_byte(range.end)
    };

    let convert_diagnostic = |diagnostic: &Diagnostic| -> diagnostic::Diagnostic<()> {
        let mut labels = vec![diagnostic::Label::new(
            diagnostic::LabelStyle::Primary,
            (),
            range_to_span(diagnostic.range),
        )];
        labels.extend(diagnostic.labels.iter().map(|label| {
            diagnostic::Label::new(
                diagnostic::LabelStyle::Secondary,
                (),
                range_to_span(label.location.range),
            )
            .with_message(&label.message)
        }));
        diagnostic::Diagnostic::new(match diagnostic.severity {
            crate::Severity::Error => diagnostic::Severity::Error,
        })
        .with_message(&diagnostic.message)
        .with_code(diagnostic.code.to_string())
        .with_labels(labels)
    };

    #[derive(Default)]
    struct StringWriter(Vec<u8>);

    impl std::io::Write for StringWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.0.write(buf)
        }

        fn flush(&mut self) -> std::io::Result<()> {
            self.0.flush()
        }
    }

    impl term::termcolor::WriteColor for StringWriter {
        fn supports_color(&self) -> bool {
            false
        }

        fn set_color(&mut self, _spec: &term::termcolor::ColorSpec) -> std::io::Result<()> {
            Ok(())
        }

        fn reset(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    let mut writer = StringWriter::default();

    for diagnostic in diagnostics {
        let diag = convert_diagnostic(&diagnostic);
        codespan_reporting::term::emit(
            &mut writer,
            &term::Config { ..Default::default() },
            &codespan_reporting::files::SimpleFile::new("test.graphql", gql),
            &diag,
        )
        .unwrap();
    }

    String::from_utf8(writer.0).unwrap()
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
