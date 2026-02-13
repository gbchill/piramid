use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::PathBuf;

use dashmap::mapref::entry;

use crate::error::Result;
use super::entry::WalEntry;

pub struct Wal {
    file: Option<BufWriter<File>>,
    path: PathBuf,
    next_seq: u64,
}

impl Wal {
    pub fn new(path: PathBuf) -> Result<Self> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)?;
        Ok(Wal {
            file: Some(BufWriter::new(file)),
            path,
            next_seq: 0,
        })
    }


    
    pub fn disabled(path: PathBuf) -> Result<Self> {
        Ok(Wal {
            file: None,
            path,
            next_seq: 0,
        })
    }

    pub fn log(&mut self, entry: &mut WalEntry) -> Result<()> {
        entry.seq = self.next_seq;
        if let Some(file) = &mut self.file {
            let json = serde_json::to_string(entry)?;
            writeln!(file, "{}", json)?;
            file.flush()?;
        }
        self.next_seq += 1;
        Ok(())
    }

    pub fn replay(&self, min_seq: u64) -> Result<Vec<WalEntry>> {
        if self.file.is_none() {
            return Ok(Vec::new());
        }


        
        let file = File::open(&self.path)?;
        let reader = BufReader::new(file);
        let mut entries = Vec::new();
        
        for line in reader.lines() {
            let line = line?;
            if line.is_empty(){
                continue;
            }
            let entry: WalEntry = serde_json::from_str(&line)?;
            if entry.seq <= min_seq {
                entries.push(entry);
            }
        }
        
        Ok(entries)
    }

    pub fn checkpoint(&mut self, timestamp: u64) -> Result<()> {
        let mut entry = WalEntry::Checkpoint { timestamp, seq: 0 };
        self.log(&mut entry)?;
        Ok(())
    }
    
    pub fn rotate(&mut self) -> Result<()> {
        // close current writer 
        drop(self.file.take());
        // open fresh empty file 
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&self.path)?;
        file.sync_all()?; // ensure truncation hits disk 
        self.file = Some(BufWriter::new(file));
        Ok(())
    }
    
    pub fn flush(&mut self) -> Result<()> {
        if let Some(file) = &mut self.file {
            file.flush()?;
        }
        Ok(())
    }
}
