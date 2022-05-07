use expect_test::{expect, Expect};
use tree_sitter::Point;

use crate::{make_parser, parse_fresh, NodeExt};

fn test(s: &str, expect: Expect) {
    let sexp = make_parser().parse(s, None).unwrap().root_node().to_sexp();
    expect.assert_eq(&sexp);
}

#[test]
fn test_named_node_at() {
    let s = "
type Foo {
    s: [Int!]!
}";
    let tree = parse_fresh(s);
    let node = tree.root_node();
    let i = node.named_node_at(Point::new(2, 8)).unwrap();
    expect![[r#"(name)"#]].assert_eq(&i.to_sexp());

    let xs = node.named_node_at(Point::new(2, 7)).unwrap();
    expect![[r#"(list_type (type (non_null_type (named_type (name)))))"#]].assert_eq(&xs.to_sexp());

    let nn = node.named_node_at(Point::new(2, 11)).unwrap();
    expect![[r#"(non_null_type (named_type (name)))"#]].assert_eq(&nn.to_sexp());
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
fn test_parse_directive_definition() {
    test(
        "directive @qux on OBJECT | FIELD_DEFINITION",
        expect![[
            r#"(document (item (directive_definition (name) (directive_locations (directive_location) (directive_location)))))"#
        ]],
    );
}

#[test]
fn test_parse_implements_interface() {
    test(
        "type Foo implements Bar {}",
        expect![[
            r#"(document (item (type_definition (object_type_definition (name) (implements_interfaces (named_type (name))) (fields_definition)))))"#
        ]],
    );

    test(
        "type Foo implements Bar & Bar {}",
        expect![[
            r#"(document (item (type_definition (object_type_definition (name) (implements_interfaces (implements_interfaces (named_type (name))) (named_type (name))) (fields_definition)))))"#
        ]],
    );

    test(
        "type Foo implements & Bar & Bar {}",
        expect![[
            r#"(document (item (type_definition (object_type_definition (name) (implements_interfaces (implements_interfaces (named_type (name))) (named_type (name))) (fields_definition)))))"#
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
fn test_resilience() {
    test(
        "type K ",
        expect![[r#"(document (item (type_definition (object_type_definition (name)))))"#]],
    );

    test(
        "extend type T ",
        expect![[r#"(document (item (type_extension (object_type_extension (name)))))"#]],
    );

    test(
        "interface I ",
        expect![[r#"(document (item (type_definition (interface_type_definition (name)))))"#]],
    );

    test(
        "enum E ",
        expect![[r#"(document (item (type_definition (enum_type_definition (name)))))"#]],
    );

    test(
        "union U ",
        expect![[r#"(document (item (type_definition (union_type_definition (name)))))"#]],
    );

    test(
        "union U =",
        expect![[
            r#"(document (item (type_definition (union_type_definition (name) (union_member_types)))))"#
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
