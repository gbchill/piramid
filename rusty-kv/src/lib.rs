use std::collections:Hashmap;
use std::fs::{File, OpenOptions}; // we use OpenOptions specificlly because we want to "edit" files
use std::io;
use std::path::Path;
use std::io::{self, Write, Seek, SeekFrom}; // for writing and seeking traits

pub mod entry;
use entry::Entry;

pub struct RustyKV{
    // the physical file on the disk
    file::File,

    // in memory map: key -> byte offset
    index: HashMap<String, u64>,
}


impl RustyKV{
    // we open hte file and prepare the database
    pub fn open(path : &str) -> io::Result<RustyKV> {
        let file = OpenOptions::new()
            .read(true) // setting config
            .write(true)
            .create(true)
            .append(true) // always write at the end
            .open(path)?;
        let store = RustyKV{
            file,
            index: HashMap::new();
        };

        // we will add core here t read the file and rebuild the index if database
        // exists already
        Ok(store)
    }

    // in log sturctured database, we will do this in four parts
    // 1-> wrap the input key value pair in the binary entry format like we are requesting
    // 2-> ask the operating system, where is the end of the file right now? basically the offset
    // 3-> append the bytes to the disk
    // 4-> update the hashmap in RAM

    pub fn set(&mut self, key: String, value: String) -> io::Result<()>{
        // preparing the data
        // using 0 for timestamps as demo

        let entry = Entry::new(
            key.clone().into_bytes(),
            // into_bytes converts string to vector of bytes
            // as_bytes converts string to slice of bytes
            // but we use into bytes because we need ownership of the data
            value.into_bytes(),
            0
            );

        let encoded = entry.encode();

        let size = encoded.len() as u64;

        // asking the OS where is the end of the file

        let current_offset= self.file.seek(SeekFrom::End(0))?;
        // when we open a file in append mode, the os usually handles writing at the end of the
        // file but we still need to know the offset where we are writing since we need to update
        // the in memory index


        // appending the data to the disk
        // we use write_all because write may not write all the bytes at once
        self.file.write_all(&encoded)?;

        // updating the in memory index
        // we map the key to the offset where the data is stored
        self.index.insert(key, current_offset);

        Ok(())

    }
}

















