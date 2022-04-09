use std::sync::Arc;
use tree_sitter::Tree;
use vfs::FileId;

#[salsa::query_group(SourceDatabaseStorage)]
pub trait SourceDatabase {
    #[salsa::input]
    fn files(&self) -> Arc<Vec<FileId>>;
    #[salsa::input]
    fn file_tree(&self, file_id: FileId) -> Arc<Tree>;
}
