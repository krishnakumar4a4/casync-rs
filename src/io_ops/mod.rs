use std::fs::File;
use std::io::Result;
use std::fs::{self, DirBuilder};
use std::fs::OpenOptions;

pub fn create_chunk_store_dir(chunk_store_dir: &str) -> Result<()> {
    DirBuilder::new()
        .recursive(true)
        .create(chunk_store_dir)?;
    Ok(())
}

pub fn create_chunk_index_file(chunk_index_file: &str) -> File {
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(chunk_index_file).unwrap()
}

pub fn get_file_to_read(filename: &str) -> File {
    File::open(filename).unwrap()
}

// pub fn get_file_to_extract(filename: &str) -> File {
//     File::open(file_name).unwrap()
// }

pub fn get_file_to_write(filename: &str) -> File {
    File::create(filename).unwrap()
}
