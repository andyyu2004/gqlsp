use std::sync::Arc;

type FileId = usize;

#[salsa::query_group(SourceDatabaseStorage)]
pub trait SourceDatabase {
    #[salsa::input]
    fn files(&self) -> Arc<Vec<FileId>>;
    #[salsa::input]
    fn file_text(&self, file_id: FileId) -> Arc<String>;
}
