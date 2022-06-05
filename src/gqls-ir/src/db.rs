use std::mem::discriminant;
use std::sync::Arc;
use std::vec;

use gqls_base_db::{InProject, SourceDatabase};
use gqls_syntax::{NodeExt, NodeKind, Position, RangeExt};
use smallvec::smallvec;
use vfs::FileId;

use crate::lower::BodyCtxt;
use crate::*;

#[salsa::query_group(DefDatabaseStorage)]
pub trait DefDatabase: SourceDatabase {
    fn field(&self, res: FieldRes) -> Field;
    fn implementations(&self, interface: InProject<Name>) -> Vec<ItemRes>;
    fn item(&self, res: ItemRes) -> Item;
    fn item_at(&self, position: Position) -> Option<Idx<Item>>;
    fn item_body(&self, res: ItemRes) -> Option<Arc<ItemBody>>;
    fn item_map(&self, file: FileId) -> Arc<ItemMap>;
    fn item_references(&self, res: ItemRes) -> References;
    fn items(&self, file: FileId) -> Arc<Items>;
    fn project_items(&self, project: InProject<()>) -> Arc<ProjectItems>;
    fn name_at(&self, position: Position) -> Option<Name>;
    fn related_files(&self, file: InProject<()>) -> HashSet<FileId>;
    fn references(&self, res: Res) -> References;
    fn resolve(&self, position: Position) -> Option<Res>;
    fn resolve_item(&self, name: InProject<Name>) -> Res;
    fn type_at(&self, position: Position) -> Option<Ty>;
    fn typedef(&self, file: FileId, idx: Idx<TypeDefinition>) -> TypeDefinition;
}

// all files that are in a common project with `file`
fn related_files(db: &dyn DefDatabase, file: InProject<()>) -> HashSet<FileId> {
    db.projects_of(file).iter().flat_map(|project| db.project_files(project)).collect()
}

fn project_items(db: &dyn DefDatabase, file: InProject<()>) -> Arc<ProjectItems> {
    let data = db.related_files(file).iter().map(|&file| (file, db.items(file))).collect();
    Arc::new(data)
}

fn implementations(db: &dyn DefDatabase, interface: InProject<Name>) -> Vec<ItemRes> {
    let mut implementations = vec![];
    for (&file, items) in db.project_items(interface.project()).iter() {
        for (idx, _) in items.iter() {
            if items.implements(idx, &interface.value) {
                implementations.push(ItemRes::new(file, idx));
            }
        }
    }
    implementations
}

fn items(db: &dyn DefDatabase, file: FileId) -> Arc<Items> {
    let data = db.file_data(file);
    lower::ItemCtxt::new(data.text).lower(data.tree)
}

fn item_at(db: &dyn DefDatabase, position: Position) -> Option<Idx<Item>> {
    db.items(position.file)
        .iter()
        .find_map(|(idx, item)| item.range.contains(position.point).then(|| idx))
}

fn item(db: &dyn DefDatabase, res: ItemRes) -> Item {
    db.items(res.file).items[res.value].clone()
}

fn field(db: &dyn DefDatabase, res: FieldRes) -> Field {
    db.item_body(res.item).unwrap().fields().unwrap()[res.idx].clone()
}

fn item_map(db: &dyn DefDatabase, file: FileId) -> Arc<ItemMap> {
    let items = db.items(file);
    let mut map = ItemMap::with_capacity(items.len());
    for (idx, item) in items.iter() {
        map.entry(item.name.clone()).or_default().push(idx);
    }
    Arc::new(map)
}

fn item_body(db: &dyn DefDatabase, res: ItemRes) -> Option<Arc<ItemBody>> {
    let items = db.items(res.file);
    let tree = db.file_tree(res.file);
    let item = &items[res.value];
    let item_node = tree.root_node().named_descendant_for_range(item.range).unwrap();
    let bcx = BodyCtxt::new(db, res.file);
    let body = match item.kind {
        ItemKind::TypeDefinition(_) => bcx.lower_typedef(item_node),
        ItemKind::DirectiveDefinition(_) => return None,
    };
    Some(Arc::new(body))
}

fn name_at(db: &dyn DefDatabase, position: Position) -> Option<Name> {
    let data = db.file_data(position.file);
    let root = data.tree.root_node();
    let node = root.named_node_at(position.point)?;
    match node.kind() {
        NodeKind::NAME | NodeKind::DIRECTIVE_NAME => Some(Name::new(&data.text, node)),
        _ => db.type_at(position).map(|ty| ty.name()),
    }
}

fn type_at(db: &dyn DefDatabase, position: Position) -> Option<Ty> {
    let data = db.file_data(position.file);
    let root = data.tree.root_node();
    let node = root.named_node_at(position.point)?;
    let type_node = match node.kind() {
        NodeKind::TYPE | NodeKind::NON_NULL_TYPE | NodeKind::LIST_TYPE | NodeKind::NAMED_TYPE =>
            node,
        _ => return None,
    };
    BodyCtxt::new(db, position.file).lower_type(type_node)
}

fn resolve(db: &dyn DefDatabase, position: Position) -> Option<Res> {
    let data = db.file_data(position.file);
    let root = data.tree.root_node();
    let node = root.named_node_at(position.point)?;
    if node.kind() != NodeKind::NAME {
        return None;
    }
    let parent = node.parent()?;
    match parent.kind() {
        NodeKind::FIELD_DEFINITION => todo!(),
        NodeKind::DIRECTIVE_DEFINITION
        | NodeKind::OBJECT_TYPE_DEFINITION
        | NodeKind::OBJECT_TYPE_EXTENSION
        | NodeKind::ENUM_TYPE_DEFINITION
        | NodeKind::SCALAR_TYPE_DEFINITION
        | NodeKind::INPUT_OBJECT_TYPE_DEFINITION
        | NodeKind::INTERFACE_TYPE_DEFINITION => {
            let idx = db
                .items(position.file)
                .items
                .iter()
                .find_map(|(idx, item)| (item.range == parent.range()).then(|| idx))
                .expect("item not found");
            Some(Res::Item(smallvec![ItemRes::new(position.file, idx)]))
        }
        // TODO
        _ => None,
    }
}

fn resolve_item(db: &dyn DefDatabase, name: InProject<Name>) -> Res {
    match name.as_str() {
        "ID" => return Res::Builtin(BuiltinScalar::ID),
        "Float" => return Res::Builtin(BuiltinScalar::Float),
        "Int" => return Res::Builtin(BuiltinScalar::Int),
        "Boolean" => return Res::Builtin(BuiltinScalar::Boolean),
        "String" => return Res::Builtin(BuiltinScalar::String),
        _ => (),
    }

    let mut resolutions = smallvec![];
    for project in db.projects_of(name.project()) {
        for file in db.project_files(project).iter() {
            let map = db.item_map(file);
            if let Some(items) = map.get(&name.value) {
                for &idx in items {
                    resolutions.push(ItemRes::new(file, idx));
                }
            }
        }
    }

    if resolutions.is_empty() { Res::Err } else { Res::Item(resolutions) }
}

fn item_references(db: &dyn DefDatabase, res: ItemRes) -> References {
    let mut references = vec![];
    let res_item = db.item(res);
    let name = res_item.name;
    for (&file, items) in db.project_items(res.project()).iter() {
        for (idx, item) in items.iter() {
            if item.name == name && discriminant(&item.kind) == discriminant(&res_item.kind) {
                references.push((file, item.name.range));
            }

            let body = db.item_body(ItemRes::new(file, idx));

            if let Some(ItemBodyKind::UnionTypeDefinition(union)) = body.as_ref().map(|b| &b.kind) {
                references.extend(
                    union.types.iter().filter(|ty| ty.name() == name).map(|ty| (file, ty.range)),
                )
            }

            let fields = body.as_deref().and_then(|b| b.fields_slice()).unwrap_or(&[]).iter();
            match res_item.kind {
                ItemKind::TypeDefinition(_) => references.extend(
                    fields
                        .filter(|field| field.ty.name() == name.clone())
                        .map(|field| (file, field.ty.name().range)),
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
    references
}

fn references(db: &dyn DefDatabase, res: Res) -> References {
    match res {
        Res::Item(resolutions) => match resolutions[..] {
            [] => vec![],
            [res, ..] => db.item_references(res),
        },
        Res::Field(_) => todo!(),
        Res::Builtin(_) | Res::Err => vec![],
    }
}

fn typedef(db: &dyn DefDatabase, file: FileId, idx: Idx<TypeDefinition>) -> TypeDefinition {
    db.items(file).typedefs[idx].clone()
}
