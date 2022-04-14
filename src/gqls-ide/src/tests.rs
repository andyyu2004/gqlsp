use expect_test::expect;
use maplit::{hashmap, hashset};

use crate::{range, ChangeSummary, Diagnostic, Ide};

#[macro_export]
macro_rules! apply_changeset {
    ($ide:ident: $file:ident:$a:literal:$b:literal..$x:literal:$y:literal => $text:expr) => {
        $ide.apply_changeset($crate::Changeset::single($crate::change!($file:$a:$b..$x:$y => $text)))
    };
    ($ide:ident: $file:ident => $text:expr) => {
        $ide.apply_changeset($crate::Changeset::single($crate::change!($file => $text)))
    };
}

#[macro_export]
macro_rules! change {
    ($file:ident:$a:literal:$b:literal..$x:literal:$y:literal => $text:expr) => {
        $crate::Change::patch($file, $crate::patch!($a: $b..$x: $y => $text))
    };
    ($file:ident => $text:expr) => {
        $crate::Change::set($file, $text.to_owned())
    };
}

#[test]
fn test_ide() {
    let mut ide = Ide::default();
    let foo = ide.vfs.intern("foo.gql");
    let summary = apply_changeset!(ide: foo => "scalar Foo");
    assert_eq!(summary, hashmap! { foo => ChangeSummary::default() });
    assert_eq!(ide.file_ropes[&foo].to_string(), "scalar Foo");
    expect![[r#"(document (item (type_definition (scalar_type_definition (name)))))"#]]
        .assert_eq(&ide.analysis().syntax_tree(foo));

    let summary = apply_changeset!(ide: foo:0:7..0:10 => "Baz");
    assert_eq!(summary, hashmap! { foo => ChangeSummary::default() });
    assert_eq!(ide.file_ropes[&foo].to_string(), "scalar Baz");
    expect![[r#"(document (item (type_definition (scalar_type_definition (name)))))"#]]
        .assert_eq(&ide.analysis().syntax_tree(foo));
}

#[test]
fn test_ide_syntax_diagnostics() {
    let mut ide = Ide::default();
    let foo = ide.vfs.intern("foo.gql");
    let summary = apply_changeset!(ide: foo => "bad");
    assert_eq!(
        summary[foo].diagnostics,
        hashset! {
            Diagnostic::syntax(range!(0:0..0:3))
        }
    );

    let summary = apply_changeset!(ide: foo:0:0..0:3 => "type Empty {}");
    assert_eq!(
        summary[foo].diagnostics,
        hashset! {
            Diagnostic::syntax(range!(0:11..0:13))
        }
    );
}
