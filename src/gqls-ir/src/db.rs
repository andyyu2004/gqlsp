use std::sync::Arc;

use gqls_base_db::{FileData, SourceDatabase};
use gqls_parse::{Node, NodeExt, NodeKind};
use vfs::VfsPath;

use crate::{Item, Items, Name, TypeDefinition};

#[salsa::query_group(DefDatabaseStorage)]
pub trait DefDatabase: SourceDatabase {
    fn items(&self, path: VfsPath) -> Arc<Items>;
}

fn items(db: &dyn DefDatabase, path: VfsPath) -> Arc<Items> {
    LowerCtxt { data: db.file_data(path) }.lower()
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
