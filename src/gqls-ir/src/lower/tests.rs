use vfs::Vfs;

use crate::tests::{idx, setup, TestDB};
use crate::DefDatabase;

#[test]
fn test_lower_item_body() {
    let mut db = TestDB::default();
    let mut vfs = Vfs::default();
    let foo = vfs.intern("foo");
    setup!(db: {
        foo: r#"type Foo {
            foo: Int
            list: [Int]
            nonNull: Int!
            nonNullList: [Int!]!
            a: [Int!]
            b: [Int]!
        }"#,
    });

    let body = db.item_body(foo, idx!(0));
    dbg!(body);
}
