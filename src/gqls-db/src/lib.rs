#![deny(rust_2018_idioms)]

pub use gqls_base_db::{FileData, SourceDatabase};
pub use gqls_ir::DefDatabase;
pub use salsa::{self, Database, ParallelDatabase, Snapshot};

use std::mem::ManuallyDrop;

#[salsa::database(gqls_base_db::SourceDatabaseStorage, gqls_ir::DefDatabaseStorage)]
pub struct GqlsDatabase {
    storage: ManuallyDrop<salsa::Storage<Self>>,
}

impl Default for GqlsDatabase {
    fn default() -> Self {
        let mut this = Self { storage: Default::default() };
        this.set_files(Default::default());
        this
    }
}

impl GqlsDatabase {
    pub fn request_cancellation(&mut self) {
        self.salsa_runtime_mut().synthetic_write(salsa::Durability::LOW);
    }
}

impl Database for GqlsDatabase {
}

impl ParallelDatabase for GqlsDatabase {
    fn snapshot(&self) -> salsa::Snapshot<Self> {
        Snapshot::new(Self { storage: ManuallyDrop::new(self.storage.snapshot()) })
    }
}

impl Drop for GqlsDatabase {
    fn drop(&mut self) {
        unsafe {
            ManuallyDrop::drop(&mut self.storage);
        }
    }
}
