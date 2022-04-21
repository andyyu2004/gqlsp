use expect_test::{expect, Expect};

use crate::make_parser;

fn test(s: &str, expect: Expect) {
    let sexp = make_parser().parse(s, None).unwrap().root_node().to_sexp();
    expect.assert_eq(&sexp);
}

#[test]
fn test_parse_empty_type() {
    test(
        "type Foo {}",
        expect![[
            r#"(document (item (type_definition (object_type_definition (name) (fields_definition)))))"#
        ]],
    );
}

#[test]
fn test_parse_union() {
    test(
        "union U = X | Y | Z",
        expect![[
            r#"(document (item (type_definition (union_type_definition (name) (union_member_types (named_type (name)) (named_type (name)) (named_type (name)))))))"#
        ]],
    );

    test(
        "union U = | X | Y",
        expect![[
            r#"(document (item (type_definition (union_type_definition (name) (union_member_types (named_type (name)) (named_type (name)))))))"#
        ]],
    );
}

#[test]
fn test_make_parser() {
    make_parser();
}

#[test]
fn test_parse_empty() {
    let mut parser = make_parser();
    let tree = parser.parse("", None).unwrap();
    assert_eq!(tree.root_node().to_sexp(), "(document)");
}
