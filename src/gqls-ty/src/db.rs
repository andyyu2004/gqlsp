use gqls_ir::{self as ir, DefDatabase, ItemKind, ItemRes, TypeDefinitionKind};
use ir::{FieldRes, Res, Value};

use crate::*;

#[salsa::query_group(TyDatabaseStorage)]
pub trait TyDatabase: DefDatabase {
    fn is_subtype(&self, ty: Ty, of: Ty) -> bool;
    fn ensure_has_type(&self, value: Value, ty: Ty) -> Result<(), TypeMismatch>;
    fn has_type(&self, value: Value, ty: Ty) -> bool;
    fn type_of_res(&self, res: Res) -> Ty;
    fn type_of_item(&self, res: ItemRes) -> Ty;
    fn field_types_of(&self, res: ItemRes) -> FieldTypes;
    fn type_of_field(&self, res: FieldRes) -> Ty;
    fn lower_type(&self, ty: ir::Ty) -> Ty;
    fn implements_interface(&self, obj: ObjectType, interface: InterfaceType) -> Option<ImplError>;
}

fn is_subtype(_db: &dyn TyDatabase, ty: Ty, of: Ty) -> bool {
    // TODO
    ty == of
}

fn ensure_has_type(db: &dyn TyDatabase, value: Value, ty: Ty) -> Result<(), TypeMismatch> {
    match (value, &ty.kind) {
        (_, TyKind::Err)
        | (Value::Boolean(_), TyKind::Boolean)
        | (Value::Float(_), TyKind::Float)
        | (Value::Int(_), TyKind::Int)
        | (Value::String(_), TyKind::String)
        | (Value::String(_), TyKind::ID) => Ok(()),
        (Value::Enum(variant), TyKind::Enum(e)) if e.variants.contains(&variant) => Ok(()),
        (Value::Enum(variant), TyKind::Enum(e)) =>
            Err(TypeMismatch::InvalidVariant(variant, e.clone())),
        (Value::Null, kind) => match kind {
            TyKind::NonNull(_) => Err(TypeMismatch::InvalidNull),
            _ => Ok(()),
        },
        (value, TyKind::NonNull(ty)) => db.ensure_has_type(value, ty.clone()),
        (Value::List(values), TyKind::List(ty)) => {
            match values
                .iter()
                .map(|value| db.ensure_has_type(value.clone(), ty.clone()))
                .find_map(|x| x.err())
            {
                Some(err) => Err(err),
                None => Ok(()),
            }
        }
        (Value::Object(obj), TyKind::Input(input)) => {
            let fields = &input.fields.fields;
            for name in obj.keys() {
                if !fields.iter().any(|field| field.name == name.name()) {
                    return Err(TypeMismatch::ExtraneousField(name.name(), ty.clone()));
                }
            }
            match fields
                .iter()
                .map(|field| match obj.get(&field.name) {
                    Some(value) =>
                        db.ensure_has_type(value.clone(), db.type_of_field(field.res.clone())),
                    None => match db.type_of_field(field.res).is_nullable() {
                        true => Ok(()),
                        false => Err(TypeMismatch::InvalidNullField(field.name.clone())),
                    },
                })
                .find_map(|x| x.err())
            {
                Some(err) => Err(err),
                None => Ok(()),
            }
        }
        (value, _) => Err(TypeMismatch::Obvious(value, ty)),
    }
}

fn has_type(db: &dyn TyDatabase, value: Value, ty: Ty) -> bool {
    db.ensure_has_type(value, ty).is_ok()
}

fn implements_interface(
    _db: &dyn TyDatabase,
    _obj: ObjectType,
    _interface: InterfaceType,
) -> Option<ImplError> {
    todo!()
}

fn lower_type(db: &dyn TyDatabase, ty: ir::Ty) -> Ty {
    match ty.kind.clone() {
        ir::TyKind::Named(_, res) => return db.type_of_res(res),
        ir::TyKind::NonNull(inner) => TyKind::NonNull(db.lower_type(inner)),
        ir::TyKind::List(inner) => TyKind::List(db.lower_type(inner)),
        ir::TyKind::Err(_) => TyKind::Err,
    }
    .intern()
}

fn type_of_res(db: &dyn TyDatabase, res: Res) -> Ty {
    match res {
        Res::Item(res) => match res[..] {
            [] => unreachable!("should be an `ir::TyKind::Err` if unresolved"),
            [res, ..] => db.type_of_item(res),
            // TODO handle multiple res?
        },
        Res::Builtin(builtin) => TyKind::from(builtin).intern(),
        Res::Err => TyKind::Err.intern(),
    }
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

fn type_of_item(db: &dyn TyDatabase, res: ItemRes) -> Ty {
    // FIXME aggregate over all the extensions
    // if there's any ambiguities/duplicates/whatever just return TyKind::Err
    let item = db.item(res);
    match item.kind {
        ItemKind::TypeDefinition(idx) => {
            let typedef = &db.items(res.file)[idx];
            let body = db.item_body(res).expect("typedef should have a body");
            let name = item.name.name();
            let kind = match &typedef.kind {
                TypeDefinitionKind::Object => TyKind::Object(ObjectType {
                    name: item.name.name(),
                    fields: db.field_types_of(res),
                }),
                TypeDefinitionKind::Interface =>
                    TyKind::Interface(InterfaceType { name, fields: db.field_types_of(res) }),
                TypeDefinitionKind::Input =>
                    TyKind::Input(InputObjectType { name, fields: db.field_types_of(res) }),
                TypeDefinitionKind::Scalar => TyKind::Scalar(ScalarType { name }),
                TypeDefinitionKind::Enum => TyKind::Enum(EnumType {
                    name,
                    variants: body
                        .as_enum()
                        .variants
                        .iter()
                        .map(|v| v.name.name().as_str().into())
                        .collect(),
                }),
                TypeDefinitionKind::Union => TyKind::Union(UnionType {
                    name,
                    types: body
                        .as_union()
                        .types
                        .iter()
                        .map(|ty| db.lower_type(ty.clone()))
                        .collect(),
                }),
            };
            kind.intern()
        }
        // can model directives as having a function type maybe?
        ItemKind::DirectiveDefinition(_) => todo!("typeof directive definition"),
    }
}
