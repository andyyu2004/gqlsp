use std::sync::Arc;

use gqls_base_db::SourceDatabase;
use gqls_parse::{NodeExt, NodeKind, Point, RangeExt};
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
    lower::ItemCtxt::new(data.text).lower(data.tree)
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
            let items = db.items(file);
            for (idx, _) in items.items.iter() {
                let body = db.item_body(ItemRes { file, idx });
                let fields = body.as_deref().and_then(|b| b.fields_slice()).unwrap_or(&[]).iter();
                match res_item.kind {
                    ItemKind::TypeDefinition(_) | ItemKind::TypeExtension(_) => references.extend(
                        fields
                            .filter(|field| field.ty.name() == name)
                            .map(|field| (file, field.ty.range)),
                    ),
                    ItemKind::DirectiveDefinition(_) => references.extend(
                        fields
                            .flat_map(|field| &field.directives)
                            .chain(items.directives(idx).map(Vec::as_slice).unwrap_or(&[]))
                            .filter(|directive| directive.name == name)
                            .map(|directive| (file, directive.range)),
                    ),
                };
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
