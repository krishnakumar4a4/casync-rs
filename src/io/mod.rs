use std::fs::File;
use std::fs::create_dir;
use std::io::Result;

pub fn create_chunk_store_dir(chunk_store_dir: &str) -> Result<()> {
    create_dir(chunk_store_dir)?;
    Ok(())
}

pub fn get_file_to_read() -> File {
    File::open("input_block").unwrap()
}

pub fn get_file_to_extract(file_name: &str) -> File {
    File::open(file_name).unwrap()
}

pub fn get_file_to_write(file_name: &str) -> File {
    File::create(file_name).unwrap()
}
