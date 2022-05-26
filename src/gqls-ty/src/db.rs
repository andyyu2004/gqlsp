use gqls_ir::{self as ir, DefDatabase, ItemKind, ItemRes, TypeDefinitionKind};
use ir::{FieldRes, InProject, Res};

use crate::{FieldType, FieldTypes, ImplError, InterfaceType, ObjectType, Ty, TyKind};

#[salsa::query_group(TyDatabaseStorage)]
pub trait TyDatabase: DefDatabase {
    fn type_of(&self, res: Res) -> Ty;
    fn type_of_item(&self, res: ItemRes) -> Ty;
    fn field_types_of(&self, res: ItemRes) -> FieldTypes;
    fn type_of_field(&self, res: FieldRes) -> Ty;
    fn lower_type(&self, ty: InProject<ir::Ty>) -> Ty;
    fn implements_interface(&self, obj: ObjectType, interface: InterfaceType) -> Option<ImplError>;
}

fn implements_interface(
    _db: &dyn TyDatabase,
    _obj: ObjectType,
    _interface: InterfaceType,
) -> Option<ImplError> {
    todo!()
}

fn lower_type(db: &dyn TyDatabase, ty: InProject<ir::Ty>) -> Ty {
    match &ty.kind {
        ir::TyKind::Named(name, _) => match name.as_str() {
            "ID" => TyKind::ID,
            "Boolean" => TyKind::Boolean,
            "Float" => TyKind::Float,
            "Int" => TyKind::Int,
            "String" => TyKind::String,
            _ => {
                let item = db.resolve_item(ty.with_value(name.clone()));
                todo!();
            }
        },
        ir::TyKind::NonNull(inner) => TyKind::NonNull(db.lower_type(ty.with_value(inner.clone()))),
        ir::TyKind::List(inner) => TyKind::List(db.lower_type(ty.with_value(inner.clone()))),
        ir::TyKind::Err(_) => TyKind::Err,
    }
    .intern()
}

fn type_of(db: &dyn TyDatabase, res: Res) -> Ty {
    match res {
        Res::Item(res) => db.type_of_item(res),
        Res::Field(res) => db.type_of_field(res),
    }
}

fn type_of_field(db: &dyn TyDatabase, res: FieldRes) -> Ty {
    let field = db.field(res);
    db.lower_type(InProject::new(res.item.file, field.ty))
}

fn field_types_of(db: &dyn TyDatabase, res: ItemRes) -> FieldTypes {
    let body = db.item_body(res).expect("queried `field_types` on item with no fields");
    FieldTypes {
        fields: body
            .fields()
            .unwrap()
            .iter()
            .map(|(idx, field)| FieldType { name: field.name.name(), res: FieldRes::new(res, idx) })
            .collect(),
    }
}

fn type_of_item(db: &dyn TyDatabase, res: ItemRes) -> Ty {
    // FIXME aggregate over all the extensions
    // if there's any ambiguities/duplicates/whatever just return TyKind::Err
    let item = db.item(res);
    match item.kind {
        ItemKind::TypeDefinition(idx) => {
            let typedef = &db.items(res.file)[idx];
            let _body = db.item_body(res).expect("typedef should have a body");
            let kind = match typedef.kind {
                TypeDefinitionKind::Object => TyKind::Object(ObjectType {
                    name: item.name.name(),
                    fields: db.field_types_of(res),
                }),
                TypeDefinitionKind::Interface => todo!(),
                TypeDefinitionKind::Input => todo!(),
                TypeDefinitionKind::Scalar => todo!(),
                TypeDefinitionKind::Enum => todo!(),
                TypeDefinitionKind::Union => todo!(),
            };
            kind.intern()
        }
        // can model directives as having a function type maybe?
        ItemKind::DirectiveDefinition(_) => todo!("typeof directive definition"),
    }
}
