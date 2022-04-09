mod edit;

use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

pub use self::edit::{Change, ChangeKind, Patch, Point, Range};
use gqls_db::{GqlsDatabase, SourceDatabase};
use ropey::Rope;
use vfs::{FileId, Vfs};

#[derive(Default)]
pub struct Ide {
    db: GqlsDatabase,
    vfs: Vfs,
    file_ropes: HashMap<FileId, Rope>,
}

impl Ide {
    pub fn change(&mut self, path: impl AsRef<Path>, kind: ChangeKind) {
        let change = Change::new(self.vfs.intern(path), kind);
        self.apply(&change);
    }

    pub fn apply(&mut self, change: &Change) {
        self.db.request_cancellation();
        let tree = match &change.kind {
            ChangeKind::Patch(patch) => {
                let mut rope = self
                    .file_ropes
                    .get(&change.file)
                    .cloned()
                    .expect("patch on initial edit");
                let edit = patch.apply(&mut rope);
                let text = rope.to_string();
                let mut old = self
                    .file_ropes
                    .insert(change.file, rope)
                    .map(|_| self.db.file_tree(change.file))
                    .unwrap();
                old.edit(&edit);
                gqls_parse::parse(&text, Some(&old))
            }
            ChangeKind::Set(text) => {
                let rope = Rope::from_str(text);
                self.file_ropes.insert(change.file, rope);
                gqls_parse::parse(&text, None)
            }
        };

        self.db.set_file_tree(change.file, tree)
    }
}

#[cfg(test)]
mod tests;
