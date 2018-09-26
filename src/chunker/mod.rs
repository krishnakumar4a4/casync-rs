use std::collections::VecDeque;
use std::fs::File;
use std::string::String;

use io;

use hash_roll::buzhash::BuzHashBuf;
use std::io::Write;
use std::io::Read;


pub struct ChunkerConfig {
    pub chunk_count: u64,
    pub chunk_store: String,
}

impl ChunkerConfig{
    pub fn get_new_chunk_file_name(&mut self) -> String {
        self.chunk_count += 1;
        let mut file_no = (self.chunk_count).to_string();
        file_no.push_str(".cnk");
        file_no
    }

    pub fn get_store_dir(self) -> String {
        self.chunk_store.clone()
    }

    pub fn new() -> ChunkerConfig {
        let chunk_store_dir = "default.cstr".to_string();
        ChunkerConfig{chunk_count: 0, chunk_store: chunk_store_dir.clone()}
    }
}


pub fn create_chunk_file(chunker: &mut ChunkerConfig, data: &VecDeque<u8>) {
    let file_path_write = format!("{}/{}","default.cstr",&chunker.get_new_chunk_file_name());
    let mut file_to_write = io::get_file_to_write(&file_path_write);
    file_to_write.write_all(data.as_slices().0);
}

pub fn process_chunks(b: &mut BuzHashBuf, other_hash: u8, file: File, chunker: &mut ChunkerConfig) -> usize {
    let mut chunk_buf = VecDeque::new();
    for (_i, v) in file.bytes().enumerate() {
        let each_byte = v.unwrap();
        chunk_buf.push_back(each_byte.clone());
        b.push_byte(each_byte);
        if b.hash() == other_hash {
            //println!("length of chunk, {}",chunk_buf.len());
            create_chunk_file(chunker, &chunk_buf);
            chunk_buf = VecDeque::new();
            //return i+1;
        }
    }
    create_chunk_file(chunker, &chunk_buf);
    0
}
