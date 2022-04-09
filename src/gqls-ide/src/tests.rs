use expect_test::expect;
use gqls_db::SourceDatabase;
use maplit::hashset;

use crate::{range, ChangeSummary, Diagnostic, Ide};

#[macro_export]
macro_rules! change {
    ($ide:ident: $file:ident:$a:literal:$b:literal..$x:literal:$y:literal => $text:expr) => {
        $ide.apply_changeset($crate::Changeset::new($file, vec![$crate::Change::Patch(
            $crate::patch!($a: $b..$x: $y => $text)
        )]))
    };
    ($ide:ident: $file:ident => $text:expr) => {
        $ide.apply_changeset($crate::Changeset::new($file, vec![$crate::Change::Set($text.to_owned())]))
    };
}

#[test]
fn test_ide() {
    let mut ide = Ide::default();
    let foo = ide.vfs.intern("foo.gql");
    let summary = change!(ide: foo => "query foo { bar }");
    assert_eq!(summary, ChangeSummary::empty(foo));
    let tree = ide.db.file_tree(foo);
    assert_eq!(ide.file_ropes[&foo].to_string(), "query foo { bar }");
    expect![[r#"(source_file (document (definition (executable_definition (operation_definition (operation_type) (name) (selection_set (selection (field (name)))))))))"#]].assert_eq(&tree.root_node().to_sexp());

    let summary = change!(ide: foo:0:15..0:15 => " baz");
    assert_eq!(summary, ChangeSummary::empty(foo));
    let tree = ide.db.file_tree(foo);
    assert_eq!(ide.file_ropes[&foo].to_string(), "query foo { bar baz }");
    expect![[r#"(source_file (document (definition (executable_definition (operation_definition (operation_type) (name) (selection_set (selection (field (name))) (selection (field (name)))))))))"#]].assert_eq(&tree.root_node().to_sexp());
}

#[test]
fn test_ide_syntax_diagnostics() {
    let mut ide = Ide::default();
    let foo = ide.vfs.intern("foo.gql");
    let summary = change!(ide: foo => "bad");
    assert_eq!(
        summary.diagnostics,
        hashset! {
            Diagnostic::syntax(range!(0:0..0:3))
        }
    );
}
