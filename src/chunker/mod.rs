use std::collections::VecDeque;
use std::fs::File;
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

pub struct ChunkerConfig<'a> {
    pub chunk_count: u64,
    pub default_chunk_store_dir: &'a str,
    pub default_chunk_index_file: &'a str,
    pub chunk_min_size: u32,
    pub chunk_max_size: u32,
    pub chunk_extension: &'a str,
    pub input: &'a str,
    pub output: &'a str,
}

impl<'a> ChunkerConfig<'a>{
    pub fn new(index_file: Option<&'a str>, store_dir: Option<&'a str>, io_file: Option<&'a str>) -> ChunkerConfig<'a> {
        let chunk_store_dir = match store_dir {
            Some(dir_path) => dir_path,
            None => "default.castr"
        };
        let chunk_index_file = match index_file {
            Some(file_path) => file_path,
            None => "index.caidx"
        };
        let io_file = match io_file {
            Some(file) => file,
            None => "block"
        };
        ChunkerConfig{
            chunk_count: 0,
            default_chunk_store_dir: chunk_store_dir,
            default_chunk_index_file: chunk_index_file,
            input: io_file,
            chunk_min_size: 512000,
            chunk_max_size: 1024000,
            chunk_extension: "cacnk",
            output: io_file
        }
    }
    pub fn get_chunk_store_dir_name(&self) -> &'a str {
        self.default_chunk_store_dir
    }
    pub fn get_chunk_index_file_name(&self) -> &'a str {
        self.default_chunk_index_file
    }
    pub fn get_chunk_file_extension(&self) -> &'a str {
        self.chunk_extension
    }
    pub fn get_input_file_name(&self) -> &'a str {
        self.input
    }
    pub fn get_assembled_file_name(&self) -> &'a str {
        self.output
    }
}

pub fn shall_break(chunk_size: usize) -> bool {
    chunk_size > 512000 && chunk_size < 1024000
}

pub fn process_chunks(b: &mut BuzHashBuf, other_hash: u8, file: File, chunker: &mut ChunkerConfig, chunk_index_file: &mut File) {

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
}

pub fn create_chunk_update_index(chunker: &mut ChunkerConfig, chunk_index_file: &mut File, chunk_buf: &mut VecDeque<u8>) {
    // Write hash of 32bytes in index file
    let mut hasher = Sha256::new();
    hasher.input(&chunk_buf.as_slices().0);
    let chunk_hash = hasher.result_str();
    let chunk_hash_bytes = chunk_hash.as_bytes();
    let chunk_size = chunk_buf.len();
    let chunk_size_bytes = get_chunk_size_bytes(chunk_size);
    if ! chunk_exists(chunk_index_file, chunk_hash_bytes, chunk_size_bytes) {
        // Create the chunk
        compress_and_write_chunk(&chunk_hash, chunker, chunk_buf);
    }
    chunk_index_file.write_all(&chunk_hash_bytes).expect("Error: Cannot write to chunk file");

    // Write size of chunk to standard 6bytes in index file
    // usize will be 32bits for 32bit target and 64bits for 64bit target
    write_chunk_size(chunk_index_file, chunk_size_bytes);
}

#[cfg(target_arch = "x86")]
pub fn get_chunk_size_bytes(chunk_size: usize) -> [u8; 6] {
    let size_to_write:[u8; 6] = unsafe { transmute(chunk_size.to_be()) };
    size_to_write
}

#[cfg(target_arch = "x86_64")]
pub fn get_chunk_size_bytes(chunk_size: usize) -> [u8; 6] {
    let size_to_write:[u8; 8] = unsafe { transmute(chunk_size.to_be()) };
    let mut size_array:[u8; 6] = [0;6];
    size_array.copy_from_slice(&size_to_write[2..8]);
    size_array
}

pub fn write_chunk_size(chunk_index_file: &mut File, chunk_size: [u8; 6]) {
    chunk_index_file.write_all(&chunk_size).expect("Error: Cannot write chunk size to index file");
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
            Err(_err) => {
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
    let file_path_write = format!("{}/{}.{}",chunker.get_chunk_store_dir_name(),chunk_hash, chunker.get_chunk_file_extension());
    let file_to_write = match io_ops::get_file_to_write(&file_path_write) {
        Ok(f) => f,
        Err(e) => panic!("Error: Cannot open file {} to write chunk, {:?}", file_path_write, e)
    };

    let mut encoder = Encoder::new(file_to_write,21).unwrap();
    io::copy(&mut data.as_slices().0, &mut encoder).expect("Error: Cannot write compressed data to file");
    encoder.finish().expect("Error: Cannot finish zstd encoding on chunk data");
}
