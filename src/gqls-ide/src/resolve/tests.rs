use gqls_fixture::fixture;
use gqls_ir::ItemRes;

use crate::tests::idx;
use crate::Ide;

#[test]
fn test_resolve_item_at() {
    let mut ide = Ide::default();
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
    ide.setup_fixture(&fixture);
    let snapshot = ide.snapshot();

    for (file, point) in fixture.all_points() {
        let item = snapshot.resolve_item_at(file, point).unwrap();
        assert_eq!(item, ItemRes { file, idx: idx!(1) });
    }
}
