extern crate hash_roll;
extern crate clap;
extern crate crypto;
extern crate zstd;

use std::io::Read;
use std::io::Write;

use hash_roll::buzhash::BuzHash;
use hash_roll::buzhash::BuzHashBuf;
use clap::{Arg, App};

mod chunker;
mod io_ops;
mod assembler;

use chunker::ChunkerConfig;

fn main() {
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
        let file_to_read = io_ops::get_file_to_read("input_block");
        let mut b = BuzHashBuf::from(BuzHash::with_capacity(15));
        let h = {
            let mut m = b.clone();
            //This can be configured
            m.push(&[9,45,128,100,122,9,45,128,100,122,9,45,128,100,122]);
            m.hash()
        };

        let mut chunker_obj = ChunkerConfig::new();
        match io_ops::create_chunk_store_dir("default.cstr"){
            Ok(_) => {
                let mut chunk_index_file = io_ops::create_chunk_index_file("index.caidx");
                println!("Match found at {:?}",chunker::process_chunks(&mut b,h,file_to_read,&mut chunker_obj, &mut chunk_index_file))
            },
            Err(e) => {
                println!("Unable to create chunk store at {}, reason {}", "default.cstr", e);
            }
        }
    }
    else if matches.is_present("extract") {
        assembler::assemble();
    }
}
