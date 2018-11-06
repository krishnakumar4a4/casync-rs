extern crate hash_roll;
extern crate clap;
extern crate crypto;
extern crate zstd;
extern crate reqwest;
extern crate url;

use hash_roll::buzhash::BuzHash;
use hash_roll::buzhash::BuzHashBuf;
use clap::{Arg, App, SubCommand};

mod chunker;
mod io_ops;
mod assembler;
mod extracter;

use chunker::ChunkerConfig;

fn main() {
    let matches = App::new("casync-rs")
        .version("1.0")
        .author("krishna kumar <krishna.thokala2010@gmail.com>")
        .subcommand(SubCommand::with_name("make")
                    .help("Chunk the file")
                    .arg(Arg::with_name("index")
                         .short("i")
                         .long("index")
                         .help("index file path, defualt ./index.caidx")
                         .takes_value(true))
                    .arg(Arg::with_name("store")
                         .short("s")
                         .long("store")
                         .help("Chunk store path, default ./default.castr")
                         .takes_value(true))
                    .arg(Arg::with_name("file")
                         .short("f")
                         .long("file")
                         .help("input file for make and outout file for extract")
                         .takes_value(true)
                         .required(true)))
        .subcommand(SubCommand::with_name("extract")
                    .help("Create file from chunks")
                    .arg(Arg::with_name("index")
                         .short("i")
                         .long("index")
                         .help("index file path, defualt ./index.caidx")
                         .takes_value(true))
                    .arg(Arg::with_name("store")
                         .short("s")
                         .long("store")
                         .help("Chunk store path, default ./default.castr")
                         .takes_value(true))
                    .arg(Arg::with_name("file")
                         .short("f")
                         .long("file")
                         .help("input file for make and outout file for extract")
                         .takes_value(true)
                         .required(true)))
                    .get_matches();

    match matches.subcommand() {
        ("make", Some(sub_com)) => {
            let index_file = sub_com.value_of("index");
            let store_dir = sub_com.value_of("store");
            let io_file = sub_com.value_of("file");
            let mut chunker_config = ChunkerConfig::new(index_file, store_dir, io_file);

            let file_to_read = io_ops::get_file_to_read(chunker_config.get_input_file_name());
            let mut b = BuzHashBuf::from(BuzHash::with_capacity(15));
            let h = {
                let mut m = b.clone();
                // This can be configured
                m.push(&[9,45,128,100,122,9,45,128,100,122,9,45,128,100,122]);
                m.hash()
            };

            let chunk_store_dir_name = chunker_config.get_chunk_store_dir_name();
            let chunk_index_file_name = chunker_config.get_chunk_index_file_name();

            match io_ops::create_chunk_store_dir(chunk_store_dir_name){
                Ok(_) => {
                    let mut chunk_index_file = io_ops::create_chunk_index_file(chunk_index_file_name);
                    chunker::process_chunks(&mut b,h,file_to_read,&mut chunker_config, &mut chunk_index_file)
                },
                Err(e) => {
                    println!("Unable to create chunk store at {}, reason {}", "default.cstr", e);
                }
            }
        }
        ("extract", Some(sub_com)) => {
            let index_file = sub_com.value_of("index");
            let store_dir = sub_com.value_of("store");
            let io_file = sub_com.value_of("file");
            let mut chunker_config = ChunkerConfig::new(index_file, store_dir, io_file);
            assembler::assemble(&chunker_config);
        }
        _ => {
            panic!("No matching command, try make or extract")
        }
    }
}
