use toml;
// use serde;
use serde::Deserialize;
use std::env;
use std::fs;
use std::io::Read;
// use std::io::{self, Write};
// use std::path;

#[derive(Debug, Deserialize)]
struct ConfigFile {
    client: Client,
}

#[derive(Debug, Deserialize)]
struct Client {
    port: String,
    baud: u32,
}

#[derive(Default)]
pub struct Config {
    port: String,
    baud: u32,
}

impl Config {
    fn fetch_file(&self, file_path: String) -> Result<String, bool> {
        let mut data = String::new();
        let mut file = fs::File::open(file_path).expect("Failed to open file");
        file.read_to_string(&mut data).expect("Failed to read file");
        return Ok(data);
    }

    pub fn init_config(&mut self) -> Result<(), String> {
        let curr_os = (env::consts::OS).to_owned();

        let mut _conf_data = String::new();

        if curr_os == "windows" {
            _conf_data = self
                .fetch_file("C:/ProgramData/Netra/config.toml".to_owned())
                .unwrap();
        } else if curr_os == "linux" {
            _conf_data = self
                .fetch_file("/etc/netra/config.toml".to_owned())
                .unwrap();
        } else {
            return Err("OS not supported".to_owned());
        }

        let config: ConfigFile = toml::from_str(&_conf_data).unwrap();

        self.port = config.client.port;
        self.baud = config.client.baud;

        return Ok(());
    }

    pub fn get_port(&self) -> String {
        return self.port.to_owned();
    }

    pub fn get_baud(&self) -> u32 {
        return self.baud;
    }
}
