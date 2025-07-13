//! Handles saving and loading the Sum Tree to/from disk.

use super::sum_tree::SumTree;
use sled::Db;
use std::path::Path;

/// Manages the persistence of a Sum Tree using an embedded database.
pub struct PersistentSumTree {
    tree: SumTree,
    db: Db,
}

impl PersistentSumTree {
    /// Opens or creates a persistent tree at the given path.
    pub fn open<P: AsRef<Path>>(path: P) -> sled::Result<Self> {
        let db = sled::open(path)?;
        // TODO: Implement loading the tree from the database on startup.
        // This will involve deserializing nodes with `bincode`.
        let tree = SumTree::new(Box::new(super::indexing::LineCountMetric));
        Ok(Self { tree, db })
    }

    /// Saves the current state of the tree to the database.
    pub fn flush(&self) -> sled::Result<()> {
        // TODO: Implement serialization of dirty nodes to the database.
        // This should be done transactionally.
        self.db.flush()?;
        Ok(())
    }
} 