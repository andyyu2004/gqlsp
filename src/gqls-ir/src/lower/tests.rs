use expect_test::expect;
use gqls_fixture::fixture;
use testing::TestDatabaseExt;
use vfs::Vfs;

use crate::tests::{idx, TestDB};
use crate::{DefDatabase, ItemRes};

#[test]
fn test_lower_item_body() {
    let mut vfs = Vfs::default();
    let foo = vfs.intern("foo");
    let fixture = fixture! {
        foo => "
        directive @qux on FIELD_DEFINITION
        extend type Foo {
            foo: Int @qux
            list: [Int]
            nonNull: Int! @qux
            nonNullList: [Int!]! @qux
            a: [Int!]
            b: [Int]!
        }
        "
    };
    let db = TestDB::from_fixture(&fixture);

    let body = db.item_body(ItemRes::new(foo, idx!(1)));
    expect![[r#"
        Some(
            ItemBody {
                diagnostics: [],
                kind: ObjectTypeDefinition(
                    TypeDefinitionBody {
                        fields: [
                          foo: Int @qux
                          list: [Int]
                          nonNull: Int! @qux
                          nonNullList: [Int!]! @qux
                          a: [Int!]
                          b: [Int]!
                        ],
                    },
                ),
            },
        )
    "#]]
    .assert_debug_eq(&body);
}
