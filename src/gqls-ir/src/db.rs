use std::sync::Arc;

use gqls_base_db::{FileData, SourceDatabase};
use gqls_parse::{Node, NodeExt, NodeKind};
use smallvec::smallvec;
use vfs::FileId;

use crate::{
    DirectiveDefinition, Item, ItemKind, ItemMap, Items, Name, Res, Resolutions, TypeDefinition, TypeExtension
};

#[salsa::query_group(DefDatabaseStorage)]
pub trait DefDatabase: SourceDatabase {
    fn items(&self, file: FileId) -> Arc<Items>;
    fn item(&self, res: Res) -> Item;
    fn item_map(&self, file: FileId) -> Arc<ItemMap>;
    fn resolve(&self, file: FileId, name: Name) -> Resolutions;
}

fn items(db: &dyn DefDatabase, file: FileId) -> Arc<Items> {
    LowerCtxt { data: db.file_data(file) }.lower()
}

fn item(db: &dyn DefDatabase, res: Res) -> Item {
    db.items(res.file).items[res.idx].clone()
}

fn item_map(db: &dyn DefDatabase, file: FileId) -> Arc<ItemMap> {
    let items = db.items(file);
    let mut map = ItemMap::with_capacity(items.items.len());
    for (idx, item) in items.items.iter() {
        map.entry(item.name()).or_default().push(idx);
    }
    Arc::new(map)
}

fn resolve(db: &dyn DefDatabase, file: FileId, name: Name) -> Resolutions {
    let mut resolutions = smallvec![];
    for project in db.projects_of(file) {
        for path in db.project_files(project).iter() {
            let map = db.item_map(path);
            if let Some(items) = map.get(&name) {
                for &idx in items {
                    resolutions.push(Res { file, idx });
                }
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
            .filter(|node| !node.is_extra())
            .filter_map(|node| self.lower_item(node))
            .collect();
        Arc::new(Items { items })
    }

    fn lower_item(&self, node: Node<'_>) -> Option<Item> {
        assert_eq!(node.kind(), NodeKind::ITEM);
        let def = node.sole_named_child();
        let kind = match def.kind() {
            NodeKind::TYPE_DEFINITION => {
                let typedef = def.sole_named_child();
                let name = match typedef.kind() {
                    NodeKind::OBJECT_TYPE_DEFINITION
                    | NodeKind::INTERFACE_TYPE_DEFINITION
                    | NodeKind::SCALAR_TYPE_DEFINITION
                    | NodeKind::ENUM_TYPE_DEFINITION
                    | NodeKind::UNION_TYPE_DEFINITION
                    | NodeKind::INPUT_OBJECT_TYPE_DEFINITION => typedef.find_name_node()?,
                    _ =>
                        unreachable!("invalid node kind for type definition: {:?}", typedef.kind()),
                };
                ItemKind::TypeDefinition(TypeDefinition {
                    name: Name::new(name.text(&self.data.text)),
                })
            }
            NodeKind::TYPE_EXTENSION => {
                let type_ext = def.sole_named_child();
                let name = match type_ext.kind() {
                    NodeKind::OBJECT_TYPE_EXTENSION => type_ext.find_name_node()?,
                    _ => return None,
                };
                ItemKind::TypeExtension(TypeExtension {
                    name: Name::new(name.text(&self.data.text)),
                })
            }
            NodeKind::DIRECTIVE_DEFINITION => {
                let name = def.find_name_node()?;
                ItemKind::DirectiveDefinition(DirectiveDefinition {
                    name: Name::new(name.text(&self.data.text)),
                })
            }
            // TODO
            _ => return None,
        };
        Some(Item { range: def.range(), kind })
    }
}
