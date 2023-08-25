use toml::{self, to_string};
// use serde;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::io::{Read, Write};

#[derive(Debug, Deserialize, Serialize)]
struct ConfigFile {
    client: Client,
}

#[derive(Debug, Deserialize, Serialize)]
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
    fn check_os(&self) -> Result<String, String> {
        let curr_os = (env::consts::OS).to_owned();
        let file_name = String::from("config.toml");

        if curr_os == "windows"{
            let mut path = "C:\\ProgramData\\Netra\\".to_owned();
            path.push_str(file_name.as_str());
            return Ok(path);
        }
        if curr_os == "linux" {
            let mut path = "/etc/netra/".to_owned();
            path.push_str(file_name.as_str());
            return Ok(path);
        }
        return Err("OS not supported".to_owned());
    }

    fn read_file(&self, file_path: String) -> Result<String, String> {
        let mut data = String::new();
        let file = File::open(file_path);
        if file.is_ok() {
            file.unwrap()
                .read_to_string(&mut data)
                .expect("Failed to read file");
            return Ok(data);
        }
        return Err(file.unwrap_err().to_string());
    }

    pub fn write_file(&mut self, port: String, baud: u32) -> Result<(), String> {
        let conf_str = to_string(&ConfigFile {
            client: Client { port, baud },
        });
        let curr_os_path = self.check_os();
        if curr_os_path.is_err() {
            return Err(curr_os_path.unwrap_err());
        }
        let file_path = curr_os_path.unwrap();
        let file = File::create(file_path.to_owned());
        if file.is_err() || conf_str.is_err() {
            return Err("Write failed error".to_owned());
        }
        let write_res = file.unwrap().write_all(conf_str.unwrap().as_bytes());
        if write_res.is_err() {
            return Err("Write failed error".to_owned());
        }
        return Ok(());
    }

    pub fn set_port(&mut self, port: String) -> Result<(), String> {
        let write_status = self.write_file(port, self.baud);
        if write_status.is_err() {
            return Err(write_status.unwrap_err());
        }
        return Ok(());
    }

    pub fn set_baud(&mut self, baud: u32) -> Result<(), String> {
        let write_status = self.write_file(self.port.to_owned(), baud);
        if write_status.is_err() {
            return Err(write_status.unwrap_err());
        }
        return Ok(());
    }

    pub fn init_config(&mut self) -> Result<(), String> {
        self.port = String::new();
        self.baud = 19200;

        let write_status = self.write_file(self.port.to_owned(), self.baud);
        if write_status.is_err() {
            return Err(write_status.unwrap_err());
        }

        return Ok(());
    }

    pub fn read_config(&mut self) -> Result<(), String> {
        let check_os_path = self.check_os();
        if check_os_path.is_err() {
            return Err(check_os_path.unwrap_err());
        }

        let mut _conf_data = String::new();
        let file_path = check_os_path.unwrap();

        let fetch_data = self.read_file(file_path);
        if fetch_data.is_ok() {
            _conf_data = fetch_data.unwrap();
        } else {
            return Err(fetch_data.unwrap_err());
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
