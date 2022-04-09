mod edit;

pub use self::edit::{Change, ChangeKind, Patch, Point, Range};
use gqls_db::{GqlsDatabase, SourceDatabase};
use ropey::Rope;

#[derive(Default)]
pub struct Ide {
    db: GqlsDatabase,
}

impl Ide {
    pub fn apply(&mut self, change: &Change) {
        self.db.request_cancellation();
        let rope = match &change.kind {
            ChangeKind::Patch(patch) => {
                let mut rope = self.db.file_rope(change.file);
                patch.apply(&mut rope);
                rope
            }
            ChangeKind::Set(text) => Rope::from_str(text),
        };
        self.db.set_file_rope(change.file, rope);
    }
}
