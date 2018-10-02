use std::fs::File;
use std::io::Read;
use std::io::Write;
use io;

pub fn extract() {
    //Code to reassemble from chunks
    println!("Extracting from chunks");
    process_extraction();
}

fn output_file_for_extraction(filename: &str) -> File {
    io::get_file_to_write(filename)
}

fn process_extraction() {
    let mut output_file_for_extraction = output_file_for_extraction("out");
    let mut index_file_to_read = index_file_to_read("index.caidx");
    let mut read_buf = [0; 70];
    loop {
        //TODO: Use seek for optimization, instead of read_exact
        // and match_arrays
        match index_file_to_read.read_exact(&mut read_buf) {
            Ok(()) => (),
            Err(err) => {
                break;
            }
        };
        let chunk_file_name = String::from_utf8(read_buf[..64].to_vec()).unwrap();
        let uncompressed_chunk_size = &read_buf[65..70];

        let mut path_to_chunk = "default.cstr/".to_string();
        path_to_chunk.push_str(&chunk_file_name);
        path_to_chunk.push_str(".cacnk");  
        let mut chunk_file_to_read = io::get_file_to_read(&path_to_chunk);
        let mut buffer = Vec::new();
        chunk_file_to_read.read_to_end(&mut buffer);
        output_file_for_extraction.write_all(&buffer[..]);
    };
}

fn index_file_to_read(filename: &str) -> File {
    io::get_file_to_read(filename)
}
