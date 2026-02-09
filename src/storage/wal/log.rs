use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::PathBuf;

use crate::error::Result;
use super::entry::WalEntry;

pub struct Wal {
    file: Option<BufWriter<File>>,
    path: PathBuf,
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
        })
    }
    
    pub fn disabled(path: PathBuf) -> Result<Self> {
        Ok(Wal {
            file: None,
            path,
        })
    }

    pub fn log(&mut self, entry: &WalEntry) -> Result<()> {
        if let Some(file) = &mut self.file {
            let json = serde_json::to_string(entry)?;
            writeln!(file, "{}", json)?;
            file.flush()?;
        }
        Ok(())
    }

    pub fn replay(&self) -> Result<Vec<WalEntry>> {
        if self.file.is_none() {
            return Ok(Vec::new());
        }
        
        let file = File::open(&self.path)?;
        let reader = BufReader::new(file);
        let mut entries = Vec::new();
        
        for line in reader.lines() {
            let line = line?;
            if !line.is_empty() {
                let entry: WalEntry = serde_json::from_str(&line)?;
                entries.push(entry);
            }
        }
        
        Ok(entries)
    }

    pub fn checkpoint(&mut self, timestamp: u64) -> Result<()> {
        self.log(&WalEntry::Checkpoint { timestamp })?;
        Ok(())
    }

    pub fn clear(&mut self) -> Result<()> {
        if let Some(file) = &mut self.file {
            file.get_mut().set_len(0)?;
            file.flush()?;
        }
        Ok(())
    }
    
    pub fn truncate(&mut self) -> Result<()> {
        self.clear()
    }
    
    pub fn flush(&mut self) -> Result<()> {
        if let Some(file) = &mut self.file {
            file.flush()?;
        }
        Ok(())
    }
}
