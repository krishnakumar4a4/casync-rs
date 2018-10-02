use std::collections::VecDeque;
use std::fs::File;
use std::string::String;

use io_ops;
use std::io;

use hash_roll::buzhash::BuzHashBuf;
use std::io::Write;
use std::io::Read;

use crypto::digest::Digest;
use crypto::sha2::Sha256;

use std::mem::transmute;
use std::str;

use zstd::Encoder;

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
    
    pub fn new() -> ChunkerConfig {
        let chunk_store_dir = "default.cstr".to_string();
        let chunk_index_file = "index.caidx".to_string();
        ChunkerConfig{chunk_count: 0,
                      chunk_store: chunk_store_dir.clone(),
                      chunk_index_file: chunk_index_file.clone()}
    }
}

pub fn shall_break(chunk_size: usize) -> bool {
    chunk_size > 512000 && chunk_size < 1024000
}

pub fn process_chunks(b: &mut BuzHashBuf, other_hash: u8, file: File, chunker: &mut ChunkerConfig, chunk_index_file: &mut File) -> usize {

    let mut chunk_buf = VecDeque::new();

    for (_i, v) in file.bytes().enumerate() {
        let each_byte = v.unwrap();
        chunk_buf.push_back(each_byte.clone());
        b.push_byte(each_byte);
        if (b.hash() == other_hash) && shall_break(chunk_buf.len()) {
            create_chunk_update_index(chunker, chunk_index_file, &mut chunk_buf);
            chunk_buf = VecDeque::new();
        }
    }
    create_chunk_update_index(chunker, chunk_index_file, &mut chunk_buf);
    0
}

pub fn create_chunk_update_index(chunker: &mut ChunkerConfig, chunk_index_file: &mut File, chunk_buf: &mut VecDeque<u8>) {
    // Write hash of 32bytes in index file
    let mut hasher = Sha256::new();
    hasher.input(&chunk_buf.as_slices().0);
    let chunk_hash = hasher.result_str();
    let chunk_hash_bytes = chunk_hash.as_bytes();
    let chunk_size = chunk_buf.len();
    let chunk_size_bytes = get_chunk_size_bytes(chunk_index_file, chunk_size);
    if ! chunk_exists(chunk_index_file, chunk_hash_bytes, chunk_size_bytes) {
        // Create the chunk
        compress_and_write_chunk(&chunk_hash, chunker, chunk_buf);
    }
    chunk_index_file.write_all(&chunk_hash_bytes);

    // Write size of chunk to standard 6bytes in index file
    // usize will be 32bits for 32bit target and 64bits for 64bit target
    write_chunk_size(chunk_index_file, chunk_size_bytes);
}

#[cfg(target_arch = "x86")]
pub fn get_chunk_size_bytes(chunk_index_file: &mut File, chunk_size: usize) -> [u8; 6] {
    let size_to_write:[u8; 6] = unsafe { transmute(chunk_size.to_be()) };
    size_to_write
}

#[cfg(target_arch = "x86_64")]
pub fn get_chunk_size_bytes(chunk_index_file: &mut File, chunk_size: usize) -> [u8; 6] {
    let size_to_write:[u8; 8] = unsafe { transmute(chunk_size.to_be()) };
    let mut size_array:[u8; 6] = [0;6];
    size_array.copy_from_slice(&size_to_write[2..8]);
    size_array
}

pub fn write_chunk_size(chunk_index_file: &mut File, chunk_size: [u8; 6]) {
    chunk_index_file.write_all(&chunk_size);
}

pub fn chunk_exists(chunk_index_file: &mut File, chunk_hash_bytes: &[u8], chunk_size_bytes: [u8; 6]) -> bool {
    let mut read_buf = [0; 70];
    let mut index_to_match: [u8;70] = [0; 70];

    for i in 0..70 {
        if i > 63 {
            index_to_match[i] = chunk_size_bytes[i-64];
            continue;
        }
       index_to_match[i] = chunk_hash_bytes[i];
    };

    let mut chunk_exists = false;
    loop {
        //TODO: Use seek for optimization, instead of read_exact
        // and match_arrays
        match chunk_index_file.read_exact(&mut read_buf) {
            Ok(()) => (),
            Err(err) => {
                break;
            }
        };
        if match_arrays(read_buf,index_to_match) {
            //TODO: Can compute number of repeated chunks here
            chunk_exists = true;
            break;
        }
    };
    chunk_exists
}

pub fn match_arrays(array1: [u8;70], array2: [u8;70]) -> bool {
    let mut matched = true;
    for i in 1..70 {
        if ! (array1[i] == array2[i]) {
            matched = false;
            break;
        }
    }
    matched
}

pub fn compress_and_write_chunk(chunk_hash: &str,chunker: &mut ChunkerConfig, chunk_buf: &mut VecDeque<u8>) {
    create_chunk_file(&chunk_hash, chunker, chunk_buf); 
}


pub fn create_chunk_file(chunk_hash: &str,chunker: &mut ChunkerConfig, data: &mut VecDeque<u8>) {
    let file_path_write = format!("{}/{}.cacnk","default.cstr",chunk_hash);
    let mut file_to_write = io_ops::get_file_to_write(&file_path_write);

    let mut encoder = Encoder::new(file_to_write,21).unwrap();
    io::copy(&mut data.as_slices().0, &mut encoder);
    encoder.finish();
}
