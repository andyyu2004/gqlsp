use vfs::FileId;

use crate::Snapshot;

pub struct SemanticToken {}

impl Snapshot {
    pub fn semantic_tokens(&self, file: FileId) -> Vec<SemanticToken> {
        todo!()
    }
}
