use gqls_ir::{self as ir, DefDatabase, ItemKind, ItemRes, TypeDefinitionKind};

use crate::{FieldType, Fields, ObjectType, Ty, TyKind};

#[salsa::query_group(TyDatabaseStorage)]
pub trait TyDatabase: DefDatabase {
    fn type_of(&self, res: ItemRes) -> Ty;
    fn lower_type(&self, ty: ir::Ty) -> Ty;
}

fn lower_type(db: &dyn TyDatabase, ty: ir::Ty) -> Ty {
    match &ty.kind {
        ir::TyKind::Named(_) => todo!(),
        ir::TyKind::NonNull(inner) => TyKind::NonNull(db.lower_type(inner.clone())),
        ir::TyKind::List(inner) => TyKind::List(db.lower_type(inner.clone())),
    }
    .intern()
}

fn type_of(db: &dyn TyDatabase, res: ItemRes) -> Ty {
    let item = db.item(res);
    match item.kind {
        ItemKind::TypeDefinition(idx) => {
            let typedef = &db.items(res.file)[idx];
            let body = db.item_body(res).expect("typedef should have a body");
            let kind = match typedef.kind {
                TypeDefinitionKind::Object => TyKind::Object(ObjectType {
                    fields: Fields {
                        fields: body
                            .fields_slice()
                            .unwrap()
                            .iter()
                            .map(|field| FieldType {
                                name: field.name.name(),
                                ty: db.lower_type(field.ty.clone()),
                            })
                            .collect(),
                    },
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
