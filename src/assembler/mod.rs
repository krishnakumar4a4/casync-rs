use std::io;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use io_ops;
use zstd::Decoder;
use chunker::ChunkerConfig;
use extracter;
use extracter::HttpClient;
use extracter::ClientAdapter;

pub fn assemble(chunker: &ChunkerConfig) {
    //Code to reassemble from chunks
    println!("Extracting from chunks");
    process_assembling(chunker);
}

// fn process_assembling(chunker: &ChunkerConfig) {
//     let mut output_file_for_extraction = output_file_for_extraction(chunker.get_assembled_file_name());
//     let mut index_file_to_read = index_file_to_read(chunker.get_chunk_index_file_name());
//     let mut read_buf = [0; 70];
//     loop {
//         //TODO: Use seek for optimization, instead of read_exact
//         // and match_arrays
//         match index_file_to_read.read_exact(&mut read_buf) {
//             Ok(()) => (),
//             Err(_err) => {
//                 break;
//             }
//         };
//         let chunk_file_name = String::from_utf8(read_buf[..64].to_vec()).unwrap();
//         let _uncompressed_chunk_size = &read_buf[65..70];

//         let mut path_to_chunk = chunker.get_chunk_store_dir_name().to_string();
//         path_to_chunk.push_str("/");
//         path_to_chunk.push_str(&chunk_file_name);
//         path_to_chunk.push_str(".");
//         path_to_chunk.push_str(chunker.get_chunk_file_extension());
//         let chunk_file_to_read = io_ops::get_file_to_read(&path_to_chunk);
//         let mut buffer = Vec::new();
//         let mut decoder = Decoder::new(chunk_file_to_read).unwrap();
//         io::copy(&mut decoder, &mut buffer);
//         output_file_for_extraction.write_all(&buffer[..]);
//     };
// }

fn process_assembling(chunker: &ChunkerConfig) {
    let client: HttpClient = extracter::ClientAdapter::new();
    client.downloadFile(&mut String::from("http://0.0.0.0:8000/"), String::from(chunker.get_chunk_index_file_name()), String::from("./"));
    let mut output_file_for_extraction = output_file_for_extraction(chunker.get_assembled_file_name());
    let mut index_file_to_read = index_file_to_read(chunker.get_chunk_index_file_name());
    let mut read_buf = [0; 70];
    loop {
        //TODO: Use seek for optimization, instead of read_exact
        // and match_arrays
        match index_file_to_read.read_exact(&mut read_buf) {
            Ok(()) => (),
            Err(_err) => {
                break;
            }
        };
        let chunk_file_name = String::from_utf8(read_buf[..64].to_vec()).unwrap();
        let _uncompressed_chunk_size = &read_buf[65..70];

        let mut path_to_chunk = chunker.get_chunk_store_dir_name().to_string();
        path_to_chunk.push_str("/");
        path_to_chunk.push_str(&chunk_file_name);
        path_to_chunk.push_str(".");
        path_to_chunk.push_str(chunker.get_chunk_file_extension());
        client.downloadFile(&mut String::from("http://0.0.0.0:8000/"), path_to_chunk.clone(), String::from("./defaulti.castr"));
        let chunk_file_to_read = io_ops::get_file_to_read(&path_to_chunk);
        let mut buffer = Vec::new();
        let mut decoder = Decoder::new(chunk_file_to_read).unwrap();
        io::copy(&mut decoder, &mut buffer);
        output_file_for_extraction.write_all(&buffer[..]);
    };
}

fn output_file_for_extraction(filename: &str) -> File {
    io_ops::get_file_to_write(filename)
}

fn index_file_to_read(filename: &str) -> File {
    io_ops::get_file_to_read(filename)
}
