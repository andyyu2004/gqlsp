use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tree_sitter::Tree;
use vfs::VfsPath;

#[derive(Debug, Clone)]
pub struct FileData {
    pub text: Arc<str>,
    pub tree: Tree,
}

impl FileData {
    /// The `tree` must be parsed from the given string (or equivalent)
    pub fn new(text: impl AsRef<str>, tree: Tree) -> Self {
        Self { text: Arc::from(text.as_ref()), tree }
    }
}

#[salsa::query_group(SourceDatabaseStorage)]
pub trait SourceDatabase {
    #[salsa::input]
    fn projects(&self) -> Arc<HashMap<String, Arc<HashSet<VfsPath>>>>;

    #[salsa::input]
    fn file_data(&self, path: VfsPath) -> FileData;

    #[salsa::dependencies]
    fn file_tree(&self, path: VfsPath) -> Tree;

    fn file_text(&self, path: VfsPath) -> Arc<str>;

    fn project_files(&self, project: VfsPath) -> Arc<HashSet<VfsPath>>;
}

fn file_tree(db: &dyn SourceDatabase, path: VfsPath) -> Tree {
    db.file_data(path).tree
}

fn file_text(db: &dyn SourceDatabase, path: VfsPath) -> Arc<str> {
    db.file_data(path).text
}

fn project_files(db: &dyn SourceDatabase, project: VfsPath) -> Arc<HashSet<VfsPath>> {
    todo!()
}
