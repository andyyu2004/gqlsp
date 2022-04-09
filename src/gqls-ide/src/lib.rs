mod edit;

use std::sync::Arc;

pub use self::edit::{Change, ChangeKind, Patch, Point, Range};
use gqls_db::{GqlsDatabase, SourceDatabase};

#[derive(Default)]
pub struct Ide {
    db: GqlsDatabase,
}

impl Ide {
    pub fn apply(&mut self, change: Change) {
        self.db.request_cancellation();
        match change.kind {
            ChangeKind::Patch(_) => todo!(),
            ChangeKind::Set(text) => self.db.set_file_text(todo!(), Arc::new(text)),
        }
    }
}
