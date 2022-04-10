use std::sync::Arc;
use tree_sitter::Tree;
use vfs::VfsPath;

#[salsa::query_group(SourceDatabaseStorage)]
pub trait SourceDatabase {
    #[salsa::input]
    fn files(&self) -> Arc<Vec<VfsPath>>;
    #[salsa::input]
    fn file_tree(&self, path: VfsPath) -> Tree;
}
