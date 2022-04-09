mod edit;

use std::collections::HashMap;
use std::sync::Arc;

pub use self::edit::{Change, ChangeKind, Patch, Point, Range};
use gqls_db::{GqlsDatabase, SourceDatabase};
use ropey::Rope;
use vfs::FileId;

#[derive(Default)]
pub struct Ide {
    db: GqlsDatabase,
    file_ropes: HashMap<FileId, Rope>,
}

impl Ide {
    pub fn apply(&mut self, change: &Change) {
        self.db.request_cancellation();
        let rope = match &change.kind {
            ChangeKind::Patch(patch) => {
                let mut rope =
                    self.file_ropes.get(&change.file).cloned().expect("patch on initial edit");
                patch.apply(&mut rope);
                rope
            }
            ChangeKind::Set(text) => Rope::from_str(text),
        };
        // FIXME don't need to create this string from the rope in the `Set` case
        let text = rope.to_string();
        let old = self.file_ropes.insert(change.file, rope).map(|_| self.db.file_tree(change.file));
        let tree = gqls_parse::parse(&text, old.as_deref());
        self.db.set_file_tree(change.file, Arc::new(tree));
    }
}
