use std::sync::Arc;

use gqls_base_db::{FileData, SourceDatabase};
use gqls_parse::{Node, NodeExt, NodeKind};
use smallvec::smallvec;
use vfs::VfsPath;

use crate::{Item, ItemMap, Items, Name, Res, Resolutions, TypeDefinition};

#[salsa::query_group(DefDatabaseStorage)]
pub trait DefDatabase: SourceDatabase {
    fn items(&self, path: VfsPath) -> Arc<Items>;
    fn item_map(&self, path: VfsPath) -> Arc<ItemMap>;
    fn resolve(&self, name: Name) -> Resolutions;
}

fn items(db: &dyn DefDatabase, path: VfsPath) -> Arc<Items> {
    LowerCtxt { data: db.file_data(path) }.lower()
}

fn item_map(db: &dyn DefDatabase, path: VfsPath) -> Arc<ItemMap> {
    let items = db.items(path);
    let mut map = ItemMap::with_capacity(items.items.len());
    for (idx, item) in items.items.iter() {
        map.entry(item.name()).or_default().push(idx);
    }
    Arc::new(map)
}

fn resolve(db: &dyn DefDatabase, name: Name) -> Resolutions {
    let mut resolutions = smallvec![];
    for path in db.files().iter() {
        let map = db.item_map(path);
        if let Some(items) = map.get(&name) {
            for &idx in items {
                resolutions.push(Res { path, idx });
            }
        }
    }
    resolutions
}

struct LowerCtxt {
    data: FileData,
}

impl LowerCtxt {
    fn lower(&self) -> Arc<Items> {
        let node = self.data.tree.root_node();
        let items = node
            .named_children(&mut node.walk())
            .filter_map(|node| self.lower_item(node))
            .collect();
        Arc::new(Items { items })
    }

    fn lower_item(&self, node: Node<'_>) -> Option<Item> {
        assert_eq!(node.kind(), NodeKind::ITEM);
        let node = node.sole_named_child();
        let item = match node.kind() {
            NodeKind::TYPE_DEFINITION => {
                let node = node.sole_named_child();
                let name = match node.kind() {
                    NodeKind::OBJECT_TYPE_DEFINITION
                    | NodeKind::INTERFACE_TYPE_DEFINITION
                    | NodeKind::SCALAR_TYPE_DEFINITION
                    | NodeKind::UNION_TYPE_DEFINITION
                    | NodeKind::INPUT_OBJECT_TYPE_DEFINITION => node
                        .named_children(&mut node.walk())
                        .find(|node| node.kind() == NodeKind::NAME),
                    _ => unreachable!("invalid node kind for type definition: {:?}", node.kind()),
                }?;
                Item::TypeDefinition(TypeDefinition { name: Name::new(name.text(&self.data.text)) })
            }
            _ => todo!(),
        };
        Some(item)
    }
}
