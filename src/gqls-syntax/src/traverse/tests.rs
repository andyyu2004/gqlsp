use expect_test::expect;

use crate::{parse_fresh, traverse};

#[test]
fn test_tree_traversal_simple() {
    let tree = parse_fresh("type Foo {}");
    let events = traverse(&tree).collect::<Vec<_>>();
    expect![[r#"
        [
            > item,
            > type_definition,
            > object_type_definition,
            > type,
            < type,
            > name,
            < name,
            > fields_definition,
            > {,
            < {,
            > },
            < },
            < fields_definition,
            < object_type_definition,
            < type_definition,
            < item,
        ]
    "#]]
    .assert_debug_eq(&events);
}

#[test]
fn test_tree_traversal() {
    let tree = parse_fresh(
        "type Foo {
            foo: Int!
            bar: [String!]!
        }",
    );
    let events = traverse(&tree).collect::<Vec<_>>();
    expect![[r#"
        [
            > item,
            > type_definition,
            > object_type_definition,
            > type,
            < type,
            > name,
            < name,
            > fields_definition,
            > {,
            < {,
            > field_definition,
            > name,
            < name,
            > :,
            < :,
            > type,
            > non_null_type,
            > named_type,
            < named_type,
            > !,
            < !,
            < non_null_type,
            < type,
            < field_definition,
            > field_definition,
            > name,
            < name,
            > :,
            < :,
            > type,
            > non_null_type,
            > list_type,
            > [,
            < [,
            > type,
            > non_null_type,
            > named_type,
            < named_type,
            > !,
            < !,
            < non_null_type,
            < type,
            > ],
            < ],
            < list_type,
            > !,
            < !,
            < non_null_type,
            < type,
            < field_definition,
            > },
            < },
            < fields_definition,
            < object_type_definition,
            < type_definition,
            < item,
        ]
    "#]]
    .assert_debug_eq(&events);
}
