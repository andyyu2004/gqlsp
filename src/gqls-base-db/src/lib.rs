use smallvec::SmallVec;
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

pub type Project = &'static str;

thread_local! {
    static INTERNER: std::cell::RefCell<vfs::Interner<str>> = Default::default();
}

#[salsa::query_group(SourceDatabaseStorage)]
pub trait SourceDatabase {
    #[salsa::input]
    fn projects(&self) -> Arc<HashMap<Project, HashSet<VfsPath>>>;

    #[salsa::input]
    fn file_data(&self, path: VfsPath) -> FileData;

    #[salsa::dependencies]
    fn file_tree(&self, path: VfsPath) -> Tree;

    #[salsa::dependencies]
    fn intern_project(&self, name: String) -> Project;

    fn file_text(&self, path: VfsPath) -> Arc<str>;

    fn file_to_projects(&self) -> Arc<HashMap<VfsPath, SmallVec<[Project; 1]>>>;

    fn projects_of(&self, path: VfsPath) -> SmallVec<[Project; 1]>;

    fn project_files(&self, project: Project) -> HashSet<VfsPath>;
}

fn file_tree(db: &dyn SourceDatabase, path: VfsPath) -> Tree {
    db.file_data(path).tree
}

fn file_text(db: &dyn SourceDatabase, path: VfsPath) -> Arc<str> {
    db.file_data(path).text
}

fn intern_project(_db: &dyn SourceDatabase, name: String) -> Project {
    INTERNER.with(|interner| interner.borrow_mut().intern(name))
}

fn project_files(db: &dyn SourceDatabase, project: Project) -> HashSet<VfsPath> {
    db.projects()[project].clone()
}

fn file_to_projects(db: &dyn SourceDatabase) -> Arc<HashMap<VfsPath, SmallVec<[Project; 1]>>> {
    let mut result = HashMap::<VfsPath, SmallVec<_>>::new();
    for (&project, files) in db.projects().iter() {
        for &file in files.iter() {
            result.entry(file).or_default().push(project);
        }
    }
    Arc::new(result)
}

fn projects_of(db: &dyn SourceDatabase, path: VfsPath) -> SmallVec<[Project; 1]> {
    db.file_to_projects()[&path].clone()
}
