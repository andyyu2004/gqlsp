use crate::TyDatabaseStorage;
use gqls_ir::{DefDatabaseStorage, SourceDatabaseStorage};
use testing::setup_db;

#[salsa::database(SourceDatabaseStorage, DefDatabaseStorage, TyDatabaseStorage)]
#[derive(Default)]
pub(crate) struct TestDB {
    storage: salsa::Storage<TestDB>,
}

impl salsa::Database for TestDB {
}

#[test]
fn test_lower_ty() {
    let db = TestDB::default();
}
