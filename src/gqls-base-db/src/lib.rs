use ropey::Rope;
use std::sync::Arc;
use vfs::FileId;

#[salsa::query_group(SourceDatabaseStorage)]
pub trait SourceDatabase {
    #[salsa::input]
    fn files(&self) -> Arc<Vec<FileId>>;
    #[salsa::input]
    fn file_rope(&self, file_id: FileId) -> Rope;
}
