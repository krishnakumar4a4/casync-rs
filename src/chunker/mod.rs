use std::collections::VecDeque;
use std::fs::File;
use std::string::String;

use io;

use hash_roll::buzhash::BuzHashBuf;
use std::io::Write;
use std::io::Read;

use sha3::{Digest, Sha3_256};
//use self::crypto::digest::Digest;
//use self::crypto::sha2::Sha256;

use std::mem::transmute;
use std::str;

pub struct ChunkerConfig {
    pub chunk_count: u64,
    pub chunk_store: String,
    pub chunk_index_file: String,
}

impl ChunkerConfig{
    pub fn get_new_chunk_file_name(&mut self) -> String {
        self.chunk_count += 1;
        let mut file_no = (self.chunk_count).to_string();
        file_no.push_str(".cnk");
        file_no
    }

    // pub fn get_store_dir(self) -> String {
    //     self.chunk_store.clone()
    // }

    pub fn new() -> ChunkerConfig {
        let chunk_store_dir = "default.cstr".to_string();
        let chunk_index_file = "index.caidx".to_string();
        ChunkerConfig{chunk_count: 0,
                      chunk_store: chunk_store_dir.clone(),
                      chunk_index_file: chunk_index_file.clone()}
    }
}

pub fn process_chunks(b: &mut BuzHashBuf, other_hash: u8, file: File, chunker: &mut ChunkerConfig, chunk_index_file: &mut File) -> usize {

    let mut chunk_buf = VecDeque::new();

    for (_i, v) in file.bytes().enumerate() {
        let each_byte = v.unwrap();
        chunk_buf.push_back(each_byte.clone());
        b.push_byte(each_byte);
        if b.hash() == other_hash {
            create_chunk_update_index(chunker, chunk_index_file, &chunk_buf);
            chunk_buf = VecDeque::new();
        }
    }
    create_chunk_update_index(chunker, chunk_index_file, &chunk_buf);
    0
}

pub fn create_chunk_file(chunk_hash: &[u8],chunker: &mut ChunkerConfig, data: &VecDeque<u8>) {
    //let file_path_write = format!("{}/{}","default.cstr",String::from_utf8_lossy(&chunk_hash));
    let file_path_write = format!("{}/{:x}","default.cstr",chunk_hash);
    println!("File to be created {:?}", file_path_write);
    let mut file_to_write = io::get_file_to_write(&file_path_write);
    file_to_write.write_all(data.as_slices().0);
}

pub fn create_chunk_update_index(chunker: &mut ChunkerConfig, chunk_index_file: &mut File, chunk_buf: &VecDeque<u8>) {
    // Write hash of 32bytes in index file
    let chunk_hash = Sha3_256::digest(&chunk_buf.as_slices().0);
    chunk_index_file.write_all(&(chunk_hash.as_slice()));
 
    let chunk_size = chunk_buf.len();

    // Write size of chunk to standard 6bytes in index file
    // usize will be 32bits for 32bit target and 64bits for 64bit target
    write_chunk_size_64_bit(chunk_index_file, chunk_size);

    // Create the chunk
    create_chunk_file(&chunk_hash, chunker, &chunk_buf);
}

#[cfg(target_arch = "x86")]
pub fn write_chunk_size_32_bit(chunk_index_file: &mut File, chunk_size: usize) {
    let size_to_write:[u8; 6] = unsafe { transmute(chunk_size.to_be()) };
    chunk_index_file.write_all(&size_to_write);
}

#[cfg(target_arch = "x86_64")]
pub fn write_chunk_size_64_bit(chunk_index_file: &mut File, chunk_size: usize) {
    let size_to_write:[u8; 8] = unsafe { transmute(chunk_size.to_be()) };
    chunk_index_file.write_all(&size_to_write[2..8]);
}
