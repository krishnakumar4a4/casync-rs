use io_ops;
use std::io::Write;
use reqwest::Client;

pub struct HttpClient {
    pub client: Client,
}

pub trait ClientAdapter {
    fn new() -> Self;
    fn download_file(&self, &mut String, String, String);
}

impl ClientAdapter for HttpClient {
    fn new() -> Self {
        HttpClient {
            client: Client::new()
        }
    }
    fn download_file(&self, server_url: &mut String, resource: String, dir: String) {
        let dir_path = dir.clone();
        io_ops::create_dir(&dir_path).expect("Error: cannot create directory to download  the chunks");
        let mut resource_path = "./".to_string();
        resource_path.push_str(&resource);
        let mut file = match io_ops::get_file_to_write(&resource_path) {
            Ok(f) => f,
            Err(e) => panic!("Error: Cannot open file {} to write, {:?}", resource_path, e)
        };
        server_url.push_str("/");
        server_url.push_str(&resource_path);
        let mut buf: Vec<u8> = vec![];
        match self.client.get(&*server_url).send() {
            Ok(mut resp) => {
                if resp.status().is_success() {
                    resp.copy_to(&mut buf).unwrap();
                    file.write_all(&buf).unwrap();
                } else {
                    panic!("Error while downloading {}, status {:?}", server_url, resp.status());
                }
            }
            Err(e) => {
                println!("Error while downloading {}, status {:?}", server_url, e.status());
            }
        }
    }
}

