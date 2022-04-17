use std::sync::Arc;

use gqls_base_db::SourceDatabase;
use gqls_parse::{Node, NodeExt, NodeKind, Point, RangeExt, Tree};
use smallvec::smallvec;
use vfs::FileId;

use crate::lower::BodyCtxt;
use crate::*;

#[salsa::query_group(DefDatabaseStorage)]
pub trait DefDatabase: SourceDatabase {
    fn items(&self, file: FileId) -> Arc<Items>;
    fn item(&self, res: ItemRes) -> Item;
    fn item_at(&self, file: FileId, at: Point) -> Option<Idx<Item>>;
    fn item_map(&self, file: FileId) -> Arc<ItemMap>;
    fn item_body(&self, res: ItemRes) -> Option<Arc<ItemBody>>;
    fn resolve_item(&self, file: FileId, name: Name) -> ItemResolutions;
    fn field(&self, res: FieldRes) -> Field;
    fn references(&self, file: FileId, name: Name) -> References;
}

fn items(db: &dyn DefDatabase, file: FileId) -> Arc<Items> {
    let data = db.file_data(file);
    LowerCtxt {
        text: data.text,
        types: Default::default(),
        directives: Default::default(),
        type_exts: Default::default(),
    }
    .lower(data.tree)
}

fn item_at(db: &dyn DefDatabase, file: FileId, at: Point) -> Option<Idx<Item>> {
    db.items(file).items.iter().find_map(|(idx, item)| item.range.contains(at).then(|| idx))
}

fn item(db: &dyn DefDatabase, res: ItemRes) -> Item {
    db.items(res.file).items[res.idx]
}

fn field(db: &dyn DefDatabase, res: FieldRes) -> Field {
    db.item_body(res.item).unwrap().fields().unwrap()[res.idx].clone()
}

fn item_map(db: &dyn DefDatabase, file: FileId) -> Arc<ItemMap> {
    let items = db.items(file);
    let mut map = ItemMap::with_capacity(items.items.len());
    for (idx, item) in items.items.iter() {
        map.entry(items.name(item)).or_default().push(idx);
    }
    Arc::new(map)
}

fn item_body(db: &dyn DefDatabase, res: ItemRes) -> Option<Arc<ItemBody>> {
    let items = db.items(res.file);
    let tree = db.file_tree(res.file);
    let item = items.items[res.idx];
    let item_node = tree.root_node().named_descendant_for_range(item.range).unwrap();
    let bcx = BodyCtxt::new(db.file_text(res.file));
    let body = match item.kind {
        ItemKind::TypeDefinition(_) => bcx.lower_typedef(item_node),
        ItemKind::TypeExtension(_) => bcx.lower_type_ext(item_node),
        ItemKind::DirectiveDefinition(_) => return None,
    };
    Some(Arc::new(body))
}

fn resolve_item(db: &dyn DefDatabase, file: FileId, name: Name) -> ItemResolutions {
    let mut resolutions = smallvec![];
    for project in db.projects_of(file) {
        for file in db.project_files(project).iter() {
            let map = db.item_map(file);
            if let Some(items) = map.get(&name) {
                for &idx in items {
                    resolutions.push(ItemRes { file, idx });
                }
            }
        }
    }
    resolutions
}

fn references(db: &dyn DefDatabase, file: FileId, name: Name) -> References {
    let mut references = vec![];
    for project in db.projects_of(file) {
        for &file in db.project_files(project).iter() {
            for (idx, _) in db.items(file).items.iter() {
                let body = db.item_body(ItemRes { file, idx });
                let fields = match body.as_deref().and_then(|b| b.fields()) {
                    Some(fields) => fields,
                    None => continue,
                };

                fields
                    .iter()
                    .map(|(_, field)| field)
                    .filter(|field| field.ty.name() == name)
                    .for_each(|field| references.push((file, field.ty.range)))
            }
        }
    }
    references
}

struct LowerCtxt {
    text: Arc<str>,
    types: Arena<TypeDefinition>,
    directives: Arena<DirectiveDefinition>,
    type_exts: Arena<TypeExtension>,
}

impl LowerCtxt {
    fn lower(mut self, tree: Tree) -> Arc<Items> {
        let node = tree.root_node();
        let items = node
            .relevant_children(&mut node.walk())
            .filter_map(|node| self.lower_item(node))
            .collect();
        Arc::new(Items {
            items,
            types: self.types,
            directives: self.directives,
            type_exts: self.type_exts,
        })
    }

    fn lower_item(&mut self, node: Node<'_>) -> Option<Item> {
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
                    | NodeKind::INPUT_OBJECT_TYPE_DEFINITION => typedef.name_node()?,
                    _ =>
                        unreachable!("invalid node kind for type definition: {:?}", typedef.kind()),
                };
                ItemKind::TypeDefinition(
                    self.types.alloc(TypeDefinition { name: Name::new(name.text(&self.text)) }),
                )
            }
            NodeKind::TYPE_EXTENSION => {
                let type_ext = def.sole_named_child();
                let name = match type_ext.kind() {
                    NodeKind::OBJECT_TYPE_EXTENSION => type_ext.name_node()?,
                    _ => return None,
                };
                ItemKind::TypeExtension(
                    self.type_exts.alloc(TypeExtension { name: Name::new(name.text(&self.text)) }),
                )
            }
            NodeKind::DIRECTIVE_DEFINITION => {
                let name = def.name_node()?;
                ItemKind::DirectiveDefinition(
                    self.directives
                        .alloc(DirectiveDefinition { name: Name::new(name.text(&self.text)) }),
                )
            }
            // TODO
            _ => return None,
        };
        Some(Item { range: def.range(), kind })
    }
}
