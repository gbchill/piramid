use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use uuid::Uuid;

use crate::error::Result;

//a single vector with its text
//store for each piece of text
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorEntry {
    pub id: Uuid,                // unique id for this entry
    pub vector: Vec<f32>,        // the actual embedding vector
    pub text: String,            // the original text
}

impl VectorEntry {
    //new entry with a random id
    pub fn new(vector: Vec<f32>, text: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            vector,
            text,
        }
    }
}

// handles saving and loading vectors to/from disk
// keeps everything in memory for fast access
pub struct VectorStorage {
    file: File,                           // the file we write to
    vectors: HashMap<Uuid, VectorEntry>,  // all vectors in memory
}

impl VectorStorage {
    // open a storage file creates it if it doesn't exist
    pub fn open(path: &str) -> Result<Self> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)?;

        let mut storage = Self {
            file,
            vectors: HashMap::new(),
        };

        // load any existing data
        storage.load()?;
        Ok(storage)
    }

    // add a new vector entry
    pub fn store(&mut self, entry: VectorEntry) -> Result<Uuid> {
        let id = entry.id;
        self.vectors.insert(id, entry);
        self.save()?;  // save to disk immediately
        Ok(id)
    }

    // look up a vector by its id
    pub fn get(&self, id: &Uuid) -> Option<VectorEntry> {
        self.vectors.get(id).cloned()
    }

    // get all vectors (used for searching)
    pub fn get_all(&self) -> Vec<&VectorEntry> {
        self.vectors.values().collect()
    }

    // how many vectors we have stored
    pub fn count(&self) -> usize {
        self.vectors.len()
    }

    // called when we first open the file
    fn load(&mut self) -> Result<()> {
        self.file.seek(SeekFrom::Start(0))?;
        let mut buf = Vec::new();
        self.file.read_to_end(&mut buf)?;

        // if the file is empty, that's fine
        if buf.is_empty() {
            return Ok(());
        }

        // deserialize the whole hashmap at once
        self.vectors = bincode::deserialize(&buf)?;
        Ok(())
    }

    // write all vectors back to disk
    // we serialize the entire hashmap to binary
    fn save(&mut self) -> Result<()> {
        let data = bincode::serialize(&self.vectors)?;
        self.file.seek(SeekFrom::Start(0))?;
        self.file.set_len(0)?;  // clear the file first
        self.file.write_all(&data)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_store_and_retrieve() {
        let _ = std::fs::remove_file("test.db");
        let mut storage = VectorStorage::open("test.db").unwrap();

        let vector = vec![0.1, 0.2, 0.3];
        let entry = VectorEntry::new(vector.clone(), "hello".to_string());
        let id = storage.store(entry).unwrap();

        let retrieved = storage.get(&id).unwrap();
        assert_eq!(retrieved.vector, vector);
        assert_eq!(retrieved.text, "hello");

        std::fs::remove_file("test.db").unwrap();
    }

    #[test] //tests 
    fn test_persistence() {
        let _ = std::fs::remove_file("test2.db");

        let id1;
        let id2;

        {
            let mut storage = VectorStorage::open("test2.db").unwrap();
            let e1 = VectorEntry::new(vec![1.0, 2.0], "first".to_string());
            let e2 = VectorEntry::new(vec![3.0, 4.0], "second".to_string());
            id1 = storage.store(e1).unwrap();
            id2 = storage.store(e2).unwrap();
        }

        {
            let storage = VectorStorage::open("test2.db").unwrap();
            assert_eq!(storage.count(), 2);
            assert_eq!(storage.get(&id1).unwrap().text, "first");
            assert_eq!(storage.get(&id2).unwrap().text, "second");
        }

        std::fs::remove_file("test2.db").unwrap();
    }
}
