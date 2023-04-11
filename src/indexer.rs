use std::mem::swap;
use std::{path::PathBuf};

use crate::btree::BTree;
use crate::error::{Error, ToInnerResult};
use crate::hash::{HASH_LENGTH, Hash};
use crate::offset::Offset;

/// Indexer is a struct representing the object storage's index, which maps the
/// object hash to the object's offset.
pub struct Indexer {
    b_tree: BTree,
}

impl Indexer {
    /// Create a new `Indexer` by path, it will:
    ///
    ///   - Create a new index file in the path.
    ///   - Return `Indexer` itself.
    ///
    /// If there is already a index data file, use method `open` rather than
    /// me.
    pub fn create(path: &PathBuf) -> Result<Self, Error> {
        let b_tree = BTree::new(&path.join("index"))
            .to_inner_result("open index file by B-Tree format")?;
        let mut result = Self { b_tree };
        Ok(result)
    }

    /// Open a `Index` by path from a existing index data file.
    pub fn open(path: &PathBuf) -> Result<Self, Error> {
        let b_tree = BTree::new(&path.join("index"))
            .to_inner_result("open index file by B-Tree format")?;
        let mut result = Self { b_tree };
        Ok(result)
    }

    /// Put a new record: a mapping from hash to the offset in data file.
    /// 
    /// See method `get` as well.
    pub fn put(&mut self, hash: &str, offset: u64) -> Result<(), Error> {
        let hash = Hash::from_str(hash).to_inner_result("turn to valid hash")?;
        let offset = Offset::new(offset);

        self.b_tree.put(&hash, offset)
    }

    /// Get the offset in the data file by the hash.
    pub fn get(&mut self, hash: &str) -> Result<Option<Offset>, Error> {
        let hash = Hash::from_str(hash).to_inner_result("turn to valid hash")?;

        self.b_tree.get(&hash)
    }
}
