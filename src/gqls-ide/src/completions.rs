use gqls_db::DefDatabase;
use tree_sitter::Point;
use vfs::FileId;

use crate::Snapshot;

#[derive(Debug)]
pub struct Completion {
    pub label: String,
}

impl Snapshot {
    pub fn completions(&self, file: FileId, _at: Point) -> Vec<Completion> {
        // FIXME just random implementation for now
        let project_items = self.project_items(file);
        project_items
            .values()
            .flat_map(|items| items.items.iter())
            .map(|(_, item)| Completion { label: item.name.to_string() })
            .collect()
    }
}

#[cfg(test)]
mod tests;
