use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use serde_json::Value;

use crate::error::Result;

pub enum WalEntry{
    Insert { id : Uuid, vector : Vec<f32>, metadata: HashMap<String, Value> },
    Update { id: Uuid, vector : Vec<f32>, metadata: HashMap<String, Value> },
    Delete { id: Uuid },
    Checkpoint { timestamp: u64}
}

pub struct Wal {
    file: BufWriter<File>,
    path: PathBuf,
}

impl Wal {
    pub fn new(path: PathBuf) -> Result<Self> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)?;
        Ok(Wal {
            file: BufWriter::new(file),
            path,
        })
    }
}
