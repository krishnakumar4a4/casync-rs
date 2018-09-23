extern crate hash_roll;
extern crate clap;

use std::fs::File;
use std::io::Error;
use std::io::Read;
use std::io::Write;

use hash_roll::buzhash::BuzHash;
use hash_roll::buzhash::BuzHashBuf;
use clap::{Arg, App, SubCommand};

use std::collections::VecDeque;

struct Chunker {
    chunk_count: u64,
}

impl Chunker{
    fn get_new_chunk_file_name(&mut self) -> std::string::String {
        self.chunk_count += 1;
        let mut file_no = (self.chunk_count).to_string();
        file_no.push_str(".cnk");
        file_no
    }
}

fn main() {
    println!("Hello, world!");

    let matches = App::new("casync-rs")
        .version("1.0")
        .author("krishna kumar <krishna.thokala2010@gmail.com>")
        .arg(Arg::with_name("make")
             .short("m")
             .long("make")
             .help("Chunk the file").takes_value(false))
        .arg(Arg::with_name("extract")
             .short("e")
             .long("extract")
             .help("Create file from chunks").takes_value(false))
        .get_matches();


    if matches.is_present("make") {
        let file_to_read = get_file_to_read();
        //let mut file_to_write = get_file_to_write();

        //let mut bytes = [0;1];
        //for byte in file_to_read.bytes(){
        //    bytes[0] = byte.unwrap();
        //    file_to_write.write(&bytes);
        //}

        let mut b = BuzHashBuf::from(BuzHash::with_capacity(7));
        let h = {
            let mut m = b.clone();
            m.push(&[0,0,0,0,0,0,0]);
            m.hash()
        };

        let mut chunker = Chunker{chunk_count: 0};

        println!("Match found at {:?}",process_chunks(&mut b,h,file_to_read,&mut chunker));
    }
    else if matches.is_present("extract") {
        //Code to reassemble from chunks
        println!("Extracting from chunks");
        let mut file_to_write = get_file_to_write("out.txt");
        for i in 1..20162{
            let mut file_no = (i).to_string();
            file_no.push_str(".cnk");
            let mut file_to_read = get_file_to_extract(&file_no);
            let mut buffer = Vec::new();
            file_to_read.read_to_end(&mut buffer);
            file_to_write.write_all(&buffer[..]);
        }
    }
}

fn get_file_to_read() -> File {
    File::open("input.txt").unwrap()
}

fn get_file_to_extract(file_name: &str) -> File {
    File::open(file_name).unwrap()
}

fn get_file_to_write(file_name: &str) -> File {
    File::create(file_name).unwrap()
}

fn create_chunk_file(chunker: &mut Chunker, data: &VecDeque<u8>) {
    let mut file_to_write = get_file_to_write(&chunker.get_new_chunk_file_name());
    file_to_write.write_all(data.as_slices().0);
}

fn process_chunks(b: &mut BuzHashBuf, other_hash: u8, file: File, chunker: &mut Chunker) -> usize {
    let mut chunk_buf = VecDeque::new();
    for (i, v) in file.bytes().enumerate() {
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

