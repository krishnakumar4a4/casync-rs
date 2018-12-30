use std::fs::File;
use std::io::Result;
use std::fs::{DirBuilder};
use std::fs::OpenOptions;

pub fn create_chunk_store_dir(chunk_store_dir: &str) -> Result<()> {
    create_dir(chunk_store_dir)
}

pub fn create_dir(path: &str) -> Result<()> {
    DirBuilder::new()
        .recursive(true)
        .create(path)
}

pub fn create_chunk_index_file(chunk_index_file: &str) -> Result<File> {
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(chunk_index_file)
}

pub fn get_file_to_read(filename: &str) -> Result<File> {
    File::open(filename)
}

pub fn get_file_to_write(filename: &str) -> Result<File> {
    File::create(filename)
}
