use crate::TyDatabaseStorage;
use gqls_fixture::fixture;
use gqls_ir::{DefDatabaseStorage, SourceDatabaseStorage};
use testing::TestDatabaseExt;

#[salsa::database(SourceDatabaseStorage, DefDatabaseStorage, TyDatabaseStorage)]
#[derive(Default)]
pub(crate) struct TestDB {
    storage: salsa::Storage<TestDB>,
}

impl salsa::Database for TestDB {
}

#[test]
fn test_lower_ty() {
    let fixture = fixture! {};
    let db = TestDB::from_fixture(&fixture);
}
