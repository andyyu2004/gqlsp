use gqls_fixture::fixture;
use gqls_ir::ItemRes;

use crate::tests::idx;
use crate::Ide;

#[test]
fn test_resolve_item_at() {
    let fixture = fixture! {
        "foo" => "
            type Bar {
                foo: Foo!
            }

            type Foo {
           #^^^^^^^^^^^^^^^^
                x: Int
#^^^^^^^^^^^^^^^^^^^^^^^^^^^^
            }
#^^^^^^^^^^^^
        "
    };
    let ide = Ide::from_fixture(&fixture);
    let snapshot = ide.snapshot();

    for position in fixture.positions() {
        let item = snapshot.resolve_item_at(position).unwrap();
        assert_eq!(item, ItemRes { file: position.file, idx: idx!(1) });
    }
}
