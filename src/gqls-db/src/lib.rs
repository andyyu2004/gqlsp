pub use gqls_base_db::SourceDatabase;
pub use salsa::{self, Database, ParallelDatabase, Snapshot};

use std::mem::ManuallyDrop;

#[salsa::database(gqls_base_db::SourceDatabaseStorage)]
#[derive(Default)]
pub struct GqlsDatabase {
    storage: ManuallyDrop<salsa::Storage<Self>>,
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
