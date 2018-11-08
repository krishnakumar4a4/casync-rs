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
use url::Url;
use std::io::Result;

pub fn assemble(chunker: &ChunkerConfig) {
    //Code to reassemble from chunks
    println!("Extracting from chunks");
    process_assembling(chunker);
}

fn process_assembling(chunker: &ChunkerConfig) {
    let client: HttpClient = extracter::ClientAdapter::new();
    let mut index_file = String::from(chunker.get_chunk_index_file_name());

    if is_http_url(&index_file) {
        let index_file_from_url = http_index_file_download(&client, &index_file);
        index_file = index_file_from_url;
    }
    let mut index_file_to_read = match index_file_to_read(&index_file) {
        Ok(f) => f,
        Err(e) => panic!("Error: Cannot open index file {}, {:?}", index_file, e)
    };
    let mut read_buf = [0; 70];
    let file_name_to_assemble = chunker.get_assembled_file_name();
    let mut output_file_for_extraction = match output_file_for_extraction(file_name_to_assemble) {
        Ok(f) =>  f,
        Err(e) => panic!("Error: Cannot create file {} to write the output, {:?}", file_name_to_assemble,  e)
    };

    let chunk_store_dir = chunker.get_chunk_store_dir_name().to_string();
    let mut is_store_http_url = false;

    if is_http_url(&chunk_store_dir) {
        is_store_http_url = true;
    }

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
        let default_chunk_store_dir = String::from("./default.castr");
        let mut path_to_chunk = default_chunk_store_dir;
        path_to_chunk.push_str("/");
        path_to_chunk.push_str(&chunk_file_name);
        path_to_chunk.push_str(".");
        path_to_chunk.push_str(chunker.get_chunk_file_extension());
        if is_store_http_url {
            http_chunk_file_download(&client, &chunk_store_dir, path_to_chunk.clone());
        }
        let chunk_file_to_read = match io_ops::get_file_to_read(&path_to_chunk) {
            Ok(f) => f,
            Err(e) => panic!("Error: Cannot open chunk file {} to read, {:?}", path_to_chunk, e)
        };
        let mut buffer = Vec::new();
        let mut decoder = Decoder::new(chunk_file_to_read).unwrap();
        io::copy(&mut decoder, &mut buffer).expect("Error: Cannot decompress data to file");
        output_file_for_extraction.write_all(&buffer[..]).expect("Error: Cannot write decompressed data to file");
    };
}

fn output_file_for_extraction(filename: &str) -> Result<File> {
    io_ops::get_file_to_write(filename)
}

fn index_file_to_read(filename: &str) -> Result<File> {
    io_ops::get_file_to_read(filename)
}

fn is_http_url(url: &str) -> bool {
    match Url::parse(url) {
        Ok(parsed_url) => {
            (parsed_url.scheme() == "https" || parsed_url.scheme() == "http")
        }
        Err(e) => {
            println!("Warn: Not a http url, {:?}", e);
            false
        }
    }
}

fn http_index_file_download(client: &HttpClient, url: &str) -> String {
    match Url::parse(url) {
        Ok(_) => 0,
        Err(e) => panic!("Error: Cannot parse url {}, {:?}", url, e)
    };
    // Splitting on string to preserver query params if any
    let mut path_segments: Vec<&str> = url.split("/").collect();
    let segment_count = path_segments.len();
    let mut base_url = String::from("");
    if segment_count >= 1 {
        let index_file = path_segments.swap_remove(segment_count-1);
        for s in path_segments {
            base_url.push_str(s);
        }
        client.download_file(&mut String::from(base_url), String::from(index_file), String::from("./"));
        String::from(index_file)
    } else {
        panic!("Could not find index file name")
    }
}

fn http_chunk_file_download(client: &HttpClient, url: &str, chunk_file: String) {
    client.download_file(&mut String::from(url), chunk_file, String::from("./default.castr"));
}
