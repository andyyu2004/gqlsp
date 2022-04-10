use std::sync::Arc;

use gqls_base_db::SourceDatabase;
use vfs::VfsPath;

use crate::Definitions;

#[salsa::query_group(DefDatabaseStorage)]
pub trait DefDatabase: SourceDatabase {
    fn defs(&self, path: VfsPath) -> Arc<Definitions>;
}

fn defs(db: &dyn DefDatabase, path: VfsPath) -> Arc<Definitions> {
    let file_tree = db.file_tree(path);
    todo!()
}
