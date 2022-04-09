use expect_test::expect;
use gqls_db::SourceDatabase;

use crate::Ide;

#[macro_export]
macro_rules! change {
    ($ide:ident: $file:literal, $text:expr) => {
        $ide.change($file, $crate::ChangeKind::Set($text.to_owned()));
    };
    ($ide:ident: $file:literal:$a:literal:$b:literal..$x:literal:$y:literal, $text:expr) => {
        $ide.change($file, $crate::ChangeKind::Patch(
            $crate::patch!($a: $b..$x: $y => $text),
        ));
    };
}

#[test]
fn test_ide() {
    let mut ide = Ide::default();
    let file_id = ide.vfs.intern("foo.gql");
    change!(ide: "foo.gql", "query foo { bar }");
    let tree = ide.db.file_tree(file_id);
    assert_eq!(ide.file_ropes[&file_id].to_string(), "query foo { bar }");
    expect![[r#"(source_file (document (definition (executable_definition (operation_definition (operation_type) (name) (selection_set (selection (field (name)))))))))"#]].assert_eq(&tree.root_node().to_sexp());

    change!(ide: "foo.gql":0:15..0:15, " baz");
    let tree = ide.db.file_tree(file_id);
    assert_eq!(
        ide.file_ropes[&file_id].to_string(),
        "query foo { bar baz }"
    );
    expect![[r#"(source_file (document (definition (executable_definition (operation_definition (operation_type) (name) (selection_set (selection (field (name))) (selection (field (name)))))))))"#]].assert_eq(&tree.root_node().to_sexp());
}
