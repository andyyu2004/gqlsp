use smallvec::SmallVec;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tree_sitter::Tree;
use vfs::FileId;

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

pub type Project = &'static str;

thread_local! {
    static INTERNER: std::cell::RefCell<vfs::Interner<str>> = Default::default();
}

#[salsa::query_group(SourceDatabaseStorage)]
pub trait SourceDatabase {
    #[salsa::input]
    fn projects(&self) -> Arc<HashMap<Project, HashSet<FileId>>>;

    #[salsa::input]
    fn file_data(&self, file: FileId) -> FileData;

    #[salsa::dependencies]
    fn file_tree(&self, file: FileId) -> Tree;

    #[salsa::dependencies]
    fn intern_project(&self, name: String) -> Project;

    fn file_text(&self, file: FileId) -> Arc<str>;

    fn file_to_projects(&self) -> Arc<HashMap<FileId, SmallVec<[Project; 1]>>>;

    fn projects_of(&self, file: FileId) -> SmallVec<[Project; 1]>;

    fn project_files(&self, project: Project) -> HashSet<FileId>;
}

fn file_tree(db: &dyn SourceDatabase, file: FileId) -> Tree {
    db.file_data(file).tree
}

fn file_text(db: &dyn SourceDatabase, file: FileId) -> Arc<str> {
    db.file_data(file).text
}

fn intern_project(_db: &dyn SourceDatabase, name: String) -> Project {
    INTERNER.with(|interner| interner.borrow_mut().intern(name))
}

fn project_files(db: &dyn SourceDatabase, project: Project) -> HashSet<FileId> {
    db.projects()[project].clone()
}

fn file_to_projects(db: &dyn SourceDatabase) -> Arc<HashMap<FileId, SmallVec<[Project; 1]>>> {
    let mut result = HashMap::<FileId, SmallVec<_>>::new();
    for (&project, files) in db.projects().iter() {
        for &file in files.iter() {
            result.entry(file).or_default().push(project);
        }
    }
    Arc::new(result)
}

fn projects_of(db: &dyn SourceDatabase, file: FileId) -> SmallVec<[Project; 1]> {
    db.file_to_projects()[&file].clone()
}
