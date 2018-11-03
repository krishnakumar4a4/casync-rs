use io_ops;
use std::io::Write;
use reqwest::Client;

pub struct HttpClient {
    pub client: Client,
}

pub trait ClientAdapter {
    fn new() -> Self;
    fn downloadFile(&self, &mut String, String, String);
}

impl ClientAdapter for HttpClient {
    fn new() -> Self {
        HttpClient {
            client: Client::new()
        }
    }
    fn downloadFile(&self, server_url: &mut String, resource: String, dir: String) {
        let mut dir_path = dir.clone();
        io_ops::create_dir(&dir_path);
        let mut resource_path = "./".to_string();
        resource_path.push_str(&resource);
        println!("file path to write {}",resource_path);
        let mut file = io_ops::get_file_to_write(&resource_path);
        server_url.push_str("/");
        server_url.push_str(&resource_path);
        let mut buf: Vec<u8> = vec![];
        println!("url is {}",server_url);
        match self.client.get(&*server_url).send() {
            Ok(mut resp) => {
                resp.copy_to(&mut buf).unwrap();
                file.write_all(&buf).unwrap();
            }
            Err(e) => {
                println!("Could not download {}, status {:?}", server_url, e.status());
            }
        }
    }
}

