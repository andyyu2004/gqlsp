use gqls_ir::{self as ir, DefDatabase, ItemKind, ItemRes, TypeDefinitionKind};
use ir::FieldRes;

use crate::{FieldType, FieldTypes, ObjectType, Ty, TyKind};

#[salsa::query_group(TyDatabaseStorage)]
pub trait TyDatabase: DefDatabase {
    fn type_of(&self, res: ItemRes) -> Ty;
    fn field_types_of(&self, res: ItemRes) -> FieldTypes;
    fn type_of_field(&self, res: FieldRes) -> Ty;
    fn lower_type(&self, ty: ir::Ty) -> Ty;
}

fn lower_type(db: &dyn TyDatabase, ty: ir::Ty) -> Ty {
    match &ty.kind {
        ir::TyKind::Named(name) => match name.as_str() {
            "ID" => TyKind::Id,
            "Boolean" => TyKind::Boolean,
            "Float" => TyKind::Float,
            "Int" => TyKind::Int,
            "String" => TyKind::String,
            _ => todo!(),
        },
        ir::TyKind::NonNull(inner) => TyKind::NonNull(db.lower_type(inner.clone())),
        ir::TyKind::List(inner) => TyKind::List(db.lower_type(inner.clone())),
    }
    .intern()
}

fn type_of_field(db: &dyn TyDatabase, res: FieldRes) -> Ty {
    let field = db.field(res);
    db.lower_type(field.ty)
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

fn type_of(db: &dyn TyDatabase, res: ItemRes) -> Ty {
    let item = db.item(res);
    match item.kind {
        ItemKind::TypeDefinition(idx) => {
            let typedef = &db.items(res.file)[idx];
            let body = db.item_body(res).expect("typedef should have a body");
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
