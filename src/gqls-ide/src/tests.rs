use expect_test::expect;
use gqls_fixture::Fixture;
use maplit::{hashmap, hashset};

use crate::{range, ChangeSummary, Changeset, ChangesetSummary, Diagnostic, Ide};

macro_rules! idx {
    ($idx:expr) => {
        gqls_ir::Idx::from_raw(gqls_ir::RawIdx::from($idx))
    };
}

pub(crate) use idx;

macro_rules! apply_changeset {
    ($ide:ident: $file:ident:$a:literal:$b:literal..$x:literal:$y:literal => $text:expr) => {
        $ide.apply_changeset($crate::Changeset::single($crate::change!($file:$a:$b..$x:$y => $text)))
    };
    ($ide:ident: $file:ident => $text:expr) => {
        $ide.apply_changeset($crate::Changeset::single($crate::change!($file => $text)))
    };
}

pub(crate) use apply_changeset;

#[macro_export]
macro_rules! change {
    ($file:ident:$a:literal:$b:literal..$x:literal:$y:literal => $text:expr) => {
        $crate::Change::patch($file, $crate::patch!($a: $b..$x: $y => $text))
    };
    ($file:ident => $text:expr) => {
        $crate::Change::set($file, $text.to_owned())
    };
}

macro_rules! setup {
    ($ide:ident: {
        $($file:ident: $text:expr,)*
     }) => {{
        let mut changeset = $crate::Changeset::default()
            .with_projects(maplit::hashmap! { "default" => maplit::hashset! { $($file),* } });
        $( changeset = changeset.with_change($crate::change!($file => $text)); )*
        $ide.apply_changeset(changeset)
    }};
}

pub(crate) use setup;

impl Ide {
    pub fn from_fixture(fixture: &Fixture) -> Self {
        let mut ide = Ide::default();
        ide.setup_fixture(fixture);
        ide
    }

    pub fn from_fixture_allow_errors(fixture: &Fixture) -> Self {
        let mut ide = Ide::default();
        ide.setup_fixture_allow_errors(fixture);
        ide
    }

    pub fn setup_fixture(&mut self, fixture: &Fixture) {
        let summary = self.setup_fixture_allow_errors(fixture);
        for file in fixture.fileset() {
            assert!(
                summary[file].diagnostics.is_empty(),
                "expected no diagnostics, file `{}`: `{:?}`",
                file.display(),
                summary[file].diagnostics
            );
        }
    }

    pub fn setup_fixture_allow_errors(&mut self, fixture: &Fixture) -> ChangesetSummary {
        let mut changeset = Changeset::default().with_projects(hashmap! {
            "default" => fixture.fileset()
        });
        for (file, fixture_file) in fixture.files() {
            changeset = changeset.with_change(change!(file => fixture_file.text));
        }
        self.apply_changeset(changeset)
    }
}

#[test]
fn test_ide() {
    let mut ide = Ide::default();
    let foo = ide.vfs().intern("foo.graphql");
    let summary = apply_changeset!(ide: foo => "scalar Foo");
    assert_eq!(summary, hashmap! { foo => ChangeSummary::default() });
    assert_eq!(ide.file_ropes[&foo].to_string(), "scalar Foo");
    expect![[r#"(document (item (type_definition (scalar_type_definition (name)))))"#]]
        .assert_eq(&ide.snapshot().syntax_tree(foo));

    let summary = apply_changeset!(ide: foo:0:7..0:10 => "Baz");
    assert_eq!(summary, hashmap! { foo => ChangeSummary::default() });
    assert_eq!(ide.file_ropes[&foo].to_string(), "scalar Baz");
    expect![[r#"(document (item (type_definition (scalar_type_definition (name)))))"#]]
        .assert_eq(&ide.snapshot().syntax_tree(foo));
}

#[test]
fn test_ide_syntax_diagnostics() {
    let mut ide = Ide::default();
    let foo = ide.vfs().intern("foo.graphql");
    let summary = apply_changeset!(ide: foo => "bad");
    assert_eq!(
        summary[foo].diagnostics,
        hashset! {
            Diagnostic::syntax(range!(0:0..0:3))
        }
    );
}
