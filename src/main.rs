extern crate hash_roll;
extern crate clap;

use std::io::Read;
use std::io::Write;

use hash_roll::buzhash::BuzHash;
use hash_roll::buzhash::BuzHashBuf;
use clap::{Arg, App};

mod chunker;
mod io;

use chunker::ChunkerConfig;

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
        let file_to_read = io::get_file_to_read();
        //let mut file_to_write = get_file_to_write();

        //let mut bytes = [0;1];
        //for byte in file_to_read.bytes(){
        //    bytes[0] = byte.unwrap();
        //    file_to_write.write(&bytes);
        //}

        let mut b = BuzHashBuf::from(BuzHash::with_capacity(7));
        let h = {
            let mut m = b.clone();
            //This can be configured
            m.push(&[0,0,0,0,0,0,0]);
            m.hash()
        };

        let mut chunker_obj = ChunkerConfig::new();
        match io::create_chunk_store_dir("default.cstr"){
            Ok(_) => {
                println!("Match found at {:?}",chunker::process_chunks(&mut b,h,file_to_read,&mut chunker_obj))
            },
            Err(e) => {
                println!("Unable to create chunk store at {}, reason {}", "default.cstr", e);
            }
        }
    }
    else if matches.is_present("extract") {
        //Code to reassemble from chunks
        println!("Extracting from chunks");
        let mut file_to_write = io::get_file_to_write("out.txt");
        for i in 1..81959{
            let mut path_to_chunk = "default.cstr/".to_string();
            let file_no = (i).to_string();
            path_to_chunk.push_str(&file_no);
            path_to_chunk.push_str(".cnk");
            let mut file_to_read = io::get_file_to_extract(&path_to_chunk);
            let mut buffer = Vec::new();
            file_to_read.read_to_end(&mut buffer);
            file_to_write.write_all(&buffer[..]);
        }
    }
}
