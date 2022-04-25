use gqls_db::SourceDatabase;
use vfs::FileId;

use crate::Snapshot;

pub struct SemanticToken {}

impl Snapshot {
    pub fn semantic_tokens(&self, file: FileId) -> Vec<SemanticToken> {
        let tree = self.file_tree(file);
        // gqls_syntax::traverse(&tree);
        todo!()
    }
}
