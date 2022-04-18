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
    fn name_at(&self, file: FileId, at: Point) -> Option<Name>;
    fn field(&self, res: FieldRes) -> Field;
    fn references(&self, res: Res) -> References;
    fn item_references(&self, res: ItemRes) -> References;
    fn resolve(&self, file: FileId, at: Point) -> Option<Res>;
    fn resolve_item(&self, file: FileId, name: Name) -> ItemResolutions;
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
    db.items(res.file).items[res.idx].clone()
}

fn field(db: &dyn DefDatabase, res: FieldRes) -> Field {
    db.item_body(res.item).unwrap().fields().unwrap()[res.idx].clone()
}

fn item_map(db: &dyn DefDatabase, file: FileId) -> Arc<ItemMap> {
    let items = db.items(file);
    let mut map = ItemMap::with_capacity(items.items.len());
    for (idx, item) in items.items.iter() {
        map.entry(item.name.clone()).or_default().push(idx);
    }
    Arc::new(map)
}

fn item_body(db: &dyn DefDatabase, res: ItemRes) -> Option<Arc<ItemBody>> {
    let items = db.items(res.file);
    let tree = db.file_tree(res.file);
    let item = &items.items[res.idx];
    let item_node = tree.root_node().named_descendant_for_range(item.range).unwrap();
    let bcx = BodyCtxt::new(db.file_text(res.file));
    let body = match item.kind {
        ItemKind::TypeDefinition(_) => bcx.lower_typedef(item_node),
        ItemKind::TypeExtension(_) => bcx.lower_type_ext(item_node),
        ItemKind::DirectiveDefinition(_) => return None,
    };
    Some(Arc::new(body))
}

fn name_at(db: &dyn DefDatabase, file: FileId, at: Point) -> Option<Name> {
    let data = db.file_data(file);
    let root = data.tree.root_node();
    let node = root.named_node_at(at)?;
    (node.kind() == NodeKind::NAME).then(|| Name::new(node.text(&data.text)))
}

fn resolve(db: &dyn DefDatabase, file: FileId, at: Point) -> Option<Res> {
    let data = db.file_data(file);
    let root = data.tree.root_node();
    let node = root.named_node_at(at)?;
    if node.kind() != NodeKind::NAME {
        return None;
    }
    let parent = node.parent()?;
    match parent.kind() {
        NodeKind::FIELD_DEFINITION => todo!(),
        NodeKind::DIRECTIVE_DEFINITION
        | NodeKind::OBJECT_TYPE_DEFINITION
        | NodeKind::OBJECT_TYPE_EXTENSION
        | NodeKind::SCALAR_TYPE_DEFINITION
        | NodeKind::INTERFACE_TYPE_DEFINITION => {
            let idx = db
                .items(file)
                .items
                .iter()
                .find_map(|(idx, item)| (item.range == parent.range()).then(|| idx))
                .expect("item not found");
            Some(Res::Item(ItemRes { file, idx }))
        }
        // TODO
        _ => return None,
    }
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

fn item_references(db: &dyn DefDatabase, res: ItemRes) -> References {
    let mut references = vec![];
    let res_item = db.item(res);
    let name = res_item.name;
    for project in db.projects_of(res.file) {
        for &file in db.project_files(project).iter() {
            for (idx, _item) in db.items(file).items.iter() {
                let body = db.item_body(ItemRes { file, idx });
                let fields = match body.as_deref().and_then(|b| b.fields()) {
                    Some(fields) => fields,
                    None => continue,
                };
                match res_item.kind {
                    ItemKind::TypeDefinition(_) | ItemKind::TypeExtension(_) => fields
                        .iter()
                        .map(|(_, field)| field)
                        .filter(|field| field.ty.name() == name)
                        .for_each(|field| references.push((file, field.ty.range))),
                    ItemKind::DirectiveDefinition(_) => todo!(),
                }
            }
        }
    }
    references
}

fn references(db: &dyn DefDatabase, res: Res) -> References {
    match res {
        Res::Item(item) => db.item_references(item),
        Res::Field(_) => todo!(),
    }
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
        let (name, kind) = match def.kind() {
            NodeKind::TYPE_DEFINITION => {
                let typedef = def.sole_named_child();
                let name_node = match typedef.kind() {
                    NodeKind::OBJECT_TYPE_DEFINITION
                    | NodeKind::INTERFACE_TYPE_DEFINITION
                    | NodeKind::SCALAR_TYPE_DEFINITION
                    | NodeKind::ENUM_TYPE_DEFINITION
                    | NodeKind::UNION_TYPE_DEFINITION
                    | NodeKind::INPUT_OBJECT_TYPE_DEFINITION => typedef.name_node()?,
                    _ =>
                        unreachable!("invalid node kind for type definition: {:?}", typedef.kind()),
                };
                let name = Name::new(name_node.text(&self.text));
                let directives = self.lower_directives_of(typedef);
                (name, ItemKind::TypeDefinition(self.types.alloc(TypeDefinition { directives })))
            }
            NodeKind::TYPE_EXTENSION => {
                let type_ext = def.sole_named_child();
                let name_node = match type_ext.kind() {
                    NodeKind::OBJECT_TYPE_EXTENSION => type_ext.name_node()?,
                    _ => return None,
                };
                let name = Name::new(name_node.text(&self.text));
                let directives = self.lower_directives_of(type_ext);
                (name, ItemKind::TypeExtension(self.type_exts.alloc(TypeExtension { directives })))
            }
            NodeKind::DIRECTIVE_DEFINITION => {
                let name = Name::new(def.name_node()?.text(&self.text));
                (name, ItemKind::DirectiveDefinition(self.directives.alloc(DirectiveDefinition {})))
            }
            // TODO
            _ => return None,
        };
        Some(Item { range: def.range(), name, kind })
    }

    fn lower_directives_of(&mut self, node: Node<'_>) -> Directives {
        node.child_of_kind(NodeKind::DIRECTIVES)
            .map(|node| self.lower_directives(node))
            .unwrap_or_default()
    }

    fn lower_directives(&mut self, node: Node<'_>) -> Directives {
        assert_eq!(node.kind(), NodeKind::DIRECTIVES);
        node.children_of_kind(&mut node.walk(), NodeKind::DIRECTIVE)
            .filter_map(|node| self.lower_directive(node))
            .collect()
    }

    fn lower_directive(&mut self, node: Node<'_>) -> Option<Directive> {
        assert_eq!(node.kind(), NodeKind::DIRECTIVE);
        // TODO arguments
        let name = Name::new(node.name_node()?.text(&self.text));
        Some(Directive { name })
    }
}
